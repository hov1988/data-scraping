use crate::{
    config::Config,
    crawler,
    storage::postgres::Storage,
};
use tracing::{info, warn, error};

pub struct ScrapingService {
    cfg: Config,
    storage: Storage,
}

impl ScrapingService {
    pub async fn new(cfg: Config) -> anyhow::Result<Self> {
        let storage = Storage::new(&cfg.database_url).await?;
        Ok(Self { cfg, storage })
    }

    pub async fn run(&self) -> anyhow::Result<()> {
        let mut total_saved = 0usize;

        for page in self.cfg.start_page..=self.cfg.end_page {
            info!(page, "Processing listing page");

            let links = match crawler::crawl_page_links(&self.cfg, page).await {
                Ok(v) if !v.is_empty() => v,
                Ok(_) => {
                    info!(page, "No items on page");
                    continue;
                }
                Err(e) => {
                    warn!(page, error = %e, "Failed to crawl page links");
                    continue;
                }
            };

            info!(page, count = links.len(), "Found item links");

            let houses = match crawler::crawl_details(&links).await {
                Ok(v) if !v.is_empty() => v,
                Ok(_) => {
                    warn!(page, "No house details extracted");
                    continue;
                }
                Err(e) => {
                    error!(page, error = %e, "Failed to crawl house details");
                    continue;
                }
            };

            match self.storage.save_houses_batch(&houses).await {
                Ok(saved) => {
                    total_saved += saved;
                    info!(page, saved, total_saved, "Page saved successfully");
                }
                Err(e) => {
                    error!(page, error = %e, "Failed to save page batch");
                }
            }

            tokio::time::sleep(
                std::time::Duration::from_millis(self.cfg.delay_ms)
            ).await;
        }

        info!(total_saved, "DONE: all pages processed successfully");
        Ok(())
    }
}
