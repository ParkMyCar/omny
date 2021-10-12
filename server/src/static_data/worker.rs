use anyhow::Context;
use bytes::Bytes;
use reqwest::Client;
use std::{path::PathBuf, time::Duration};
use tokio::{fs, io::AsyncWriteExt};

use super::GtfsDataSource;

const STATIC_FOLDERNAME: &str = "static-files";

enum DataComparison {
    Same,
    Different,
}

pub struct StaticDataWorker {
    client: Client,
    path: PathBuf,
    _interval: Option<Duration>,
}

impl StaticDataWorker {
    pub fn new(path: PathBuf) -> Self {
        let client = Client::new();

        StaticDataWorker {
            client,
            path,
            _interval: None,
        }
    }

    pub async fn update_all(&self) -> anyhow::Result<()> {
        for source in GtfsDataSource::all() {
            info!("updating {:?}", source);
            self.update_source(source)
                .await
                .context("updating source")?;
        }

        Ok(())
    }

    pub async fn update_source(&self, source: GtfsDataSource) -> anyhow::Result<()> {
        // fetch the gtfs data
        let bytes = self.fetch_gtfs(source).await.context("fetching source")?;
        // check if the new content is different
        let is_different = self
            .compare_content(&bytes, source)
            .await
            .context("comparing")?;

        // only save the recently fetched content if it's different
        if let DataComparison::Different = is_different {
            info!("newer content for {:?}, saving to disk!", source);
            self.save_zip(bytes, source)
                .await
                .context("updating content")?;
        }

        Ok(())
    }

    pub async fn path_for(&self, source: GtfsDataSource) -> Option<PathBuf> {
        let filename_zip = format!("{}.zip", source.filename());
        let path = self.path.join(STATIC_FOLDERNAME).join(filename_zip);
        // check if the file exists
        fs::metadata(&path).await.ok().map(|_| path)
    }

    async fn fetch_gtfs(&self, source: GtfsDataSource) -> anyhow::Result<Bytes> {
        // make a GET request to fetch the GTFS data
        self.client
            .get(source.url())
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("network error! {:?}", e))?
            .bytes()
            .await
            .map_err(|e| anyhow::anyhow!("parsing error! {:?}", e))
    }

    async fn compare_content(
        &self,
        bytes: &Bytes,
        source: GtfsDataSource,
    ) -> anyhow::Result<DataComparison> {
        let filename_zip = format!("{}.zip", source.filename());
        let path = self.path.join(STATIC_FOLDERNAME).join(filename_zip);

        let existing_bytes = match fs::read(&path).await {
            // TODO: Stream in these bytes and compute a hash, instead of reading them all at once
            Ok(bytes) => Bytes::from(bytes),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                return Ok(DataComparison::Different);
            }
            Err(e) => return Err(e).context("reading existing file"),
        };

        if bytes == &existing_bytes {
            Ok(DataComparison::Same)
        } else {
            Ok(DataComparison::Different)
        }
    }

    async fn save_zip(&self, mut bytes: Bytes, source: GtfsDataSource) -> anyhow::Result<()> {
        let filename_zip = format!("{}.zip", source.filename());
        let path = self.path.join(STATIC_FOLDERNAME).join(filename_zip);

        // make sure the static folder exists, if it doesn't exist, create it
        self.create_internal_dir(STATIC_FOLDERNAME).await?;

        // overwrite any temp file that may already exist
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&path)
            .await?;

        // write the file
        file.write_all_buf(&mut bytes)
            .await
            .context("writing bytes into zip")?;
        file.sync_all().await.context("fsyncing file to disk")?;

        // success!
        Ok(())
    }

    async fn create_internal_dir(&self, foldername: &'static str) -> anyhow::Result<bool> {
        let temp_folder_path = self.path.join(foldername);

        // check if the folder exists
        let should_create = match fs::metadata(&temp_folder_path).await {
            // already exists
            Ok(metadata) => {
                info!("temp folder already exists! {:?}", metadata);
                false
            }
            // does not exist
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                info!(
                    "folder does not exist, creating! path: {:?}",
                    temp_folder_path
                );
                true
            }
            // hit some error!
            Err(e) => {
                anyhow::bail!("failed while checking metadata for temp folder {:?}", e);
            }
        };

        // create the folder if it doesn't already exist
        if should_create {
            fs::create_dir(&temp_folder_path)
                .await
                .context("creating temp dir")?;
            info!("created temp folder!");
        }

        Ok(should_create)
    }
}
