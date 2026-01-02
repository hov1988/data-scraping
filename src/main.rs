mod config;
mod crawler;
mod storage;
mod scheduler;

use config::Config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let cfg = Config::from_env()?;

    // -------- STEP 1: crawl category pages --------
    let mut links: Vec<String> = crawler::crawl_first_pages(&cfg)
        .await?
        .into_iter()
        .collect();

    links.sort();

    println!("\n==============================");
    println!("TOTAL ITEMS FOUND: {}", links.len());
    println!("==============================\n");

    // -------- STEP 2: take first 1–2 links --------
    let sample_links: Vec<String> = links.into_iter().take(2).collect();

    println!("Fetching details for {} items...\n", sample_links.len());

    // -------- STEP 3: fetch & parse details --------
    let details = crawler::crawl_details(&sample_links).await?;

    // -------- STEP 4: print results --------
    for (idx, item) in details.iter().enumerate() {
        println!("========== ITEM {} ==========", idx + 1);
        println!("{:#?}", item);
        println!();
    }

    Ok(())
}
