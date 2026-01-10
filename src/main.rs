mod config;
mod crawler;
mod storage;
mod scheduler;
mod checker;

use std::env;

use config::Config;
use crawler::service::ScrapingService;
use checker::service::RemovalCheckService;
use storage::postgres::Storage;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let mode = env::args().nth(1).unwrap_or_else(|| "scraper".to_string());

    let cfg = Config::from_env()?;

    match mode.as_str() {
        "scraper" => {
            let service = ScrapingService::new(cfg).await?;
            service.run().await?;
        }

        "checker" => {
            let storage = Storage::new(&cfg.database_url).await?;
            let checker = RemovalCheckService::new(storage);
            checker.run().await?;
        }

        _ => {
            eprintln!(
                "Unknown mode: {}\nUsage: <binary> [scraper|checker]",
                mode
            );
            std::process::exit(1);
        }
    }

    Ok(())
}