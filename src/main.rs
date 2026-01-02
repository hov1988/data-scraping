mod config;
mod crawler;
mod storage;
mod scheduler;

use config::Config;
use storage::postgres::Storage;

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

    for link in &links {
        println!("{}", link);
    }

    // 2️⃣ Crawl details (RETURNS DATA)
    println!("\nFetching details for items...\n");
    let details = crawler::crawl_details(&links).await?;

    // 3️⃣ Init storage
    let storage = Storage::new(&cfg.database_url).await?;

    // 4️⃣ Save everything to DB
    for house in &details {
        let house_id = storage.save_house(house).await?;
        println!("✅ Saved house external_id={} db_id={}",
            house.external_id,
            house_id
        );
    }

    // 5️⃣ Print first 1–2 items as sample
    println!("\n========== SAMPLE OUTPUT ==========\n");
    for item in details.iter().take(2) {
        println!("{:#?}", item);
    }

    println!("\n🎉 DONE: {} houses saved to database", details.len());

    Ok(())
}
