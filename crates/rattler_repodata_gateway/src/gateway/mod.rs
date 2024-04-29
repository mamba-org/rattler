mod barrier_cell;
mod channel_config;
mod error;
mod local_subdir;
mod remote_subdir;
mod repo_data;
mod sharded_subdir;
mod subdir;

pub use barrier_cell::BarrierCell;
pub use channel_config::{ChannelConfig, SourceConfig};
pub use error::GatewayError;

use crate::fetch::FetchRepoDataError;
use crate::gateway::repo_data::RepoData;
use dashmap::{mapref::entry::Entry, DashMap};
use futures::{select_biased, stream::FuturesUnordered, StreamExt};
use itertools::Itertools;
use local_subdir::LocalSubdirClient;
use rattler_conda_types::{Channel, PackageName, Platform};
use reqwest::Client;
use reqwest_middleware::ClientWithMiddleware;
use std::{
    borrow::Borrow,
    collections::HashSet,
    path::PathBuf,
    sync::{Arc, Weak},
};
use subdir::{Subdir, SubdirData};
use tokio::sync::broadcast;

// TODO: Instead of using `Channel` it would be better if we could use just the base url. Maybe we
//  can wrap that in a type. Mamba has the CondaUrl class.

/// A builder for constructing a [`Gateway`].
#[derive(Default)]
pub struct GatewayBuilder {
    channel_config: ChannelConfig,
    client: Option<ClientWithMiddleware>,
    cache: Option<PathBuf>,
}

impl GatewayBuilder {
    /// New instance of the builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the client to use for fetching repodata.
    #[must_use]
    pub fn with_client(mut self, client: ClientWithMiddleware) -> Self {
        self.client = Some(client);
        self
    }

    /// Set the channel configuration to use for fetching repodata.
    #[must_use]
    pub fn with_channel_config(mut self, channel_config: ChannelConfig) -> Self {
        self.channel_config = channel_config;
        self
    }

    /// Set the directory to use for caching repodata.
    #[must_use]
    pub fn with_cache_dir(mut self, cache: impl Into<PathBuf>) -> Self {
        self.cache = Some(cache.into());
        self
    }

    /// Finish the construction of the gateway returning a constructed gateway.
    pub fn finish(self) -> Gateway {
        let client = self
            .client
            .unwrap_or_else(|| ClientWithMiddleware::from(Client::new()));

        let cache = self.cache.unwrap_or_else(|| {
            dirs::cache_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join("rattler/cache")
        });

        Gateway {
            inner: Arc::new(GatewayInner {
                subdirs: Default::default(),
                client,
                channel_config: self.channel_config,
                cache,
            }),
        }
    }
}

/// Central access point for fetching repodata records.
#[derive(Clone)]
pub struct Gateway {
    inner: Arc<GatewayInner>,
}

impl Gateway {
    /// Constructs a simple gateway with the default configuration. Use [`Gateway::builder`] if you
    /// want more control over how the gateway is constructed.
    pub fn new() -> Self {
        Gateway::builder().finish()
    }

    /// Constructs a new gateway with the given client and channel configuration.
    pub fn builder() -> GatewayBuilder {
        GatewayBuilder::default()
    }

    /// Recursively loads all repodata records for the given channels, platforms and package names.
    ///
    /// This function will asynchronously load the repodata from all subdirectories (combination of
    /// channels and platforms) and recursively load all repodata records and the dependencies of
    /// the those records.
    ///
    /// Most processing will happen on the background so downloading and parsing can happen
    /// simultaneously.
    ///
    /// Repodata is cached by the [`Gateway`] so calling this function twice with the same channels
    /// will not result in the repodata being fetched twice.
    pub async fn load_records_recursive<
        AsChannel,
        ChannelIter,
        PlatformIter,
        PackageNameIter,
        IntoPackageName,
    >(
        &self,
        channels: ChannelIter,
        platforms: PlatformIter,
        names: PackageNameIter,
    ) -> Result<Vec<RepoData>, GatewayError>
    where
        AsChannel: Borrow<Channel> + Clone,
        ChannelIter: IntoIterator<Item = AsChannel>,
        PlatformIter: IntoIterator<Item = Platform>,
        <PlatformIter as IntoIterator>::IntoIter: Clone,
        PackageNameIter: IntoIterator<Item = IntoPackageName>,
        IntoPackageName: Into<PackageName>,
    {
        // Collect all the channels and platforms together
        let channels = channels.into_iter().collect_vec();
        let channel_count = channels.len();
        let channels_and_platforms = channels
            .into_iter()
            .enumerate()
            .cartesian_product(platforms.into_iter())
            .collect_vec();

        // Create barrier cells for each subdirectory. This can be used to wait until the subdir
        // becomes available.
        let mut subdirs = Vec::with_capacity(channels_and_platforms.len());
        let mut pending_subdirs = FuturesUnordered::new();
        for ((channel_idx, channel), platform) in channels_and_platforms.into_iter() {
            // Create a barrier so work that need this subdir can await it.
            let barrier = Arc::new(BarrierCell::new());
            subdirs.push((channel_idx, barrier.clone()));

            let inner = self.inner.clone();
            pending_subdirs.push(async move {
                let subdir = inner
                    .get_or_create_subdir(channel.borrow(), platform)
                    .await?;
                barrier.set(subdir).expect("subdir was set twice");
                Ok(())
            });
        }

        // Package names that we have or will issue requests for.
        let mut seen = names.into_iter().map(Into::into).collect::<HashSet<_>>();

        // Package names that we still need to fetch.
        let mut pending_package_names = seen.iter().cloned().collect::<Vec<_>>();

        // A list of futures to fetch the records for the pending package names. The main task
        // awaits these futures.
        let mut pending_records = FuturesUnordered::new();

        // The resulting list of repodata records.
        let mut result = vec![RepoData::default(); channel_count];

        // Loop until all pending package names have been fetched.
        loop {
            // Iterate over all pending package names and create futures to fetch them from all
            // subdirs.
            for pending_package_name in pending_package_names.drain(..) {
                for (channel_idx, subdir) in subdirs.iter().cloned() {
                    let pending_package_name = pending_package_name.clone();
                    pending_records.push(async move {
                        let barrier_cell = subdir.clone();
                        let subdir = barrier_cell.wait().await;
                        match subdir.as_ref() {
                            Subdir::Found(subdir) => subdir
                                .get_or_fetch_package_records(&pending_package_name)
                                .await
                                .map(|records| (channel_idx, records)),
                            Subdir::NotFound => Ok((channel_idx, Arc::from(vec![]))),
                        }
                    });
                }
            }

            // Wait for the subdir to become available.
            select_biased! {
                // Handle any error that was emitted by the pending subdirs.
                subdir_result = pending_subdirs.select_next_some() => {
                    if let Err(subdir_result) = subdir_result {
                        return Err(subdir_result);
                    }
                }

                // Handle any records that were fetched
                records = pending_records.select_next_some() => {
                    let (channel_idx, records) = records?;

                    // Extract the dependencies from the records and recursively add them to the
                    // list of package names that we need to fetch.
                    for record in records.iter() {
                        for dependency in &record.package_record.depends {
                            let dependency_name = PackageName::new_unchecked(
                                dependency.split_once(' ').unwrap_or((dependency, "")).0,
                            );
                            if seen.insert(dependency_name.clone()) {
                                pending_package_names.push(dependency_name.clone());
                            }
                        }
                    }

                    // Add the records to the result
                    if records.len() > 0 {
                        let result = &mut result[channel_idx];
                        result.len += records.len();
                        result.shards.push(records);
                    }
                }

                // All futures have been handled, all subdirectories have been loaded and all
                // repodata records have been fetched.
                complete => {
                    break;
                }
            }
        }

        Ok(result)
    }
}

struct GatewayInner {
    /// A map of subdirectories for each channel and platform.
    subdirs: DashMap<(Channel, Platform), PendingOrFetched<Arc<Subdir>>>,

    /// The client to use to fetch repodata.
    client: ClientWithMiddleware,

    /// The channel configuration
    channel_config: ChannelConfig,

    /// The directory to store any cache
    cache: PathBuf,
}

impl GatewayInner {
    /// Returns the [`Subdir`] for the given channel and platform. This function will create the
    /// [`Subdir`] if it does not exist yet, otherwise it will return the previously created subdir.
    ///
    /// If multiple threads request the same subdir their requests will be coalesced, and they will
    /// all receive the same subdir. If an error occurs while creating the subdir all waiting tasks
    /// will also return an error.
    async fn get_or_create_subdir(
        &self,
        channel: &Channel,
        platform: Platform,
    ) -> Result<Arc<Subdir>, GatewayError> {
        let sender = match self.subdirs.entry((channel.clone(), platform)) {
            Entry::Vacant(entry) => {
                // Construct a sender so other tasks can subscribe
                let (sender, _) = broadcast::channel(1);
                let sender = Arc::new(sender);

                // Modify the current entry to the pending entry, this is an atomic operation
                // because who holds the entry holds mutable access.
                entry.insert(PendingOrFetched::Pending(Arc::downgrade(&sender)));

                sender
            }
            Entry::Occupied(mut entry) => {
                let subdir = entry.get();
                match subdir {
                    PendingOrFetched::Pending(sender) => {
                        let sender = sender.upgrade();

                        if let Some(sender) = sender {
                            // Explicitly drop the entry, so we don't block any other tasks.
                            drop(entry);

                            // The sender is still active, so we can wait for the subdir to be
                            // created.
                            return match sender.subscribe().recv().await {
                                Ok(subdir) => Ok(subdir),
                                Err(_) => {
                                    // If this happens the sender was dropped.
                                    Err(GatewayError::IoError(
                                        "a coalesced request failed".to_string(),
                                        std::io::ErrorKind::Other.into(),
                                    ))
                                }
                            };
                        } else {
                            // Construct a sender so other tasks can subscribe
                            let (sender, _) = broadcast::channel(1);
                            let sender = Arc::new(sender);

                            // Modify the current entry to the pending entry, this is an atomic
                            // operation because who holds the entry holds mutable access.
                            entry.insert(PendingOrFetched::Pending(Arc::downgrade(&sender)));

                            sender
                        }
                    }
                    PendingOrFetched::Fetched(records) => return Ok(records.clone()),
                }
            }
        };

        // At this point we have exclusive write access to this specific entry. All other tasks
        // will find a pending entry and will wait for the records to become available.
        //
        // Let's start by creating the subdir. If an error occurs we immediately return the error.
        // This will drop the sender and all other waiting tasks will receive an error.
        let subdir = Arc::new(self.create_subdir(channel, platform).await?);

        // Store the fetched files in the entry.
        self.subdirs.insert(
            (channel.clone(), platform),
            PendingOrFetched::Fetched(subdir.clone()),
        );

        // Send the records to all waiting tasks. We don't care if there are no receivers, so we
        // drop the error.
        let _ = sender.send(subdir.clone());

        Ok(subdir)
    }

    async fn create_subdir(
        &self,
        channel: &Channel,
        platform: Platform,
    ) -> Result<Subdir, GatewayError> {
        let url = channel.platform_url(platform);
        let subdir_data = if url.scheme() == "file" {
            if let Ok(path) = url.to_file_path() {
                LocalSubdirClient::from_directory(&path)
                    .await
                    .map(SubdirData::from_client)
            } else {
                return Err(GatewayError::UnsupportedUrl(
                    "unsupported file based url".to_string(),
                ));
            }
        } else if url.scheme() == "http" || url.scheme() == "https" {
            if url
                .as_str()
                .starts_with("https://conda.anaconda.org/conda-forge/")
            {
                sharded_subdir::ShardedSubdir::new(
                    channel.clone(),
                    platform.to_string(),
                    self.client.clone(),
                    self.cache.clone(),
                )
                .await
                .map(SubdirData::from_client)
            } else {
                remote_subdir::RemoteSubdirClient::new(
                    channel.clone(),
                    platform,
                    self.client.clone(),
                    self.cache.clone(),
                    self.channel_config.get(channel).clone(),
                )
                .await
                .map(SubdirData::from_client)
            }
        } else {
            return Err(GatewayError::UnsupportedUrl(format!(
                "'{}' is not a supported scheme",
                url.scheme()
            )));
        };

        match subdir_data {
            Ok(client) => Ok(Subdir::Found(client)),
            Err(GatewayError::FetchRepoDataError(FetchRepoDataError::NotFound(_)))
                if platform != Platform::NoArch =>
            {
                // If the subdir was not found and the platform is not `noarch` we assume its just
                // empty.
                Ok(Subdir::NotFound)
            }
            Err(err) => Err(err),
        }
    }
}

/// A record that is either pending or has been fetched.
#[derive(Clone)]
enum PendingOrFetched<T> {
    Pending(Weak<broadcast::Sender<T>>),
    Fetched(T),
}

#[cfg(test)]
mod test {
    use crate::gateway::Gateway;
    use crate::utils::simple_channel_server::SimpleChannelServer;
    use rattler_conda_types::{Channel, PackageName, Platform};
    use std::path::Path;
    use std::str::FromStr;
    use std::time::Instant;
    use url::Url;

    fn local_conda_forge() -> Channel {
        Channel::from_directory(
            &Path::new(env!("CARGO_MANIFEST_DIR")).join("../../test-data/channels/conda-forge"),
        )
    }

    async fn remote_conda_forge() -> SimpleChannelServer {
        SimpleChannelServer::new(
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../test-data/channels/conda-forge"),
        )
        .await
    }

    #[tokio::test]
    async fn test_local_gateway() {
        let gateway = Gateway::new();

        let records = gateway
            .load_records_recursive(
                vec![local_conda_forge()],
                vec![Platform::Linux64, Platform::Win32, Platform::NoArch],
                vec![PackageName::from_str("rubin-env").unwrap()].into_iter(),
            )
            .await
            .unwrap();

        let total_records: usize = records.iter().map(|r| r.len()).sum();
        assert_eq!(total_records, 45060);
    }

    #[tokio::test]
    async fn test_remote_gateway() {
        let gateway = Gateway::new();

        let index = remote_conda_forge().await;

        let records = gateway
            .load_records_recursive(
                vec![index.channel()],
                vec![Platform::Linux64, Platform::Win32, Platform::NoArch],
                vec![PackageName::from_str("rubin-env").unwrap()].into_iter(),
            )
            .await
            .unwrap();

        let total_records: usize = records.iter().map(|r| r.len()).sum();
        assert_eq!(total_records, 45060);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_sharded_gateway() {
        let gateway = Gateway::new();

        let start = Instant::now();
        let records = gateway
            .load_records_recursive(
                vec![Channel::from_url(
                    Url::parse("https://conda.anaconda.org/conda-forge").unwrap(),
                )],
                vec![Platform::Linux64, Platform::NoArch],
                vec![
                    PackageName::from_str("rubin-env").unwrap(),
                    // PackageName::from_str("jupyterlab").unwrap(),
                    // PackageName::from_str("detectron2").unwrap(),

                    // PackageName::from_str("python").unwrap(),
                    // PackageName::from_str("boto3").unwrap(),
                    // PackageName::from_str("requests").unwrap(),
                ]
                .into_iter(),
            )
            .await
            .unwrap();
        let end = Instant::now();
        println!("{} records in {:?}", records.len(), end - start);

        let total_records: usize = records.iter().map(|r| r.len()).sum();
        assert_eq!(total_records, 84242);
    }
}
