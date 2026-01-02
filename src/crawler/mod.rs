use std::collections::HashSet;
use tokio::time::{sleep, Duration};

use crate::config::Config;

mod fetcher;
mod parser;
mod models;

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
