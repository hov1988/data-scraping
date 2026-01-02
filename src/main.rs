mod config;
mod crawler;
mod storage;
mod scheduler;

use config::Config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let cfg = Config::from_env()?;

    let mut links: Vec<String> = crawler::crawl_first_pages(&cfg)
        .await?
        .into_iter()
        .collect();

    links.sort();

    println!("\n==============================");
    println!("TOTAL ITEMS FOUND: {}", links.len());
    println!("==============================\n");

    for link in &links {
        println!("{}", link);
    }

    Ok(())
}
