//! # JLAP
//!
//! This module contains functions and data types for downloading and applying patches from JLAP
//! files.
//!
//! JLAP files provide a way to incrementally retrieve and build the `repodata.json` files
//! that conda compatible applications use to query conda packages. For more information about
//! how this file format works, please read this CEP proposal:
//!
//! - <https://github.com/conda-incubator/ceps/pull/20/files>
//!
//! ## Example
//!
//! The recommended way to use this module is by using the JLAPManager struct. This struct is meant
//! to act as a kind of "facade" object which orchestrates the underlying operations necessary
//! to fetch JLAP data used to update our current `repodata.json` file.
//!
//! Below is an example of how to initialize the struct and patch an existing `repodata.json` file:
//!
//! ```no_run
//! use std::{path::Path};
//! use reqwest::Client;
//! use url::Url;
//!
//! use rattler_digest::{compute_bytes_digest, Blake2b256};
//! use rattler_repodata_gateway::fetch::jlap::{patch_repo_data, JLAPState, RepoDataState};
//!
//! #[tokio::main]
//! pub async fn main() {
//!     let subdir_url = Url::parse("https://conda.anaconda.org/conda-forge/osx-64/").unwrap();
//!     let client = Client::new();
//!     let cache = Path::new("./cache");
//!     let current_repo_data = cache.join("c93ef9c9.json");
//!
//!     let repo_data_state: RepoDataState =  serde_json::from_str(r#"{
//!        "url": "https://conda.anaconda.org/conda-forge/osx-64/repodata.json.zst",
//!        "etag": "W/\"49aa6d9ea6f3285efe657780a7c8cd58\"",
//!        "mod": "Tue, 30 May 2023 20:03:48 GMT",
//!        "cache_control": "public, max-age=30",
//!        "mtime_ns": 1685509481332236078,
//!        "size": 38317593,
//!        "blake2_hash": "580100cb35459305eaaa31feeebacb06aad6422257754226d832e504666fc1c6",
//!        "has_zst": {
//!          "value": true,
//!          "last_checked": "2023-05-21T12:14:21.904003Z"
//!        },
//!        "has_bz2": {
//!          "value": true,
//!          "last_checked": "2023-05-21T12:14:21.904003Z"
//!        },
//!        "has_jlap": {
//!          "value": true,
//!          "last_checked": "2023-05-21T12:14:21.903512Z"
//!        },
//!        "jlap": {
//!          "iv": "0000000000000000000000000000000000000000000000000000000000000000",
//!          "pos": 0,
//!          "footer": {
//!            "url": "repodata.json",
//!            "latest": "580100cb35459305eaaa31feeebacb06aad6422257754226d832e504666fc1c6"
//!          }
//!        }
//!      }"#).unwrap();
//!
//!     // Patches `current_repo_data` and returns an updated JLAP state object
//!     let updated_jlap_state = patch_repo_data(
//!         &client,
//!         subdir_url,
//!         repo_data_state,
//!         &current_repo_data
//!     ).await.unwrap();
//!
//!     // Now we can use the `updated_jlap_state` object to update our `.state.json` file
//! }
//! ```
//!
//! ## TODO
//!
//! The following items still need to be implemented before this module should be considered
//! complete:
//!  - Our tests do not exhaustively cover our error states. Some of these are pretty easy to
//!    trigger (e.g. invalid JLAP file or invalid JSON within), so we should definitely make
//!    tests for them.

use blake2::digest::Output;
use blake2::digest::{FixedOutput, Update};
use rattler_digest::{compute_bytes_digest, parse_digest_from_hex, Blake2b256, Blake2bMac256};
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client, Response, StatusCode,
};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::str;
use tokio::io::AsyncWriteExt;
use url::Url;

use crate::fetch::cache;
pub use crate::fetch::cache::{JLAPFooter, JLAPState, RepoDataState};

/// File suffix for JLAP file
pub const JLAP_FILE_SUFFIX: &str = "jlap";

/// File name of JLAP file
pub const JLAP_FILE_NAME: &str = "repodata.jlap";

/// File suffix for JLAP files
pub const JLAP_FOOTER_OFFSET: usize = 2;

/// Default position for JLAP requests
pub const JLAP_START_POSITION: u64 = 0;

/// Default initialization vector for JLAP requests
pub const JLAP_START_INITIALIZATION_VECTOR: &[u8] = &[0; 32];

/// Represents the variety of errors that we come across while processing JLAP files
#[derive(Debug, thiserror::Error)]
pub enum JLAPError {
    #[error(transparent)]
    /// Pass-thru for JSON errors found while parsing JLAP file
    JSONParse(serde_json::Error),

    #[error(transparent)]
    /// Pass-thru for JSON errors found while patching
    JSONPatch(json_patch::PatchError),

    #[error(transparent)]
    /// Pass-thru for HTTP errors encountered while requesting JLAP
    HTTP(reqwest::Error),

    #[error(transparent)]
    /// Pass-thru for file system errors encountered while requesting JLAP
    FileSystem(tokio::io::Error),

    #[error("No patches found in the JLAP file")]
    /// Error returned when JLAP file has no patches in it
    NoPatchesFound,

    #[error("No matching hashes can be found in the JLAP file")]
    /// Error returned when none of the patches match the hash of our current `repodata.json`
    /// This also means we need to reset the `position` in our JLAPState to zero so we can
    /// begin our search from the beginning next time.
    NoHashFound,

    #[error("Hash from the JLAP metadata and hash from updated repodata file do not match")]
    /// Error when we have mismatched hash values after updating our `repodata.json` file
    /// At this point, we should consider the `repodata.json` file corrupted and fetch a new
    /// version from the server.
    HashesNotMatching,

    #[error("A mismatch occurred when validating the checksum on the JLAP response")]
    /// Error returned when we are unable to validate the checksum on the JLAP response.
    /// The checksum is the last line of the response.
    ChecksumMismatch,

    #[error("Unable to parse hex values in hash; cache is corrupted")]
    /// Error that occurs when we are unable to parse a hex values in the cache file.
    HexParse,
}

/// Represents the numerous patches found in a JLAP file which makes up a majority
/// of the file
#[derive(Serialize, Deserialize, Debug)]
pub struct Patch {
    /// Next hash of `repodata.json` file
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "cache::deserialize_blake2_hash",
        serialize_with = "cache::serialize_blake2_hash"
    )]
    pub to: Option<Output<Blake2b256>>,

    /// Previous hash of `repodata.json` file
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "cache::deserialize_blake2_hash",
        serialize_with = "cache::serialize_blake2_hash"
    )]
    pub from: Option<Output<Blake2b256>>,

    /// Patches to apply to current `repodata.json` file
    pub patch: json_patch::Patch, // [] is a valid, empty patch
}

/// Represents a single JLAP response
///
/// All of the data contained in this struct is everything we can determine from the
/// JLAP response itself.
#[derive(Debug)]
pub struct JLAP<'a> {
    /// First line of the JLAP response
    initialization_vector: Vec<u8>,

    /// List of patches parsed from the JLAP Response
    patches: Vec<Patch>,

    /// Footer of the request which contains data like the latest hash
    footer: JLAPFooter,

    /// Checksum located at the end of the request
    checksum: Output<Blake2b256>,

    /// Bytes offset to use for next JLAP request
    bytes_offset: u64,

    /// All the lines of the JLAP request (raw response, minus '\n' characters
    lines: Vec<&'a str>,

    /// Offset to use when iterating through JLAP response lines (i.e. position where the patches
    /// begin).
    offset: usize,
}

impl<'a> JLAP<'a> {
    /// Parses a JLAP object from Response string.
    ///
    /// To successfully parse the response, it needs to at least be three lines long.
    /// This would include a `Patch`, `JLAPFooter` and a `checksum`.
    ///
    /// On top of the response string, we also pass in a value for `initialization_vector`.
    /// If the response is not partial (i.e. it does not begin with an initialization vector,
    /// then this is the value we use. This is an important value for validating the checksum.
    pub fn new(response: &'a str, initialization_vector: &[u8]) -> Result<Self, JLAPError> {
        let lines: Vec<&str> = response.split('\n').collect();
        let length = lines.len();

        if lines.len() > JLAP_FOOTER_OFFSET {
            let mut offset = 0;

            // The first line can either be a valid hex string or JSON. We determine the `offset`
            // value to use and the value of the initialization vector here.
            let initialization_vector = match hex::decode(lines[0]) {
                Ok(value) => {
                    offset = 1;
                    value
                }
                Err(_) => initialization_vector.to_vec(),
            };

            let footer = lines[length - 2];
            let checksum =
                parse_digest_from_hex::<Blake2b256>(lines[length - 1]).unwrap_or_default();
            let bytes_offset = get_bytes_offset(&lines);

            let footer: JLAPFooter = match serde_json::from_str(footer) {
                Ok(value) => value,
                Err(err) => return Err(JLAPError::JSONParse(err)),
            };

            let patch_lines = lines[offset..length - JLAP_FOOTER_OFFSET].iter();
            let patches: Result<Vec<Patch>, JLAPError> =
                patch_lines.map(parse_patch_json).collect();

            match patches {
                Ok(patches) => {
                    if !patches.is_empty() {
                        Ok(JLAP {
                            initialization_vector,
                            patches,
                            footer,
                            checksum,
                            bytes_offset,
                            lines,
                            offset,
                        })
                    } else {
                        Err(JLAPError::NoPatchesFound)
                    }
                }
                Err(error) => Err(error),
            }
        } else {
            Err(JLAPError::NoPatchesFound)
        }
    }

    /// Applies patches to a `repo_data_json_path` file provided using the `hash` value to
    /// find the correct ones to apply.
    pub async fn apply(
        &self,
        repo_data_json_path: &Path,
        hash: Output<Blake2b256>,
    ) -> Result<(), JLAPError> {
        // We use the current hash to find which patches we need to apply
        let current_idx = find_current_patch_index(&self.patches, hash);

        return if let Some(idx) = current_idx {
            let applicable_patches: Vec<&Patch> =
                self.patches[idx..self.patches.len()].iter().collect();
            let new_hash = apply_jlap_patches(&applicable_patches, repo_data_json_path).await?;

            // TODO: This check might be a little redundant considering we have validated our
            //       checksums by now, but it could be nice to keep here for extra validation.
            //       We could remove it if performance would benefit.
            if new_hash != self.footer.latest.unwrap_or_default() {
                return Err(JLAPError::HashesNotMatching);
            }

            Ok(())
        } else {
            Err(JLAPError::NoHashFound)
        };
    }

    /// Returns a new JLAPState based on values in JLAP object
    ///
    /// We accept `position` as an argument because it is not derived from the JLAP response.
    ///
    /// The `iv` (initialization vector) value is optionally passed in because we may wish
    /// to override what was initially stored there, which would be the value calculated
    /// with `validate_checksum`.
    pub fn get_state(&self, position: u64, iv: Option<Output<Blake2b256>>) -> JLAPState {
        let iv = match iv {
            Some(value) => format!("{:x}", value),
            None => hex::encode(&self.initialization_vector),
        };
        JLAPState {
            pos: position + self.bytes_offset,
            iv,
            footer: self.footer.clone(),
        }
    }

    /// Validates the checksum present on a JLAP object
    pub fn validate_checksum(&self) -> Result<Output<Blake2b256>, JLAPError> {
        let mut iv: Option<Output<Blake2b256>> = None;
        let mut iv_values: Vec<Output<Blake2bMac256>> = vec![];
        let end = self.lines.len() - JLAP_FOOTER_OFFSET;

        for line in &self.lines[self.offset..end] {
            match iv {
                Some(value) => {
                    iv = Some(blake2b_256_hash_with_key(line.as_bytes(), &value));
                }
                None => {
                    iv = Some(blake2b_256_hash_with_key(
                        line.as_bytes(),
                        &self.initialization_vector,
                    ));
                }
            }
            iv_values.push(iv.unwrap());
        }

        if !iv_values.is_empty() {
            let new_iv = iv_values[iv_values.len() - 1];
            if new_iv != self.checksum {
                return Err(JLAPError::ChecksumMismatch);
            }

            Ok(new_iv)
        } else {
            Err(JLAPError::NoPatchesFound)
        }
    }
}

/// Calculates the bytes offset. We default to zero if we receive a shorter than
/// expected vector.
fn get_bytes_offset(lines: &Vec<&str>) -> u64 {
    if lines.len() >= JLAP_FOOTER_OFFSET {
        lines[0..lines.len() - JLAP_FOOTER_OFFSET]
            .iter()
            .map(|x| format!("{}\n", x).into_bytes().len() as u64)
            .sum()
    } else {
        0
    }
}

/// Finds the index of the of the most applicable patch to use
fn find_current_patch_index(patches: &[Patch], hash: Output<Blake2b256>) -> Option<usize> {
    patches.iter().position(|patch| patch.from == Some(hash))
}

fn parse_patch_json(line: &&str) -> Result<Patch, JLAPError> {
    serde_json::from_str(line).map_err(JLAPError::JSONParse)
}

/// Attempts to patch a current `repodata.json` file
///
/// This method first makes a request to fetch JLAP data we need. It relies on the information we
/// pass via the `repo_data_state` argument to retrieve the correct response.
///
/// After this, it will apply JSON patches to the file located at `repo_data_json_path`.
/// At the end, we compare the new `blake2b` hash with what was listed in the JLAP metadata to
/// ensure the file is correct.
///
/// The return value is the updated `JLAPState`
pub async fn patch_repo_data(
    client: &Client,
    subdir_url: Url,
    repo_data_state: RepoDataState,
    repo_data_json_path: &Path,
) -> Result<JLAPState, JLAPError> {
    // Determine the starting `position` and `initialization_vector`
    let (position, initialization_vector) =
        get_position_and_initialization_vector(repo_data_state.jlap)?;

    let jlap_url = subdir_url.join(JLAP_FILE_NAME).unwrap();

    let (response, position) = fetch_jlap_with_retry(jlap_url.as_str(), client, position).await?;
    let response_text = response.text().await.unwrap();

    let jlap = JLAP::new(&response_text, &initialization_vector)?;
    let hash = repo_data_state.blake2_hash.unwrap_or_default();
    let latest_hash = jlap.footer.latest.unwrap_or_default();

    // We already have the latest version; return early because there's nothing to do
    if latest_hash == hash {
        return Ok(jlap.get_state(position, None));
    }

    let new_iv = jlap.validate_checksum()?;

    // Applies patches and returns early if an error is encountered
    jlap.apply(repo_data_json_path, hash).await?;

    Ok(jlap.get_state(position, Some(new_iv)))
}

/// Fetches a JLAP response from server
pub async fn fetch_jlap(
    url: &str,
    client: &Client,
    range: &str,
) -> Result<Response, reqwest::Error> {
    let request_builder = client.get(url);
    let mut headers = HeaderMap::default();

    headers.insert(
        reqwest::header::RANGE,
        HeaderValue::from_str(range).unwrap(),
    );

    request_builder.headers(headers).send().await
}

/// Fetches the JLAP response but also retries in the case of a RANGE_NOT_SATISFIABLE error
///
/// When a JLAP file is updated on the server, it may cause new requests to trigger a
/// RANGE_NOT_SATISFIABLE error because the local cache is now out of sync. In this case, we
/// try the request once more from the beginning.
///
/// We return a new value for position if this was triggered so that we can update the
/// JLAPState accordingly.
pub async fn fetch_jlap_with_retry(
    url: &str,
    client: &Client,
    position: u64,
) -> Result<(Response, u64), JLAPError> {
    let range = format!("bytes={}-", position);

    match fetch_jlap(url, client, &range).await {
        Ok(response) => Ok((response, position)),
        Err(error) => {
            if error.status().unwrap_or_default() == StatusCode::RANGE_NOT_SATISFIABLE
                && position != 0
            {
                let range = "bytes=0-";
                return match fetch_jlap(url, client, range).await {
                    Ok(response) => Ok((response, 0)),
                    Err(error) => Err(JLAPError::HTTP(error)),
                };
            }
            Err(JLAPError::HTTP(error))
        }
    }
}

/// Applies JLAP patches to a `repodata.json` file
///
/// This is a multi-step process that involves:
///
/// 1. Opening and parsing the current repodata file
/// 2. Applying patches to this repodata file
/// 3. Saving this repodata file to disk
/// 4. Generating a new `blake2b` hash
///
/// The return value is the `blake2b` hash we used to verify the updated file's contents.
pub async fn apply_jlap_patches(
    patches: &Vec<&Patch>,
    repo_data_path: &Path,
) -> Result<Output<Blake2b256>, JLAPError> {
    // Open and read the current repodata into a JSON doc
    let repo_data_contents = match tokio::fs::read_to_string(repo_data_path).await {
        Ok(contents) => contents,
        Err(error) => return Err(JLAPError::FileSystem(error)),
    };

    let mut doc = match serde_json::from_str(&repo_data_contents) {
        Ok(doc) => doc,
        Err(error) => return Err(JLAPError::JSONParse(error)),
    };

    // Apply the patches we current have to it
    for patch in patches {
        if let Err(error) = json_patch::patch(&mut doc, &patch.patch) {
            return Err(JLAPError::JSONPatch(error));
        }
    }

    // Save the updated repodata JSON doc
    let mut updated_file = match tokio::fs::File::create(repo_data_path).await {
        Ok(file) => file,
        Err(error) => return Err(JLAPError::FileSystem(error)),
    };

    let mut updated_json = match serde_json::to_string_pretty(&doc) {
        Ok(value) => value,
        Err(error) => return Err(JLAPError::JSONParse(error)),
    };

    // We need to add an extra newline character to the end of our string so the hashes match 🤷‍
    updated_json.insert(updated_json.len(), '\n');
    let content = updated_json.into_bytes();

    match updated_file.write_all(&content).await {
        Ok(_) => Ok(compute_bytes_digest::<Blake2b256>(content)),
        Err(error) => Err(JLAPError::FileSystem(error)),
    }
}

/// Retrieves the correct values for `position` and `initialization_vector` from a JLAPState object
///
/// If we cannot find the correct values, we provide defaults from this module.
/// When we can correctly parse a hex string (the `initialization_vector` should always be a
/// hex string), we return an error.
fn get_position_and_initialization_vector(
    state: Option<JLAPState>,
) -> Result<(u64, Vec<u8>), JLAPError> {
    match state {
        Some(state) => {
            let initialization_vector = match hex::decode(state.iv) {
                Ok(value) => value,
                Err(_) => return Err(JLAPError::HexParse),
            };
            Ok((state.pos, initialization_vector))
        }
        None => Ok((
            JLAP_START_POSITION,
            JLAP_START_INITIALIZATION_VECTOR.to_vec(),
        )),
    }
}

/// Creates a keyed hash
fn blake2b_256_hash_with_key(data: &[u8], key: &[u8]) -> Output<Blake2bMac256> {
    let mut state = Blake2bMac256::new_with_salt_and_personal(key, &[], &[]).unwrap();
    state.update(data);
    state.finalize_fixed()
}

#[cfg(test)]
mod test {
    use super::patch_repo_data;
    use std::path::PathBuf;

    use crate::fetch::cache::RepoDataState;
    use crate::utils::simple_channel_server::SimpleChannelServer;

    use rattler_digest::{parse_digest_from_hex, Blake2b256};
    use reqwest::Client;
    use tempfile::TempDir;
    use url::Url;

    const FAKE_STATE_DATA_INITIAL: &str = r#"{
  "url": "https://repo.example.com/pkgs/main/osx-64/repodata.json.zst",
  "etag": "W/\"49aa6d9ea6f3285efe657780a7c8cd58\"",
  "mod": "Tue, 30 May 2023 20:03:48 GMT",
  "cache_control": "public, max-age=30",
  "mtime_ns": 1685509481332236078,
  "size": 38317593,
  "blake2_hash": "580100cb35459305eaaa31feeebacb06aad6422257754226d832e504666fc1c6",
  "has_zst": {
    "value": true,
    "last_checked": "2023-05-21T12:14:21.904003Z"
  },
  "has_bz2": {
    "value": true,
    "last_checked": "2023-05-21T12:14:21.904003Z"
  },
  "has_jlap": {
    "value": true,
    "last_checked": "2023-05-21T12:14:21.903512Z"
  }
}"#;

    const FAKE_STATE_DATA_PARTIAL: &str = r#"{
  "url": "https://repo.example.com/pkgs/main/osx-64/repodata.json.zst",
  "etag": "W/\"49aa6d9ea6f3285efe657780a7c8cd58\"",
  "mod": "Tue, 30 May 2023 20:03:48 GMT",
  "cache_control": "public, max-age=30",
  "mtime_ns": 1685509481332236078,
  "size": 38317593,
  "blake2_hash": "9b76165ba998f77b2f50342006192bf28817dad474d78d760ab12cc0260e3ed9",
  "has_zst": {
    "value": true,
    "last_checked": "2023-05-21T12:14:21.904003Z"
  },
  "has_bz2": {
    "value": true,
    "last_checked": "2023-05-21T12:14:21.904003Z"
  },
  "has_jlap": {
    "value": true,
    "last_checked": "2023-05-21T12:14:21.903512Z"
  },
  "jlap": {
    "iv": "5ec4a4fc3afd07b398ed78ffbd30ce3ef7c1f935f0e0caffc61455352ceedeff",
    "pos": 738,
    "footer": {
      "url": "repodata.json",
      "latest": "9b76165ba998f77b2f50342006192bf28817dad474d78d760ab12cc0260e3ed9"
    }
  }
}"#;

    const FAKE_REPO_DATA_INITIAL: &str = r#"{
  "info": {
    "subdir": "osx-64"
  },
  "packages.conda": {
    "zstd-1.5.4-hc035e20_0.conda": {
      "build": "hc035e20_0",
      "build_number": 0,
      "depends": [
        "libcxx >=14.0.6",
        "lz4-c >=1.9.4,<1.10.0a0",
        "xz >=5.2.10,<6.0a0",
        "zlib >=1.2.13,<1.3.0a0"
      ],
      "license": "BSD-3-Clause AND GPL-2.0-or-later",
      "license_family": "BSD",
      "md5": "f284fea068c51b1a0eaea3ac58c300c0",
      "name": "zstd",
      "sha256": "0af4513ef7ad7fa8854fa714130c25079f3744471fc106f47df80eb10c34429d",
      "size": 605550,
      "subdir": "osx-64",
      "timestamp": 1680034665911,
      "version": "1.5.4"
    }
  },
  "repodata_version": 1
}
"#;

    const FAKE_REPO_DATA_UPDATE_ONE: &str = r#"{
  "info": {
    "subdir": "osx-64"
  },
  "packages.conda": {
    "zstd-1.5.4-hc035e20_0.conda": {
      "build": "hc035e20_0",
      "build_number": 0,
      "depends": [
        "libcxx >=14.0.6",
        "lz4-c >=1.9.4,<1.10.0a0",
        "xz >=5.2.10,<6.0a0",
        "zlib >=1.2.13,<1.3.0a0"
      ],
      "license": "BSD-3-Clause AND GPL-2.0-or-later",
      "license_family": "BSD",
      "md5": "f284fea068c51b1a0eaea3ac58c300c0",
      "name": "zstd",
      "sha256": "0af4513ef7ad7fa8854fa714130c25079f3744471fc106f47df80eb10c34429d",
      "size": 605550,
      "subdir": "osx-64",
      "timestamp": 1680034665911,
      "version": "1.5.4"
    },
    "zstd-1.5.5-hc035e20_0.conda": {
      "build": "hc035e20_0",
      "build_number": 0,
      "depends": [
        "libcxx >=14.0.6",
        "lz4-c >=1.9.4,<1.10.0a0",
        "xz >=5.2.10,<6.0a0",
        "zlib >=1.2.13,<1.3.0a0"
      ],
      "license": "BSD-3-Clause AND GPL-2.0-or-later",
      "license_family": "BSD",
      "md5": "5e0b7ddb1b7dc6b630e1f9a03499c19c",
      "name": "zstd",
      "sha256": "5b192501744907b841de036bb89f5a2776b4cac5795ccc25dcaebeac784db038",
      "size": 622467,
      "subdir": "osx-64",
      "timestamp": 1681304595869,
      "version": "1.5.5"
    }
  },
  "repodata_version": 1
}
"#;

    const FAKE_REPO_DATA_UPDATE_TWO: &str = r#"{
  "info": {
    "subdir": "osx-64"
  },
  "packages.conda": {
    "zstd-1.5.4-hc035e20_0.conda": {
      "build": "hc035e20_0",
      "build_number": 0,
      "depends": [
        "libcxx >=14.0.6",
        "lz4-c >=1.9.4,<1.10.0a0",
        "xz >=5.2.10,<6.0a0",
        "zlib >=1.2.13,<1.3.0a0"
      ],
      "license": "BSD-3-Clause AND GPL-2.0-or-later",
      "license_family": "BSD",
      "md5": "f284fea068c51b1a0eaea3ac58c300c0",
      "name": "zstd",
      "sha256": "0af4513ef7ad7fa8854fa714130c25079f3744471fc106f47df80eb10c34429d",
      "size": 605550,
      "subdir": "osx-64",
      "timestamp": 1680034665911,
      "version": "1.5.4"
    },
    "zstd-1.5.5-hc035e20_0.conda": {
      "build": "hc035e20_0",
      "build_number": 0,
      "depends": [
        "libcxx >=14.0.6",
        "lz4-c >=1.9.4,<1.10.0a0",
        "xz >=5.2.10,<6.0a0",
        "zlib >=1.2.13,<1.3.0a0"
      ],
      "license": "BSD-3-Clause AND GPL-2.0-or-later",
      "license_family": "BSD",
      "md5": "5e0b7ddb1b7dc6b630e1f9a03499c19c",
      "name": "zstd",
      "sha256": "5b192501744907b841de036bb89f5a2776b4cac5795ccc25dcaebeac784db038",
      "size": 622467,
      "subdir": "osx-64",
      "timestamp": 1681304595869,
      "version": "1.5.5"
    },
    "zstd-static-1.4.5-hb1e8313_0.conda": {
      "build": "hb1e8313_0",
      "build_number": 0,
      "depends": [
        "libcxx >=10.0.0",
        "zstd 1.4.5 h41d2c2f_0"
      ],
      "license": "BSD 3-Clause",
      "md5": "5447986040e0b73d6c681a4d8f615d6c",
      "name": "zstd-static",
      "sha256": "3759ab53ff8320d35c6db00d34059ba99058eeec1cbdd0da968c5e12f73f7658",
      "size": 13930,
      "subdir": "osx-64",
      "timestamp": 1595965109852,
      "version": "1.4.5"
    }
  },
  "repodata_version": 1
}
"#;

    const FAKE_REPO_DATA_UPDATE_ONE_HASH: &str =
        "9b76165ba998f77b2f50342006192bf28817dad474d78d760ab12cc0260e3ed9";

    const FAKE_REPO_DATA_UPDATE_TWO_HASH: &str =
        "160b529c5f72b9755f951c1b282705d49d319a5f2f80b33fb1a670d02ddeacf9";

    const FAKE_JLAP_DATA_INITIAL: &str = r#"0000000000000000000000000000000000000000000000000000000000000000
{"to": "9b76165ba998f77b2f50342006192bf28817dad474d78d760ab12cc0260e3ed9", "from": "580100cb35459305eaaa31feeebacb06aad6422257754226d832e504666fc1c6", "patch": [{"op": "add", "path": "/packages.conda/zstd-1.5.5-hc035e20_0.conda", "value": {"build": "hc035e20_0","build_number": 0,"depends": ["libcxx >=14.0.6","lz4-c >=1.9.4,<1.10.0a0","xz >=5.2.10,<6.0a0","zlib >=1.2.13,<1.3.0a0"],"license": "BSD-3-Clause AND GPL-2.0-or-later","license_family": "BSD","md5": "5e0b7ddb1b7dc6b630e1f9a03499c19c","name": "zstd","sha256": "5b192501744907b841de036bb89f5a2776b4cac5795ccc25dcaebeac784db038","size": 622467,"subdir": "osx-64","timestamp": 1681304595869, "version": "1.5.5"}}]}
{"url": "repodata.json", "latest": "9b76165ba998f77b2f50342006192bf28817dad474d78d760ab12cc0260e3ed9"}
5ec4a4fc3afd07b398ed78ffbd30ce3ef7c1f935f0e0caffc61455352ceedeff"#;

    const FAKE_JLAP_DATA_UPDATE_ONE: &str = r#"0000000000000000000000000000000000000000000000000000000000000000
{"to": "9b76165ba998f77b2f50342006192bf28817dad474d78d760ab12cc0260e3ed9", "from": "580100cb35459305eaaa31feeebacb06aad6422257754226d832e504666fc1c6", "patch": [{"op": "add", "path": "/packages.conda/zstd-1.5.5-hc035e20_0.conda", "value": {"build": "hc035e20_0","build_number": 0,"depends": ["libcxx >=14.0.6","lz4-c >=1.9.4,<1.10.0a0","xz >=5.2.10,<6.0a0","zlib >=1.2.13,<1.3.0a0"],"license": "BSD-3-Clause AND GPL-2.0-or-later","license_family": "BSD","md5": "5e0b7ddb1b7dc6b630e1f9a03499c19c","name": "zstd","sha256": "5b192501744907b841de036bb89f5a2776b4cac5795ccc25dcaebeac784db038","size": 622467,"subdir": "osx-64","timestamp": 1681304595869, "version": "1.5.5"}}]}
{"to": "160b529c5f72b9755f951c1b282705d49d319a5f2f80b33fb1a670d02ddeacf9", "from": "9b76165ba998f77b2f50342006192bf28817dad474d78d760ab12cc0260e3ed9", "patch": [{"op": "add", "path": "/packages.conda/zstd-static-1.4.5-hb1e8313_0.conda", "value": {"build": "hb1e8313_0", "build_number": 0, "depends": ["libcxx >=10.0.0", "zstd 1.4.5 h41d2c2f_0"], "license": "BSD 3-Clause", "md5": "5447986040e0b73d6c681a4d8f615d6c", "name": "zstd-static", "sha256": "3759ab53ff8320d35c6db00d34059ba99058eeec1cbdd0da968c5e12f73f7658", "size": 13930, "subdir": "osx-64", "timestamp": 1595965109852, "version": "1.4.5"}}]}
{"url": "repodata.json", "latest": "160b529c5f72b9755f951c1b282705d49d319a5f2f80b33fb1a670d02ddeacf9"}
7d6e2b5185cf5e14f852355dc79eeba1233550d974f274f1eaf7db21c7b2c4e8"#;

    /// Writes the desired files to the "server" environment
    async fn setup_server_environment(
        server_repo_data: Option<&str>,
        server_jlap: Option<&str>,
    ) -> TempDir {
        // Create a directory with some repodata; this is the "server" data
        let subdir_path = TempDir::new().unwrap();

        if let Some(content) = server_jlap {
            // Add files we need to request to the server
            tokio::fs::write(subdir_path.path().join("repodata.jlap"), content)
                .await
                .unwrap();
        }

        if let Some(content) = server_repo_data {
            // Add files we need to request to the server
            tokio::fs::write(subdir_path.path().join("repodata.json"), content)
                .await
                .unwrap();
        }

        subdir_path
    }

    /// Writes the desired files to the "client" environment
    async fn setup_client_environment(
        server_url: &Url,
        cache_repo_data: Option<&str>,
    ) -> (TempDir, PathBuf) {
        // Create our cache location and files we need there; this is our "cache" location
        let cache_dir = TempDir::new().unwrap();

        // This is the existing `repodata.json` file that will be patched
        let cache_key = crate::utils::url_to_cache_filename(
            &server_url
                .join("repodata.json")
                .expect("file name is valid"),
        );
        let cache_repo_data_path = cache_dir.path().join(format!("{}.json", cache_key));

        if let Some(content) = cache_repo_data {
            tokio::fs::write(cache_repo_data_path.clone(), content)
                .await
                .unwrap();
        }

        (cache_dir, cache_repo_data_path)
    }

    #[tokio::test]
    /// Performs a test to make sure that patches can be applied when we retrieve
    /// a "fresh" (i.e. no bytes offset) version of the JLAP file.
    pub async fn test_patch_repo_data() {
        // Begin setup
        let subdir_path = setup_server_environment(None, Some(FAKE_JLAP_DATA_INITIAL)).await;
        let server = SimpleChannelServer::new(subdir_path.path());
        let server_url = server.url();

        let (_cache_dir, cache_repo_data_path) =
            setup_client_environment(&server_url, Some(FAKE_REPO_DATA_INITIAL)).await;

        let client = Client::default();

        let repo_data_state: RepoDataState = serde_json::from_str(FAKE_STATE_DATA_INITIAL).unwrap();
        // End setup

        let updated_jlap_state =
            patch_repo_data(&client, server_url, repo_data_state, &cache_repo_data_path)
                .await
                .unwrap();

        // Make assertions
        let repo_data = tokio::fs::read_to_string(cache_repo_data_path)
            .await
            .unwrap();

        // Ensure the repo data was updated appropriately
        assert_eq!(repo_data, FAKE_REPO_DATA_UPDATE_ONE);

        // Ensure the the updated JLAP state matches what it should
        assert_eq!(updated_jlap_state.pos, 738);
        assert_eq!(
            updated_jlap_state.iv,
            "5ec4a4fc3afd07b398ed78ffbd30ce3ef7c1f935f0e0caffc61455352ceedeff"
        );
        assert_eq!(
            updated_jlap_state.footer.latest.unwrap_or_default(),
            parse_digest_from_hex::<Blake2b256>(FAKE_REPO_DATA_UPDATE_ONE_HASH).unwrap()
        );
    }

    #[tokio::test]
    /// Performs a test to make sure that patches can be applied when we retrieve
    /// a "partial" (i.e. one with a byte offset) version of the JLAP file.
    pub async fn test_patch_repo_data_partial() {
        // Begin setup
        let subdir_path = setup_server_environment(None, Some(FAKE_JLAP_DATA_UPDATE_ONE)).await;
        let server = SimpleChannelServer::new(subdir_path.path());
        let server_url = server.url();

        let (_cache_dir, cache_repo_data_path) =
            setup_client_environment(&server_url, Some(FAKE_REPO_DATA_UPDATE_ONE)).await;

        let client = Client::default();

        let repo_data_state: RepoDataState = serde_json::from_str(FAKE_STATE_DATA_PARTIAL).unwrap();
        // End setup

        // Run the code under test
        let updated_jlap_state =
            patch_repo_data(&client, server_url, repo_data_state, &cache_repo_data_path)
                .await
                .unwrap();

        // Make assertions
        let repo_data = tokio::fs::read_to_string(cache_repo_data_path)
            .await
            .unwrap();

        // Ensure the repo data was updated appropriately
        assert_eq!(repo_data, FAKE_REPO_DATA_UPDATE_TWO);

        // Ensure the the updated JLAP state matches what it should
        assert_eq!(updated_jlap_state.pos, 1341);
        assert_eq!(
            updated_jlap_state.iv,
            "7d6e2b5185cf5e14f852355dc79eeba1233550d974f274f1eaf7db21c7b2c4e8"
        );
        assert_eq!(
            updated_jlap_state.footer.latest.unwrap_or_default(),
            parse_digest_from_hex::<Blake2b256>(FAKE_REPO_DATA_UPDATE_TWO_HASH).unwrap()
        );
    }
}
