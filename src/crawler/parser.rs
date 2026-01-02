use scraper::{Html, Selector};
use std::collections::HashSet;
use crate::crawler::models::{HouseDetails, PriceHistory};
use std::collections::HashMap;
use regex::Regex;

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

pub fn scrape_house_details(html: &str) -> HouseDetails {
    let doc = Html::parse_document(html);

    // Take ALL visible text and normalize into non-empty tokens.
    let body_sel = Selector::parse("body").unwrap();
    let body = doc.select(&body_sel).next();

    let mut tokens: Vec<String> = Vec::new();
    if let Some(b) = body {
        for t in b.text() {
            let s = t.trim();
            if !s.is_empty() {
                tokens.push(s.to_string());
            }
        }
    }

    // Helper: get value after a label (label -> next non-empty token)
    let next_after = |label: &str| -> Option<String> {
        let i = tokens.iter().position(|t| t == label)?;
        tokens.get(i + 1).cloned()
    };

    // Helper: section tokens between two headings (exclusive)
    let section_between = |start: &str, end: &str| -> Vec<String> {
        let si = match tokens.iter().position(|t| t == start) {
            Some(v) => v + 1,
            None => return vec![],
        };
        let ei = match tokens.iter().position(|t| t == end) {
            Some(v) => v,
            None => tokens.len(),
        };
        tokens[si..ei].to_vec()
    };

    // Simple parsers
    let parse_u8 = |v: Option<String>| v.and_then(|x| x.parse::<u8>().ok());
    let parse_m2 = |v: Option<String>| {
        v.and_then(|x| {
            let cleaned = x.replace("sq.m.", "").replace("ք.մ", "").trim().to_string();
            cleaned.parse::<f32>().ok()
        })
    };
    let split_csv = |v: Option<String>| -> Vec<String> {
        v.map(|x| {
            x.split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect()
        })
        .unwrap_or_default()
    };

    // -------- Description (multi-line) --------
    // From "Description" up to "Location"
    let desc_tokens = section_between("Description", "Location");
    let description = if desc_tokens.is_empty() {
        String::new()
    } else {
        desc_tokens.join("\n").trim().to_string()
    };

    // -------- Location --------
    let location = next_after("Location");

    // -------- Price History --------
    // Tokens between "Price History" and "Description"
    let price_tokens = section_between("Price History", "Description");

    // Typical rows look like:
    // "December 07, 2025€228,000"
    // "September 24, 2025$265,000-$10,000 ▼"
    // We'll parse date + price + optional diff
    let re = Regex::new(r"^(?P<date>[A-Za-z]+\s+\d{2},\s+\d{4})(?P<rest>.*)$").unwrap();

    let mut price_history: Vec<PriceHistory> = Vec::new();
    for tok in price_tokens {
        let tok = tok.replace('\u{00A0}', " ").trim().to_string(); // normalize NBSP
        if let Some(caps) = re.captures(&tok) {
            let date = caps["date"].trim().to_string();
            let rest = caps["rest"].trim().to_string();

            // Split rest into "price" and optional "diff"
            // Find first '-' or '+' AFTER the price currency chunk (if exists)
            // We'll treat everything up to the first " -" / " +" as price.
            let (price, diff) = if let Some(idx) = rest.find(" -") {
                (rest[..idx].trim().to_string(), Some(rest[idx..].trim().to_string()))
            } else if let Some(idx) = rest.find(" +") {
                (rest[..idx].trim().to_string(), Some(rest[idx..].trim().to_string()))
            } else if rest.contains("-$") || rest.contains("-€") || rest.contains("-֏") {
                // sometimes diff is glued like "$265,000-$10,000"
                // split on the second currency occurrence with '-' before it
                let cut = rest.find("-").unwrap_or(rest.len());
                (rest[..cut].trim().to_string(), Some(rest[cut..].trim().to_string()))
            } else {
                (rest.trim().to_string(), None)
            };

            if !price.is_empty() {
                price_history.push(PriceHistory { date, price, diff });
            }
        }
    }

    // -------- Build final struct (label -> next token) --------
    HouseDetails {
        condition: next_after("Condition"),
        rooms: parse_u8(next_after("Number of Rooms")),
        house_area_m2: parse_m2(next_after("House Area")),
        construction_type: next_after("Construction Type"),
        floors: parse_u8(next_after("Floors in the Building")),
        bathrooms: parse_u8(next_after("Number of Bathrooms")),
        garage: next_after("Garage"),
        renovation: next_after("Renovation"),
        appliances: split_csv(next_after("Appliances")),
        service_lines: split_csv(next_after("Service Lines")),
        facilities: split_csv(next_after("Facilities")),
        furniture: next_after("Furniture"),
        land_area_m2: parse_m2(next_after("Land Area")),
        price_history,
        description,
        location,
    }
}

fn split_list(value: Option<&String>) -> Vec<String> {
    value
        .map(|v| {
            v.split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect()
        })
        .unwrap_or_default()
}

