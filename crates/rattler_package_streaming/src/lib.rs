//! This crate provides the ability to extract a package archive or specific parts of it.

use std::path::Path;

#[cfg(feature = "sync")]
pub mod read;

/// An error that can occur when extracting a package archive.
#[derive(thiserror::Error, Debug)]
pub enum ExtractError {
    #[error("an io error occurred")]
    IoError(#[from] std::io::Error),

    #[error("could not create the destination path")]
    CouldNotCreateDestination(#[source] std::io::Error),

    #[error("invalid zip archive")]
    ZipError(#[from] zip::result::ZipError),
}

/// Describes the type of package archive. This can be derived from the file extension of a package.
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum ArchiveType {
    /// A file with the `.tar.bz2` extension.
    TarBz2,

    /// A file with the `.conda` extension.
    Conda,
}

impl ArchiveType {
    /// Tries to determine the type of a Conda archive from its filename.
    pub fn try_from(path: &Path) -> Option<ArchiveType> {
        if path.ends_with(".conda") {
            Some(ArchiveType::Conda)
        } else if path.ends_with(".tar.bz2") {
            Some(ArchiveType::TarBz2)
        } else {
            None
        }
    }
}
