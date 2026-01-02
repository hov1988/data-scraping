use std::collections::HashSet;
use tokio::time::{sleep, Duration};
use crate::crawler::models::HouseDetails;
use crate::config::Config;

mod fetcher;
mod parser;
mod models;

pub async fn crawl_details(
    links: &[String],
) -> anyhow::Result<Vec<HouseDetails>> {
    let client = fetcher::build_client();
    let mut results = Vec::new();

    for link in links {
        let external_id = link
            .split("/item/")
            .nth(1)
            .unwrap()
            .split('?')
            .next()
            .unwrap()
            .to_string();

        println!("Fetching detail page for item {}", external_id);

        let html = fetcher::fetch_html(&client, link).await?;

        let details = parser::scrape_house_details(&html);

        results.push(details);

        // polite delay
        sleep(Duration::from_millis(300)).await;
    }

    Ok(results)
}

pub async fn crawl_first_pages(cfg: &Config) -> anyhow::Result<HashSet<String>> {
    let client = fetcher::build_client();
    let mut all_links: HashSet<String> = HashSet::new();

    for page in cfg.start_page..=cfg.end_page {
        let url = format!("{}/{}", cfg.base_url, page);
        println!("Fetching page {}", page);

        let html = fetcher::fetch_html(&client, &url).await?;
        let page_links = parser::extract_item_links(&html);

        all_links.extend(page_links);

        sleep(Duration::from_millis(cfg.delay_ms)).await;
    }

    Ok(all_links)
}
