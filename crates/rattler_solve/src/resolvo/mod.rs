//! Provides an solver implementation based on the [`resolvo`] crate.

use std::{
    cell::RefCell,
    cmp::Ordering,
    collections::{HashMap, HashSet},
    fmt::{Display, Formatter},
    marker::PhantomData,
    ops::Deref,
    rc::Rc,
};

use chrono::{DateTime, Utc};
use itertools::Itertools;
use rattler_conda_types::{
    package::ArchiveType, GenericVirtualPackage, MatchSpec, Matches, NamelessMatchSpec,
    PackageName, PackageRecord, ParseMatchSpecError, ParseStrictness, RepoDataRecord,
};
use resolvo::{
    Candidates, Dependencies, DependencyProvider, KnownDependencies, NameId, Pool, SolvableDisplay,
    SolvableId, Solver as LibSolvRsSolver, SolverCache, UnsolvableOrCancelled, VersionSet,
    VersionSetId,
};

use crate::{
    resolvo::conda_util::CompareStrategy, ChannelPriority, IntoRepoData, SolveError, SolveStrategy,
    SolverRepoData, SolverTask,
};

mod conda_util;

/// Represents the information required to load available packages into libsolv
/// for a single channel and platform combination
#[derive(Clone)]
pub struct RepoData<'a> {
    /// The actual records after parsing `repodata.json`
    pub records: Vec<&'a RepoDataRecord>,
}

impl<'a> FromIterator<&'a RepoDataRecord> for RepoData<'a> {
    fn from_iter<T: IntoIterator<Item = &'a RepoDataRecord>>(iter: T) -> Self {
        Self {
            records: Vec::from_iter(iter),
        }
    }
}

impl<'a> SolverRepoData<'a> for RepoData<'a> {}

/// Wrapper around `MatchSpec` so that we can use it in the `resolvo` pool
#[repr(transparent)]
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct SolverMatchSpec<'a> {
    inner: NamelessMatchSpec,
    _marker: PhantomData<&'a PackageRecord>,
}

impl<'a> From<NamelessMatchSpec> for SolverMatchSpec<'a> {
    fn from(value: NamelessMatchSpec) -> Self {
        Self {
            inner: value,
            _marker: PhantomData,
        }
    }
}

impl<'a> Display for SolverMatchSpec<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl<'a> Deref for SolverMatchSpec<'a> {
    type Target = NamelessMatchSpec;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'a> VersionSet for SolverMatchSpec<'a> {
    type V = SolverPackageRecord<'a>;

    fn contains(&self, v: &Self::V) -> bool {
        match v {
            SolverPackageRecord::Record(rec) => self.inner.matches(*rec),
            SolverPackageRecord::VirtualPackage(GenericVirtualPackage {
                version,
                build_string,
                ..
            }) => {
                if let Some(spec) = self.inner.version.as_ref() {
                    if !spec.matches(version) {
                        return false;
                    }
                }

                if let Some(build_match) = self.inner.build.as_ref() {
                    if !build_match.matches(build_string) {
                        return false;
                    }
                }

                true
            }
        }
    }
}

/// Wrapper around [`PackageRecord`] so that we can use it in resolvo pool
#[derive(Eq, PartialEq)]
enum SolverPackageRecord<'a> {
    Record(&'a RepoDataRecord),
    VirtualPackage(&'a GenericVirtualPackage),
}

impl<'a> PartialOrd<Self> for SolverPackageRecord<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for SolverPackageRecord<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name()
            .cmp(other.name())
            .then_with(|| self.version().cmp(other.version()))
            .then_with(|| self.build_number().cmp(&other.build_number()))
            .then_with(|| self.timestamp().cmp(&other.timestamp()))
    }
}

impl<'a> SolverPackageRecord<'a> {
    fn name(&self) -> &PackageName {
        match self {
            SolverPackageRecord::Record(rec) => &rec.package_record.name,
            SolverPackageRecord::VirtualPackage(rec) => &rec.name,
        }
    }

    fn version(&self) -> &rattler_conda_types::Version {
        match self {
            SolverPackageRecord::Record(rec) => rec.package_record.version.version(),
            SolverPackageRecord::VirtualPackage(rec) => &rec.version,
        }
    }

    fn track_features(&self) -> &[String] {
        const EMPTY: [String; 0] = [];
        match self {
            SolverPackageRecord::Record(rec) => &rec.package_record.track_features,
            SolverPackageRecord::VirtualPackage(_rec) => &EMPTY,
        }
    }

    fn build_number(&self) -> u64 {
        match self {
            SolverPackageRecord::Record(rec) => rec.package_record.build_number,
            SolverPackageRecord::VirtualPackage(_rec) => 0,
        }
    }

    fn timestamp(&self) -> Option<&chrono::DateTime<chrono::Utc>> {
        match self {
            SolverPackageRecord::Record(rec) => rec.package_record.timestamp.as_ref(),
            SolverPackageRecord::VirtualPackage(_rec) => None,
        }
    }
}

impl<'a> Display for SolverPackageRecord<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SolverPackageRecord::Record(rec) => {
                write!(f, "{}", &rec.package_record)
            }
            SolverPackageRecord::VirtualPackage(rec) => {
                write!(f, "{rec}")
            }
        }
    }
}

/// Dependency provider for conda
#[derive(Default)]
pub(crate) struct CondaDependencyProvider<'a> {
    pool: Rc<Pool<SolverMatchSpec<'a>, String>>,

    records: HashMap<NameId, Candidates>,

    matchspec_to_highest_version:
        RefCell<HashMap<VersionSetId, Option<(rattler_conda_types::Version, bool)>>>,

    parse_match_spec_cache: RefCell<HashMap<&'a str, VersionSetId>>,

    stop_time: Option<std::time::SystemTime>,

    strategy: SolveStrategy,

    direct_dependencies: HashSet<NameId>,
}

impl<'a> CondaDependencyProvider<'a> {
    #[allow(clippy::too_many_arguments)]
    pub fn from_solver_task(
        repodata: impl IntoIterator<Item = RepoData<'a>>,
        favored_records: &'a [RepoDataRecord],
        locked_records: &'a [RepoDataRecord],
        virtual_packages: &'a [GenericVirtualPackage],
        match_specs: &[MatchSpec],
        stop_time: Option<std::time::SystemTime>,
        channel_priority: ChannelPriority,
        exclude_newer: Option<DateTime<Utc>>,
        strategy: SolveStrategy,
    ) -> Result<Self, SolveError> {
        let pool = Rc::new(Pool::default());
        let mut records: HashMap<NameId, Candidates> = HashMap::default();

        // Add virtual packages to the records
        for virtual_package in virtual_packages {
            let name = pool.intern_package_name(virtual_package.name.as_normalized());
            let solvable =
                pool.intern_solvable(name, SolverPackageRecord::VirtualPackage(virtual_package));
            records.entry(name).or_default().candidates.push(solvable);
        }

        // Compute the direct dependencies
        let direct_dependencies = match_specs
            .iter()
            .filter_map(|spec| spec.name.as_ref())
            .map(|name| pool.intern_package_name(name.as_normalized()))
            .collect();

        // TODO: Normalize these channel names to urls so we can compare them correctly.
        let channel_specific_specs = match_specs
            .iter()
            .filter(|spec| spec.channel.is_some())
            .collect::<Vec<_>>();

        // Hashmap that maps the package name to the channel it was first found in.
        let mut package_name_found_in_channel = HashMap::<String, &String>::new();

        // Add additional records
        for repo_datas in repodata {
            // Iterate over all records and dedup records that refer to the same package
            // data but with different archive types. This can happen if you
            // have two variants of the same package but with different
            // extensions. We prefer `.conda` packages over `.tar.bz`.
            //
            // Its important to insert the records in the same order as how they were
            // presented to this function to ensure that each solve is
            // deterministic. Iterating over HashMaps is not deterministic at
            // runtime so instead we store the values in a Vec as we iterate over the
            // records. This guarentees that the order of records remains the same over
            // runs.
            let mut ordered_repodata = Vec::with_capacity(repo_datas.records.len());
            let mut package_to_type: HashMap<&str, (ArchiveType, usize, bool)> =
                HashMap::with_capacity(repo_datas.records.len());

            for record in repo_datas.records {
                // Determine if this record will be excluded.
                let excluded = matches!((&exclude_newer, &record.package_record.timestamp),
                    (Some(exclude_newer), Some(record_timestamp))
                        if record_timestamp > exclude_newer);

                let (file_name, archive_type) = ArchiveType::split_str(&record.file_name)
                    .unwrap_or((&record.file_name, ArchiveType::TarBz2));
                match package_to_type.get_mut(file_name) {
                    None => {
                        let idx = ordered_repodata.len();
                        ordered_repodata.push(record);
                        package_to_type.insert(file_name, (archive_type, idx, excluded));
                    }
                    Some((prev_archive_type, idx, previous_excluded)) => {
                        if *previous_excluded && !excluded {
                            // The previous package would have been excluded by the solver. If the
                            // current record won't be excluded we should always use that.
                            *prev_archive_type = archive_type;
                            ordered_repodata[*idx] = record;
                            *previous_excluded = false;
                        } else if excluded && !*previous_excluded {
                            // The previous package would not have been excluded
                            // by the solver but
                            // this one will, so we'll keep the previous one
                            // regardless of the type.
                        } else {
                            match archive_type.cmp(prev_archive_type) {
                                Ordering::Greater => {
                                    // A previous package has a worse package "type", we'll use the
                                    // current record instead.
                                    *prev_archive_type = archive_type;
                                    ordered_repodata[*idx] = record;
                                    *previous_excluded = excluded;
                                }
                                Ordering::Less => {
                                    // A previous package that we already stored
                                    // is actually a package of a better
                                    // "type" so we'll just use that instead
                                    // (.conda > .tar.bz)
                                }
                                Ordering::Equal => {
                                    return Err(SolveError::DuplicateRecords(
                                        record.file_name.clone(),
                                    ));
                                }
                            }
                        }
                    }
                }
            }

            for record in ordered_repodata {
                let package_name =
                    pool.intern_package_name(record.package_record.name.as_normalized());
                let solvable_id =
                    pool.intern_solvable(package_name, SolverPackageRecord::Record(record));
                let candidates = records.entry(package_name).or_default();
                candidates.candidates.push(solvable_id);

                // Filter out any records that are newer than a specific date.
                match (&exclude_newer, &record.package_record.timestamp) {
                    (Some(exclude_newer), Some(record_timestamp))
                        if record_timestamp > exclude_newer =>
                    {
                        let reason = pool.intern_string(format!(
                            "the package is uploaded after the cutoff date of {exclude_newer}"
                        ));
                        candidates.excluded.push((solvable_id, reason));
                    }
                    _ => {}
                }

                // Add to excluded when package is not in the specified channel.
                if !channel_specific_specs.is_empty() {
                    if let Some(spec) = channel_specific_specs.iter().find(|&&spec| {
                        spec.name
                            .as_ref()
                            .expect("expecting a name")
                            .as_normalized()
                            == record.package_record.name.as_normalized()
                    }) {
                        // Check if the spec has a channel, and compare it to the repodata channel
                        if let Some(spec_channel) = &spec.channel {
                            if record.channel != spec_channel.base_url.to_string() {
                                tracing::debug!("Ignoring {} from {} because it was not requested from that channel.", &record.package_record.name.as_normalized(), &record.channel);
                                // Add record to the excluded with reason of being in the non
                                // requested channel.
                                let message = format!(
                                    "candidate not in requested channel: '{}'",
                                    spec_channel
                                        .name
                                        .clone()
                                        .unwrap_or(spec_channel.base_url.to_string())
                                );
                                candidates
                                    .excluded
                                    .push((solvable_id, pool.intern_string(message)));
                                continue;
                            }
                        }
                    }
                }

                // Enforce channel priority
                // This function makes the assumption that the records are given in order of the
                // channels.
                if let (Some(first_channel), ChannelPriority::Strict) = (
                    package_name_found_in_channel
                        .get(&record.package_record.name.as_normalized().to_string()),
                    channel_priority,
                ) {
                    // Add the record to the excluded list when it is from a different channel.
                    if first_channel != &&record.channel {
                        tracing::debug!(
                            "Ignoring '{}' from '{}' because of strict channel priority.",
                            &record.package_record.name.as_normalized(),
                            &record.channel
                        );
                        candidates.excluded.push((
                            solvable_id,
                            pool.intern_string(format!(
                                "due to strict channel priority not using this option from: '{}'",
                                &record.channel
                            )),
                        ));
                        continue;
                    }
                } else {
                    package_name_found_in_channel.insert(
                        record.package_record.name.as_normalized().to_string(),
                        &record.channel,
                    );
                }

                candidates.hint_dependencies_available.push(solvable_id);
            }
        }

        // Add favored packages to the records
        for favored_record in favored_records {
            let name = pool.intern_package_name(favored_record.package_record.name.as_normalized());
            let solvable = pool.intern_solvable(name, SolverPackageRecord::Record(favored_record));
            let candidates = records.entry(name).or_default();
            candidates.candidates.push(solvable);
            candidates.favored = Some(solvable);
        }

        for locked_record in locked_records {
            let name = pool.intern_package_name(locked_record.package_record.name.as_normalized());
            let solvable = pool.intern_solvable(name, SolverPackageRecord::Record(locked_record));
            let candidates = records.entry(name).or_default();
            candidates.candidates.push(solvable);
            candidates.locked = Some(solvable);
        }

        Ok(Self {
            pool,
            records,
            matchspec_to_highest_version: RefCell::default(),
            parse_match_spec_cache: RefCell::default(),
            stop_time,
            strategy,
            direct_dependencies,
        })
    }
}

/// The reason why the solver was cancelled
pub enum CancelReason {
    /// The solver was cancelled because the timeout was reached
    Timeout,
}

impl<'a> DependencyProvider<SolverMatchSpec<'a>> for CondaDependencyProvider<'a> {
    fn pool(&self) -> Rc<Pool<SolverMatchSpec<'a>, String>> {
        self.pool.clone()
    }

    async fn sort_candidates(
        &self,
        solver: &SolverCache<SolverMatchSpec<'a>, String, Self>,
        solvables: &mut [SolvableId],
    ) {
        if solvables.is_empty() {
            // Short circuit if there are no solvables to sort
            return;
        }

        let mut highest_version_spec = self.matchspec_to_highest_version.borrow_mut();

        let strategy = match self.strategy {
            SolveStrategy::Highest => CompareStrategy::Default,
            SolveStrategy::LowestVersion => CompareStrategy::LowestVersion,
            SolveStrategy::LowestVersionDirect => {
                if self
                    .direct_dependencies
                    .contains(&self.pool.resolve_solvable(solvables[0]).name_id())
                {
                    CompareStrategy::LowestVersion
                } else {
                    CompareStrategy::Default
                }
            }
        };
        solvables.sort_by(|&p1, &p2| {
            conda_util::compare_candidates(p1, p2, solver, &mut highest_version_spec, strategy)
        });
    }

    async fn get_candidates(&self, name: NameId) -> Option<Candidates> {
        self.records.get(&name).cloned()
    }

    async fn get_dependencies(&self, solvable: SolvableId) -> Dependencies {
        let mut dependencies = KnownDependencies::default();
        let SolverPackageRecord::Record(rec) = self.pool.resolve_solvable(solvable).inner() else {
            return Dependencies::Known(dependencies);
        };

        let mut parse_match_spec_cache = self.parse_match_spec_cache.borrow_mut();
        for depends in rec.package_record.depends.iter() {
            let version_set_id =
                parse_match_spec(&self.pool, depends, &mut parse_match_spec_cache).unwrap();
            dependencies.requirements.push(version_set_id);
        }

        for constrains in rec.package_record.constrains.iter() {
            let version_set_id =
                parse_match_spec(&self.pool, constrains, &mut parse_match_spec_cache).unwrap();
            dependencies.constrains.push(version_set_id);
        }

        Dependencies::Known(dependencies)
    }

    fn should_cancel_with_value(&self) -> Option<Box<dyn std::any::Any>> {
        if let Some(stop_time) = self.stop_time {
            if std::time::SystemTime::now() > stop_time {
                return Some(Box::new(CancelReason::Timeout));
            }
        }
        None
    }
}

/// Displays the different candidates by their version and sorted by their
/// version
pub struct CondaSolvableDisplay;

impl SolvableDisplay<SolverMatchSpec<'_>> for CondaSolvableDisplay {
    fn display_candidates(
        &self,
        pool: &Pool<SolverMatchSpec<'_>, String>,
        merged_candidates: &[SolvableId],
    ) -> String {
        merged_candidates
            .iter()
            .map(|&id| pool.resolve_solvable(id).inner().version())
            .sorted()
            .map(ToString::to_string)
            .join(" | ")
    }
}

/// A [`Solver`] implemented using the `resolvo` library
#[derive(Default)]
pub struct Solver;

impl super::SolverImpl for Solver {
    type RepoData<'a> = RepoData<'a>;

    #[allow(clippy::redundant_closure_for_method_calls)]
    fn solve<
        'a,
        R: IntoRepoData<'a, Self::RepoData<'a>>,
        TAvailablePackagesIterator: IntoIterator<Item = R>,
    >(
        &mut self,
        task: SolverTask<TAvailablePackagesIterator>,
    ) -> Result<Vec<RepoDataRecord>, SolveError> {
        let stop_time = task
            .timeout
            .map(|timeout| std::time::SystemTime::now() + timeout);

        // Construct a provider that can serve the data.
        let provider = CondaDependencyProvider::from_solver_task(
            task.available_packages.into_iter().map(|r| r.into()),
            &task.locked_packages,
            &task.pinned_packages,
            &task.virtual_packages,
            task.specs.clone().as_ref(),
            stop_time,
            task.channel_priority,
            task.exclude_newer,
            task.strategy,
        )?;
        let pool = provider.pool.clone();

        // Construct the requirements that the solver needs to satisfy.
        let root_requirements = task
            .specs
            .iter()
            .map(|spec| {
                let (name, nameless_spec) = spec.clone().into_nameless();
                let name = if let Some(name) = name {
                    name.as_normalized().to_string()
                } else if let Some(url) = &nameless_spec.url {
                    url.to_string()
                } else {
                    panic!("Cannot use matchspec without a name")
                };
                let name_id = provider.pool.intern_package_name(name.as_str());
                provider
                    .pool
                    .intern_version_set(name_id, nameless_spec.into())
            })
            .collect();

        let root_constraints = task
            .constraints
            .iter()
            .map(|spec| {
                let (name, spec) = spec.clone().into_nameless();
                let name = name.expect("cannot use matchspec without a name");
                let name_id = provider.pool.intern_package_name(name.as_normalized());
                provider.pool.intern_version_set(name_id, spec.into())
            })
            .collect();

        // Construct a solver and solve the problems in the queue
        let mut solver = LibSolvRsSolver::new(provider);
        let solvables = solver.solve(root_requirements, root_constraints).map_err(
            |unsolvable_or_cancelled| {
                match unsolvable_or_cancelled {
                    UnsolvableOrCancelled::Unsolvable(problem) => {
                        SolveError::Unsolvable(vec![problem
                            .display_user_friendly(&solver, pool, &CondaSolvableDisplay)
                            .to_string()])
                    }
                    // We are not doing this as of yet
                    // put a generic message in here for now
                    UnsolvableOrCancelled::Cancelled(_) => SolveError::Cancelled,
                }
            },
        )?;

        // Get the resulting packages from the solver.
        let required_records = solvables
            .into_iter()
            .filter_map(|id| match *solver.pool.resolve_solvable(id).inner() {
                SolverPackageRecord::Record(rec) => Some(rec.clone()),
                SolverPackageRecord::VirtualPackage(_) => None,
            })
            .collect();

        Ok(required_records)
    }
}

fn parse_match_spec<'a>(
    pool: &Pool<SolverMatchSpec<'a>>,
    spec_str: &'a str,
    parse_match_spec_cache: &mut HashMap<&'a str, VersionSetId>,
) -> Result<VersionSetId, ParseMatchSpecError> {
    if let Some(spec_id) = parse_match_spec_cache.get(spec_str) {
        Ok(*spec_id)
    } else {
        let match_spec = MatchSpec::from_str(spec_str, ParseStrictness::Lenient)?;
        let (name, spec) = match_spec.into_nameless();
        let dependency_name = pool.intern_package_name(
            name.as_ref()
                .expect("match specs without names are not supported")
                .as_normalized(),
        );
        let version_set_id = pool.intern_version_set(dependency_name, spec.into());
        parse_match_spec_cache.insert(spec_str, version_set_id);
        Ok(version_set_id)
    }
}
