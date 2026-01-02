mod config;
mod crawler;
mod storage;
mod scheduler;

use config::Config;
use storage::postgres::Storage;
use tracing::{info, warn, error, debug};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let cfg = Config::from_env()?;

    // Crawl listing pages (links only)
    let mut links: Vec<String> = crawler::crawl_first_pages(&cfg)
        .await?
        .into_iter()
        .collect();

    links.sort();

    info!("\n==============================");
    info!("TOTAL ITEMS FOUND: {}", links.len());
    info!("==============================\n");

    // Crawl details (RETURNS DATA)
    info!("\nFetching details for items...\n");
    let details = crawler::crawl_details(&links).await?;

    // Init storage
    let storage = Storage::new(&cfg.database_url).await?;

    // Save everything to DB
    for house in &details {
        let house_id = storage.save_house(house).await?;
        info!("Saved house external_id={} db_id={}",
            house.external_id,
            house_id
        );
    }

    info!("DONE: {} houses saved to database", details.len());

    Ok(())
}
