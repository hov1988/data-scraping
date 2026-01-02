mod config;
mod crawler;
mod storage;
mod scheduler;

use config::Config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let cfg = Config::from_env()?;

    // 1️⃣ Crawl listing pages (links only)
    let mut links: Vec<String> = crawler::crawl_first_pages(&cfg)
        .await?
        .into_iter()
        .collect();

    links.sort();

    println!("\n==============================");
    println!("TOTAL ITEMS FOUND: {}", links.len());
    println!("==============================\n");

    // Optional: print links
    for link in &links {
        println!("{}", link);
    }

    println!("\nFetching details for items...\n");

    // 2️⃣ Crawl details (RETURNS DATA)
    let details = crawler::crawl_details(&links).await?;

    // 3️⃣ Print first 1–2 items as sample
    for item in details.iter().take(2) {
        println!("========== ITEM DETAILS ==========");
        println!("{:#?}", item);
    }

    Ok(())
}
