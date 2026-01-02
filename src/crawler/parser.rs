use scraper::{Html, Selector};
use std::collections::HashSet;

pub fn extract_item_links(html: &str) -> HashSet<String> {
    let document = Html::parse_document(html);
    let selector = Selector::parse("a[href*=\"/en/item/\"]").unwrap();

    let mut links = HashSet::new();

    for el in document.select(&selector) {
        if let Some(href) = el.value().attr("href") {
            if href.starts_with("/en/item/") {
                let clean = href.split('?').next().unwrap();
                links.insert(format!("https://www.list.am{}", clean));
            }
        }
    }

    links
}
