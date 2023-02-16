//! Rattler is an experimental library and executable to work with [Conda](http://conda.io)
//! environments. Conda is a cross-platform open-source package management system and environment
//! management system.
//!
//! Conda is originally written in Python and has evolved a lot since it was first conceived.
//! Rattler is an attempt at reimplementing a lot of the machinery supporting Conda but making it
//! available to a wider range of languages. The goal is to be able to integrate the Conda ecosystem
//! in a wide variaty of tools that do not rely on Python. Rust has excellent support for
//! interfacing with many other languages (WASM, Javascript, Python, C, etc) and is therefor a good
//! candidate for a reimplementation.

pub mod install;
pub mod package_cache;
pub mod repo_data;
pub mod solver;
pub mod validation;

pub(crate) mod utils;

/// A helper function that returns a [`Channel`] instance that points to an empty channel on disk
/// that is bundled with this repository.
#[cfg(any(doctest, test))]
pub fn empty_channel() -> rattler_conda_types::Channel {
    let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let channel_path = manifest_dir.join("resources/channels/empty");
    rattler_conda_types::Channel::from_str(
        &format!("file://{}[noarch]", channel_path.display()),
        &rattler_conda_types::ChannelConfig::default(),
    )
    .unwrap()
}

#[cfg(test)]
use std::path::{Path, PathBuf};

#[cfg(test)]
pub(crate) fn get_test_data_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../test-data")
}
