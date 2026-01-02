mod config;
mod crawler;
mod storage;
mod scheduler;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    // Single item link
    let link = "https://www.list.am/en/item/22137770?ld_src=2".to_string();

    println!("Fetching details for single item...\n");

    let details = crawler::crawl_details(&vec![link]).await?;

    // Print result
    if let Some(item) = details.first() {
        println!("========== ITEM DETAILS ==========");
        println!("{:#?}", item);
    } else {
        println!("No data extracted");
    }

    Ok(())
}
