use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
    str::FromStr,
};

use pep508_rs::Requirement;
use pyo3::{pyclass, pymethods, PyResult};
use rattler_lock::{
    Channel, Environment, LockFile, LockFileBuilder, Package, PackageHashes, PypiPackageData,
    PypiPackageEnvironmentData,
};

use crate::{error::PyRattlerError, platform::PyPlatform, record::PyRecord};

/// Represents a lock-file for both Conda packages and Pypi packages.
///
/// Lock-files can store information for multiple platforms and for multiple environments.
#[pyclass]
#[repr(transparent)]
#[derive(Clone)]
pub struct PyLockFile {
    pub(crate) inner: LockFile,
}

impl From<LockFile> for PyLockFile {
    fn from(value: LockFile) -> Self {
        Self { inner: value }
    }
}

impl From<PyLockFile> for LockFile {
    fn from(value: PyLockFile) -> Self {
        value.inner
    }
}

#[pymethods]
impl PyLockFile {
    #[new]
    pub fn new(
        channels: Vec<PyLockFileChannelConfig>,
        conda_packages: Vec<PyCondaPackageConfig>,
        pypi_packages: Vec<PyPypiPackageConfig>,
    ) -> PyResult<Self> {
        let mut lock_file = LockFileBuilder::new();

        for c in channels {
            lock_file.set_channels(c.env, c.channels);
        }

        for pkg in conda_packages {
            lock_file.add_conda_package(
                pkg.env,
                pkg.platform.into(),
                pkg.locked_package.try_as_repodata_record()?.clone().into(),
            );
        }

        for pkg in pypi_packages {
            lock_file.add_pypi_package(
                pkg.env,
                pkg.platform.inner,
                pkg.locked_package.inner,
                pkg.env_data.inner,
            );
        }

        Ok(lock_file.finish().into())
    }

    /// Writes the conda lock to a file
    pub fn to_path(&self, path: PathBuf) -> PyResult<()> {
        Ok(self.inner.to_path(&path).map_err(PyRattlerError::from)?)
    }

    /// Parses an rattler-lock file from a file.
    #[staticmethod]
    pub fn from_path(path: PathBuf) -> PyResult<Self> {
        Ok(LockFile::from_path(&path)
            .map(Into::into)
            .map_err(PyRattlerError::from)?)
    }

    /// Returns the environment with the given name.
    pub fn environment(&self, name: &str) -> Option<PyEnvironment> {
        self.inner.environment(name).map(Into::into)
    }

    /// Returns the environment with the default name as defined by [`DEFAULT_ENVIRONMENT_NAME`].
    pub fn default_environment(&self) -> Option<PyEnvironment> {
        self.inner.default_environment().map(Into::into)
    }

    /// Returns an iterator over all environments defined in the lock-file.
    pub fn environments(&self) -> Vec<(String, PyEnvironment)> {
        self.inner
            .environments()
            .map(|(name, env)| (name.to_owned(), env.into()))
            .collect()
    }
}

#[pyclass]
#[derive(Clone)]
pub struct PyLockFileChannelConfig {
    #[pyo3(get, set)]
    env: String,
    #[pyo3(get, set)]
    channels: Vec<PyLockChannel>,
}

#[pymethods]
impl PyLockFileChannelConfig {
    #[new]
    pub fn new(env: String, channels: Vec<PyLockChannel>) -> Self {
        Self { env, channels }
    }
}

#[pyclass]
#[derive(Clone)]
pub struct PyCondaPackageConfig {
    #[pyo3(get, set)]
    env: String,
    #[pyo3(get, set)]
    platform: PyPlatform,
    #[pyo3(get, set)]
    locked_package: PyRecord,
}

#[pymethods]
impl PyCondaPackageConfig {
    #[new]
    pub fn new(env: String, platform: PyPlatform, locked_package: PyRecord) -> Self {
        Self {
            env,
            platform,
            locked_package,
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub struct PyPypiPackageConfig {
    #[pyo3(get, set)]
    env: String,
    #[pyo3(get, set)]
    platform: PyPlatform,
    #[pyo3(get, set)]
    locked_package: PyPypiPackageData,
    #[pyo3(get, set)]
    env_data: PyPypiPackageEnvironmentData,
}

#[pymethods]
impl PyPypiPackageConfig {
    #[new]
    pub fn new(
        env: String,
        platform: PyPlatform,
        locked_package: PyPypiPackageData,
        env_data: PyPypiPackageEnvironmentData,
    ) -> Self {
        Self {
            env,
            platform,
            locked_package,
            env_data,
        }
    }
}

#[pyclass]
#[repr(transparent)]
#[derive(Clone)]
pub struct PyEnvironment {
    pub(crate) inner: Environment,
}

impl From<Environment> for PyEnvironment {
    fn from(value: Environment) -> Self {
        Self { inner: value }
    }
}

impl From<PyEnvironment> for Environment {
    fn from(value: PyEnvironment) -> Self {
        value.inner
    }
}

#[pymethods]
impl PyEnvironment {
    /// Returns all the platforms for which we have a locked-down environment.
    pub fn platforms(&self) -> Vec<PyPlatform> {
        self.inner.platforms().map(Into::into).collect::<Vec<_>>()
    }

    /// Returns the channels that are used by this environment.
    ///
    /// Note that the order of the channels is significant. The first channel is the highest
    /// priority channel.
    pub fn channels(&self) -> Vec<PyLockChannel> {
        self.inner
            .channels()
            .iter()
            .map(|p| p.clone().into())
            .collect()
    }

    /// Returns all the packages for a specific platform in this environment.
    pub fn packages(&self, platform: PyPlatform) -> Option<Vec<PyLockPackage>> {
        if let Some(packages) = self.inner.packages(platform.inner) {
            return Some(packages.map(std::convert::Into::into).collect());
        }
        None
    }

    /// Returns a list of all packages and platforms defined for this environment
    pub fn packages_by_platform(&self) -> Vec<(PyPlatform, Vec<PyLockPackage>)> {
        self.inner
            .packages_by_platform()
            .map(|(platform, pkgs)| (platform.into(), pkgs.map(Into::into).collect::<Vec<_>>()))
            .collect()
    }

    /// Returns all pypi packages for all platforms
    pub fn pypi_packages(
        &self,
    ) -> HashMap<PyPlatform, Vec<(PyPypiPackageData, PyPypiPackageEnvironmentData)>> {
        self.inner
            .pypi_packages()
            .into_iter()
            .map(|(platform, data_vec)| {
                let data = data_vec
                    .into_iter()
                    .map(|(pkg_data, pkg_env_data)| (pkg_data.into(), pkg_env_data.into()))
                    .collect::<Vec<(PyPypiPackageData, PyPypiPackageEnvironmentData)>>();
                (platform.into(), data)
            })
            .collect()
    }

    /// Returns all conda packages for all platforms and converts them to [`PyRecord`].
    pub fn conda_repodata_records(&self) -> PyResult<HashMap<PyPlatform, Vec<PyRecord>>> {
        Ok(self
            .inner
            .conda_repodata_records()
            .map_err(PyRattlerError::from)?
            .into_iter()
            .map(|(platform, record_vec)| {
                (
                    platform.into(),
                    record_vec.into_iter().map(Into::into).collect(),
                )
            })
            .collect())
    }

    /// Takes all the conda packages, converts them to [`PyRecord`] and returns them or
    /// returns an error if the conversion failed. Returns `None` if the specified platform is not
    /// defined for this environment.
    pub fn conda_repodata_records_for_platform(
        &self,
        platform: PyPlatform,
    ) -> PyResult<Option<Vec<PyRecord>>> {
        if let Some(records) = self
            .inner
            .conda_repodata_records_for_platform(platform.inner)
            .map_err(PyRattlerError::from)?
        {
            return Ok(Some(records.into_iter().map(Into::into).collect()));
        }
        Ok(None)
    }

    /// Returns all the pypi packages and their associated environment data for the specified
    /// platform. Returns `None` if the platform is not defined for this environment.
    pub fn pypi_packages_for_platform(
        &self,
        platform: PyPlatform,
    ) -> Option<Vec<(PyPypiPackageData, PyPypiPackageEnvironmentData)>> {
        if let Some(data) = self.inner.pypi_packages_for_platform(platform.inner) {
            return Some(
                data.into_iter()
                    .map(|(pkg_data, env_data)| (pkg_data.into(), env_data.into()))
                    .collect(),
            );
        }
        None
    }
}

#[pyclass]
#[repr(transparent)]
#[derive(Clone)]
pub struct PyLockChannel {
    pub(crate) inner: Channel,
}

impl From<Channel> for PyLockChannel {
    fn from(value: Channel) -> Self {
        Self { inner: value }
    }
}

impl From<rattler_conda_types::Channel> for PyLockChannel {
    fn from(value: rattler_conda_types::Channel) -> Self {
        Self {
            inner: Channel::from(value.base_url().to_string()),
        }
    }
}

impl From<PyLockChannel> for Channel {
    fn from(value: PyLockChannel) -> Self {
        value.inner
    }
}

#[pymethods]
impl PyLockChannel {
    #[new]
    pub fn new(url: String) -> Self {
        Self {
            inner: Channel::from(url),
        }
    }

    pub fn as_str(&self) -> String {
        self.inner.url.clone()
    }
}

#[pyclass]
#[repr(transparent)]
#[derive(Clone)]
pub struct PyLockPackage {
    pub(crate) inner: Package,
}

impl From<Package> for PyLockPackage {
    fn from(value: Package) -> Self {
        Self { inner: value }
    }
}

impl From<PyLockPackage> for Package {
    fn from(value: PyLockPackage) -> Self {
        value.inner
    }
}

#[pyclass]
#[repr(transparent)]
#[derive(Clone)]
pub struct PyPypiPackageData {
    pub(crate) inner: PypiPackageData,
}

impl From<PypiPackageData> for PyPypiPackageData {
    fn from(value: PypiPackageData) -> Self {
        Self { inner: value }
    }
}

impl From<PyPypiPackageData> for PypiPackageData {
    fn from(value: PyPypiPackageData) -> Self {
        value.inner
    }
}

#[pymethods]
impl PyPypiPackageData {
    /// Returns true if this package satisfies the given `spec`.
    pub fn satisfies(&self, spec: String) -> PyResult<bool> {
        Ok(self.inner.satisfies(
            &Requirement::from_str(&spec)
                .map_err(|e| PyRattlerError::RequirementError(e.to_string()))?,
        ))
    }

    /// The name of the package.
    #[getter]
    pub fn name(&self) -> String {
        self.inner.name.clone()
    }

    /// The version of the package.
    #[getter]
    pub fn version(&self) -> String {
        self.inner.version.clone().to_string()
    }

    /// The URL that points to where the artifact can be downloaded from.
    #[getter]
    pub fn url(&self) -> String {
        self.inner.url.to_string()
    }

    /// Hashes of the file pointed to by `url`.
    #[getter]
    pub fn hash(&self) -> Option<PyPackageHashes> {
        if let Some(hash) = self.inner.hash.clone() {
            return Some(hash.into());
        }
        None
    }

    /// A list of dependencies on other packages.
    #[getter]
    pub fn requires_dist(&self) -> Vec<String> {
        self.inner
            .requires_dist
            .clone()
            .into_iter()
            .map(|req| req.to_string())
            .collect()
    }

    /// The python version that this package requires.
    #[getter]
    pub fn requires_python(&self) -> Option<String> {
        if let Some(specifier) = self.inner.requires_python.clone() {
            return Some(specifier.to_string());
        }
        None
    }
}

#[pyclass]
#[repr(transparent)]
#[derive(Clone)]
pub struct PyPypiPackageEnvironmentData {
    pub(crate) inner: PypiPackageEnvironmentData,
}

impl From<PypiPackageEnvironmentData> for PyPypiPackageEnvironmentData {
    fn from(value: PypiPackageEnvironmentData) -> Self {
        Self { inner: value }
    }
}

impl From<PyPypiPackageEnvironmentData> for PypiPackageEnvironmentData {
    fn from(value: PyPypiPackageEnvironmentData) -> Self {
        value.inner
    }
}

#[pymethods]
impl PyPypiPackageEnvironmentData {
    /// The extras enabled for the package. Note that the order doesn't matter.
    #[getter]
    pub fn extras(&self) -> HashSet<String> {
        self.inner.extras.clone()
    }
}

#[pyclass]
#[repr(transparent)]
#[derive(Clone)]
pub struct PyPackageHashes {
    pub(crate) inner: PackageHashes,
}

impl From<PackageHashes> for PyPackageHashes {
    fn from(value: PackageHashes) -> Self {
        Self { inner: value }
    }
}

impl From<PyPackageHashes> for PackageHashes {
    fn from(value: PyPackageHashes) -> Self {
        value.inner
    }
}
