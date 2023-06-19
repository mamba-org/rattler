//! Builder for the creation of lock files. Currently,
//!
use crate::conda_lock::content_hash::CalculateContentHashError;
use crate::conda_lock::{
    content_hash, Channel, CondaLock, GitMeta, LockMeta, LockedDependency, Manager, PackageHashes,
    TimeMeta, VersionConstraint,
};
use crate::{MatchSpec, NoArchType, Platform};
use fxhash::{FxHashMap, FxHashSet};
use url::Url;

/// Struct used to build a conda-lock file
#[derive(Default)]
pub struct LockFileBuilder {
    /// Channels used to resolve dependencies
    pub channels: Vec<Channel>,
    /// The platforms this lock file supports
    pub platforms: FxHashSet<Platform>,
    /// Paths to source files, relative to the parent directory of the lockfile
    pub sources: Option<Vec<String>>,
    /// Metadata dealing with the time lockfile was created
    pub time_metadata: Option<TimeMeta>,
    /// Metadata dealing with the git repo the lockfile was created in and the user that created it
    pub git_metadata: Option<GitMeta>,

    /// Keep track of locked packages per platform
    pub locked_packages: FxHashMap<Platform, LockedPackages>,

    /// MatchSpecs input
    /// This is only used to calculate the content_hash
    /// for the lock file
    pub input_specs: Vec<MatchSpec>,
}

impl LockFileBuilder {
    /// Generate a new lock file using the builder pattern
    /// channels, platforms and input_specs need to be provided
    pub fn new(
        channels: impl IntoIterator<Item = impl Into<Channel>>,
        platforms: impl IntoIterator<Item = Platform>,
        input_spec: impl IntoIterator<Item = MatchSpec>,
    ) -> Self {
        Self {
            channels: channels
                .into_iter()
                .map(|into_channel| into_channel.into())
                .collect(),
            platforms: platforms.into_iter().collect(),
            input_specs: input_spec.into_iter().collect(),
            ..Default::default()
        }
    }

    /// Add locked packages per platform
    pub fn add_locked_packages(mut self, locked_packages: LockedPackages) -> Self {
        let platform = &locked_packages.platform;
        if self.locked_packages.contains_key(platform) {
            panic!("Tried to insert packages for {platform} twice")
        }

        self.locked_packages
            .insert(locked_packages.platform, locked_packages);
        self
    }

    /// Build a conda_lock file
    pub fn build(self) -> Result<CondaLock, CalculateContentHashError> {
        let content_hash = self
            .platforms
            .iter()
            .map(|plat| {
                Ok((
                    *plat,
                    content_hash::calculate_content_hash(plat, &self.input_specs, &self.channels)?,
                ))
            })
            .collect::<Result<FxHashMap<_, _>, CalculateContentHashError>>()?;

        let lock = CondaLock {
            metadata: LockMeta {
                content_hash,
                channels: self.channels,
                platforms: self.platforms.iter().cloned().collect(),
                sources: self.sources.unwrap_or_default(),
                time_metadata: self.time_metadata,
                git_metadata: self.git_metadata,
                inputs_metadata: None,
                custom_metadata: None,
            },
            package: self
                .locked_packages
                .into_values()
                .flat_map(|package| package.build())
                .collect(),
            version: super::default_version(),
        };
        Ok(lock)
    }
}

/// Shorthand for creating packages per platform
pub struct LockedPackages {
    /// The number of locked packages
    pub locked_packages: Vec<LockedPackage>,
    /// The to lock the packages to
    pub platform: Platform,
}

impl LockedPackages {
    /// Create a list of locked packages per platform
    pub fn new(platform: Platform) -> Self {
        Self {
            locked_packages: Vec::new(),
            platform,
        }
    }

    /// Add a locked package
    pub fn add_locked_package(mut self, locked_package: LockedPackage) -> Self {
        self.locked_packages.push(locked_package);
        self
    }

    /// Transform into list of [`LockedDependency`] objects
    pub fn build(self) -> Vec<LockedDependency> {
        self.locked_packages
            .into_iter()
            .map(|locked_package| {
                LockedDependency {
                    name: locked_package.name,
                    version: locked_package.version,
                    /// Use conda as default manager for now
                    manager: Manager::Conda,
                    platform: self.platform,
                    dependencies: locked_package.dependency_list,
                    url: locked_package.url,
                    hash: locked_package.package_hashes,
                    optional: locked_package.optional.unwrap_or(false),
                    category: super::default_category(),
                    source: None,
                    build: Some(locked_package.build_string),
                    arch: locked_package.arch,
                    subdir: locked_package.subdir,
                    build_number: locked_package.build_number,
                    constrains: if locked_package.constrains.is_empty() {
                        None
                    } else {
                        Some(locked_package.constrains)
                    },
                    features: locked_package.features,
                    track_features: if locked_package.track_features.is_empty() {
                        None
                    } else {
                        Some(locked_package.track_features)
                    },
                    license: locked_package.license,
                    license_family: locked_package.license_family,
                    noarch: locked_package.noarch,
                    size: locked_package.size,
                    timestamp: locked_package.timestamp,
                }
            })
            .collect()
    }
}

/// Short-hand for creating a LockedPackage that transforms into a [`LockedDependency`]
pub struct LockedPackage {
    /// Name of the locked package
    pub name: String,
    /// Package version
    pub version: String,
    /// Package build string
    pub build_string: String,
    /// Url where the package is hosted
    pub url: Url,
    /// Collection of package hash fields
    pub package_hashes: PackageHashes,
    /// List of dependencies for this package
    pub dependency_list: FxHashMap<String, VersionConstraint>,
    /// Check if package is optional
    pub optional: Option<bool>,

    /// Experimental: architecture field
    pub arch: Option<String>,

    /// Experimental: the subdir where the package can be found
    pub subdir: Option<String>,

    /// Experimental: conda build number of the package
    pub build_number: Option<u64>,

    /// Experimental: see: [Constrains](crate::repo_data::PackageRecord::constrains)
    pub constrains: Vec<String>,

    /// Experimental: see: [Features](crate::repo_data::PackageRecord::features)
    pub features: Option<String>,

    /// Experimental: see: [Track features](crate::repo_data::PackageRecord::track_features)
    pub track_features: Vec<String>,

    /// Experimental: the specific license of the package
    pub license: Option<String>,

    /// Experimental: the license family of the package
    pub license_family: Option<String>,

    /// Experimental: If this package is independent of architecture this field specifies in what way. See
    /// [`NoArchType`] for more information.
    pub noarch: NoArchType,

    /// Experimental: The size of the package archive in bytes
    pub size: Option<u64>,

    /// Experimental: The date this entry was created.
    pub timestamp: Option<chrono::DateTime<chrono::Utc>>,
}

impl LockedPackage {
    /// Set if the package should be optional
    pub fn set_optional(mut self, optional: bool) -> Self {
        self.optional = Some(optional);
        self
    }

    /// Add a single dependency
    pub fn add_dependency<S: AsRef<str>>(
        mut self,
        key: S,
        version_constraint: VersionConstraint,
    ) -> Self {
        self.dependency_list
            .insert(key.as_ref().to_string(), version_constraint);
        self
    }

    /// Add multiple dependencies
    pub fn add_dependencies(
        mut self,
        value: impl IntoIterator<Item = (String, VersionConstraint)>,
    ) -> Self {
        self.dependency_list.extend(value);
        self
    }

    /// Set the subdir for for the package
    pub fn set_arch<S: AsRef<str>>(mut self, arch: String) -> Self {
        self.subdir = Some(arch);
        self
    }

    /// Set the subdir for for the package
    pub fn set_subdir<S: AsRef<str>>(mut self, subdir: String) -> Self {
        self.subdir = Some(subdir);
        self
    }

    /// Set the subdir for for the package
    pub fn set_build_number<S: AsRef<str>>(mut self, build_number: u64) -> Self {
        self.build_number = Some(build_number);
        self
    }

    /// Add the constrains for this package
    pub fn add_constrain<S: AsRef<str>>(mut self, constrain: S) -> Self {
        self.constrains.push(constrain.as_ref().to_string());
        self
    }

    /// Add the constrains for this package
    pub fn add_constrains<S: AsRef<str>>(
        mut self,
        constrain: impl IntoIterator<Item = String>,
    ) -> Self {
        self.constrains.extend(constrain);
        self
    }

    /// Set the features for for the package
    pub fn set_features<S: AsRef<str>>(mut self, features: S) -> Self {
        self.features = Some(features.as_ref().to_string());
        self
    }

    /// Add a track feature for the package
    pub fn add_track_feature<S: AsRef<str>>(mut self, track_feature: S) -> Self {
        self.track_features.push(track_feature.as_ref().to_string());
        self
    }

    /// Add multiple track features for for the package
    pub fn add_track_features(mut self, value: impl IntoIterator<Item = String>) -> Self {
        self.track_features.extend(value);
        self
    }

    /// Set the licence for for the package
    pub fn add_license<S: AsRef<str>>(mut self, license: S) -> Self {
        self.license = Some(license.as_ref().to_string());
        self
    }

    /// Set the license family for for the package
    pub fn add_license_family<S: AsRef<str>>(mut self, license_family: S) -> Self {
        self.license_family = Some(license_family.as_ref().to_string());
        self
    }

    /// Set the noarch type for for the package
    pub fn add_noarch(mut self, noarch_type: NoArchType) -> Self {
        self.noarch = noarch_type;
        self
    }

    /// Set the size of the package
    pub fn set_size(mut self, size: u64) -> Self {
        self.size = Some(size);
        self
    }

    /// Set the timestamp of the package
    pub fn set_timestamp(mut self, timestamp: chrono::DateTime<chrono::Utc>) -> Self {
        self.timestamp = Some(timestamp);
        self
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::conda_lock::builder::{LockFileBuilder, LockedPackage, LockedPackages};
    use crate::conda_lock::{CondaLock, PackageHashes};
    use crate::{ChannelConfig, MatchSpec, Platform};
    use rattler_digest::parse_digest_from_hex;

    #[test]
    fn create_lock_file() {
        let _channel_config = ChannelConfig::default();
        let lock = LockFileBuilder::new(
            ["conda_forge"],
            [Platform::Osx64],
            [MatchSpec::from_str("python =3.11.0").unwrap()]
        )
            .add_locked_packages(LockedPackages::new(Platform::Osx64)
                .add_locked_package(LockedPackage {
                    name: "python".to_string(),
                    version: "3.11.0".to_string(),
                    build_string: "h4150a38_1_cpython".to_string(),
                    url: "https://conda.anaconda.org/conda-forge/osx-64/python-3.11.0-h4150a38_1_cpython.conda".parse().unwrap(),
                    package_hashes:  PackageHashes::Md5Sha256(parse_digest_from_hex::<rattler_digest::Md5>("c6f4b87020c72e2700e3e94c1fc93b70").unwrap(),
                                                               parse_digest_from_hex::<rattler_digest::Sha256>("7c58de8c7d98b341bd9be117feec64782e704fec5c30f6e14713ebccaab9b5d8").unwrap()),
                    dependency_list: Default::default(),
                    optional: None,
                    arch: None,
                    subdir: None,
                    build_number: None,
                    constrains: vec![],
                    features: None,
                    track_features: vec![],
                    license: None,
                    license_family: None,
                    noarch: Default::default(),
                    size: None,
                    timestamp: None,
                }))
            .build().unwrap();

        // See if we can serialize/deserialize it
        let s = serde_yaml::to_string(&lock).unwrap();
        serde_yaml::from_str::<CondaLock>(&s).unwrap();
    }
}
