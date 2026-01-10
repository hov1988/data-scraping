use crate::storage::postgres::Storage;
use tracing::{info, warn};
use tokio::time::{sleep, Duration};

const BATCH_SIZE: i64 = 100;

pub struct RemovalCheckService {
    storage: Storage,
    client: reqwest::Client,
}

impl RemovalCheckService {
    pub fn new(storage: Storage) -> Self {
        let client = reqwest::Client::builder()
            .redirect(reqwest::redirect::Policy::limited(5))
            .build()
            .unwrap();

        Self { storage, client }
    }

    pub async fn run(&self) -> anyhow::Result<()> {
        let mut offset = 0;
        let mut total_marked = 0usize;

        loop {
            let batch = self
                .storage
                .fetch_active_houses_batch(BATCH_SIZE, offset)
                .await?;

            if batch.is_empty() {
                break;
            }

            info!(
                offset,
                batch_size = batch.len(),
                "Checking batch for removed pages"
            );

            let mut removed_ids = Vec::new();

            for (id, url) in &batch {
                if self.page_is_removed(url).await {
                    removed_ids.push(*id);
                }

                // polite delay per request
                sleep(Duration::from_millis(200)).await;
            }

            if !removed_ids.is_empty() {
                warn!(
                    count = removed_ids.len(),
                    "Marking houses as deleted"
                );

                self.storage
                    .mark_houses_as_deleted(&removed_ids)
                    .await?;

                total_marked += removed_ids.len();
            }

            offset += BATCH_SIZE;
        }

        info!(total_marked, "Removal check finished");
        Ok(())
    }

    async fn page_is_removed(&self, url: &str) -> bool {
        match self.client.get(url).send().await {
            Ok(resp) if resp.status() == reqwest::StatusCode::NOT_FOUND => true,

            Ok(resp) if resp.status().is_success() => false,

            Ok(resp) => {
                let body = resp.text().await.unwrap_or_default();
                body.contains("not found")
                    || body.contains("removed")
                    || body.contains("deleted")
                    || body.contains("no longer available")
            }

            Err(_) => false, // network errors ≠ removal
        }
    }
}
