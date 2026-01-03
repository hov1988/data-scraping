mod config;
mod crawler;
mod storage;
mod scheduler;

use config::Config;
use storage::postgres::Storage;
use tracing::{info, warn, error};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let cfg = Config::from_env()?;
    let storage = Storage::new(&cfg.database_url).await?;

    let mut total_saved = 0usize;

    for page in cfg.start_page..=cfg.end_page {
        info!(page, "Processing listing page");

        // Fetch links for THIS page
        let links = match crawler::crawl_page_links(&cfg, page).await {
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

        // Fetch ALL details for this page
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

        // Save ALL houses from this page in ONE transaction
        match storage.save_houses_batch(&houses).await {
            Ok(saved) => {
                total_saved += saved;
                info!(
                    page,
                    saved,
                    total_saved,
                    "Page saved successfully"
                );
            }
            Err(e) => {
                error!(
                    page,
                    error = %e,
                    "Failed to save page batch"
                );
            }
        }

        // Polite delay between pages
        tokio::time::sleep(std::time::Duration::from_millis(cfg.delay_ms)).await;
    }

    info!(
        total_saved,
        "DONE: all pages processed successfully"
    );

    Ok(())
}
