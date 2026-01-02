use scraper::{Html, Selector};
use std::collections::HashSet;
use crate::crawler::models::{HouseDetails, PriceHistory};
use regex::Regex;
use crate::crawler::models::ContactInfo;
use chrono::{DateTime, NaiveDateTime, Utc, NaiveDate};

fn normalize_price_date(raw: &str) -> Option<String> {
    // Example: "December 07, 2025"
    NaiveDate::parse_from_str(raw, "%B %d, %Y")
        .ok()
        .map(|d| {
            let dt = d.and_hms_opt(0, 0, 0)?;
            let utc: DateTime<Utc> = DateTime::from_utc(dt, Utc);
            Some(utc.to_rfc3339())
        })
        .flatten()
}

pub fn parse_price_history_iso(html: &str) -> Vec<PriceHistory> {
    let doc = Html::parse_document(html);

    let row_sel = Selector::parse(".price_history table tbody tr").unwrap();
    let cell_sel = Selector::parse("td").unwrap();

    let mut result = Vec::new();

    for row in doc.select(&row_sel) {
        let cells: Vec<String> = row
            .select(&cell_sel)
            .map(|c| c.text().collect::<String>().replace('\u{00A0}', " ").trim().to_string())
            .collect();

        if cells.len() < 2 {
            continue;
        }

        let date_iso = normalize_price_date(&cells[0])
            .unwrap_or_else(|| cells[0].clone());

        let price = cells[1].clone();

        let diff = cells.get(2).and_then(|v| {
            let t = v.trim();
            if t.is_empty() {
                None
            } else {
                Some(t.to_string())
            }
        });

        result.push(PriceHistory {
            date: date_iso,
            price,
            diff,
        });
    }

    result
}


pub fn parse_created_updated_iso(
    html: &str,
) -> (Option<String>, Option<String>) {
    let doc = Html::parse_document(html);

    // -------- CREATED AT (already ISO) --------
    let created_at = Selector::parse(r#"span[itemprop="datePosted"]"#)
        .ok()
        .and_then(|sel| doc.select(&sel).next())
        .and_then(|el| el.value().attr("content"))
        .map(|v| v.to_string());

    // -------- UPDATED AT (normalize) --------
    // "Renewed 02.01.2026, 13:23"
    let updated_at = Selector::parse(".footer span")
        .ok()
        .and_then(|sel| {
            doc.select(&sel)
                .filter_map(|el| {
                    let text = el.text().collect::<String>().trim().to_string();
                    text.strip_prefix("Renewed ")
                        .map(|v| v.to_string())
                })
                .next()
        })
        .and_then(|raw| {
            // raw = "02.01.2026, 13:23"
            let raw = raw.replace(',', "");
            NaiveDateTime::parse_from_str(&raw, "%d.%m.%Y %H:%M")
                .ok()
                .map(|dt| {
                    let utc: DateTime<Utc> = DateTime::from_utc(dt, Utc);
                    utc.to_rfc3339()
                })
        });

    (created_at, updated_at)
}


pub fn parse_image_urls(html: &str) -> Vec<String> {
    let doc = Html::parse_document(html);
    let img_sel = Selector::parse("img").unwrap();

    let mut urls: Vec<String> = Vec::new();

    // 1️⃣ Find FIRST image only
    for img in doc.select(&img_sel) {
        let attrs = img.value();

        let src = attrs
            .attr("src")
            .or_else(|| attrs.attr("data-src"))
            .or_else(|| attrs.attr("data-original"));

        if let Some(src) = src {
            if src.contains("s.list.am") {
                let cleaned = src
                    .trim()
                    .trim_start_matches("//")
                    .to_string();

                urls.push(cleaned);
                break; // 🔴 IMPORTANT: stop after first image
            }
        }
    }

    // If no image at all → return empty
    let first = match urls.first() {
        Some(v) => v.clone(),
        None => return vec![],
    };

    // 2️⃣ Detect image count from gallery divs
    let gallery_sel = Selector::parse("div.p").unwrap();
    let item_sel = Selector::parse("div.p > div").unwrap();

    let gallery = match doc.select(&gallery_sel).next() {
        Some(v) => v,
        None => return vec![first],
    };

    let gallery_width = gallery
        .value()
        .attr("style")
        .and_then(extract_width_px)
        .unwrap_or(0);

    let item_width = gallery
        .select(&item_sel)
        .next()
        .and_then(|n| n.value().attr("style"))
        .and_then(extract_width_px)
        .unwrap_or(0);

    if gallery_width == 0 || item_width == 0 {
        return vec![first];
    }

    let image_count = (gallery_width / item_width) as usize;

    // 3️⃣ Generate sequential URLs
    let (base, number_part) = match first.rsplit_once('/') {
        Some(v) => v,
        None => return vec![first],
    };

    let start_number: u64 = match number_part
        .trim_end_matches(".webp")
        .parse()
    {
        Ok(v) => v,
        Err(_) => return vec![first],
    };

    let mut generated = Vec::new();
    for i in 0..image_count {
        generated.push(format!("{}/{}.webp", base, start_number + i as u64));
    }

    generated
}


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
    let desc_tokens = section_between("Description", "Location");
    let description = if desc_tokens.is_empty() {
        String::new()
    } else {
        desc_tokens.join("\n").trim().to_string()
    };

    // -------- Location --------
    let location = next_after("Location");

    // -------- Price History --------
    let price_tokens = section_between("Price History", "Description");

    let re = Regex::new(r"^(?P<date>[A-Za-z]+\s+\d{2},\s+\d{4})(?P<rest>.*)$").unwrap();

        // -------- TITLE --------
    let title = Selector::parse(r#"h1[itemprop="name"]"#)
    .ok()
    .and_then(|s| doc.select(&s).next())
    .map(|e| e.text().collect::<String>().trim().to_string());

    // -------- PRICE --------
    let price = Selector::parse("#abar")
    .ok()
    .and_then(|s| doc.select(&s).next())
    .map(|e| e.text().collect::<String>().trim().to_string());

    // -------- SELLER NAME (HTML) --------
    let seller_name = Selector::parse(".user-name, .seller-name")
    .ok()
    .and_then(|s| doc.select(&s).next())
    .map(|e| e.text().collect::<String>().trim().to_string());

    // -------- PRICE HISTORY (HTML) --------
    let price_history = parse_price_history_iso(html);

    // -------- CREATED AT & UPDATED AT (HTML) --------
    let (created_at, updated_at) = parse_created_updated_iso(html);

    // -------- Build final struct (label -> next token) --------
    HouseDetails {
        title,
        price,
        contact: ContactInfo {
            seller_name,
            phone_display: None,
            phone_raw: None,
        },
        price_history,
        images: vec![],
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
        description,
        location,
        created_at,
        updated_at,
    }
}


pub fn parse_contact_from_popup(html: &str) -> ContactInfo {
    let doc = Html::parse_fragment(html);

    // Seller name: <span class="nmsp">Grigor</span>
    let seller_name = Selector::parse("span.nmsp")
        .ok()
        .and_then(|sel| doc.select(&sel).next())
        .map(|e| e.text().collect::<String>().trim().to_string());

    // Display phone: <div class="phone-number-section">(099) 09-91-11</div>
    let phone_display = Selector::parse(".phone-number-section")
        .ok()
        .and_then(|sel| doc.select(&sel).next())
        .map(|e| e.text().collect::<String>().trim().to_string());

    // Raw phone: <a href="tel:099099111" class="phone-number">
    let phone_raw = Selector::parse("a.phone-number")
        .ok()
        .and_then(|sel| doc.select(&sel).next())
        .and_then(|e| e.value().attr("href"))
        .map(|href| href.replace("tel:", "").trim().to_string());

    ContactInfo {
        seller_name,
        phone_display,
        phone_raw,
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

fn extract_width_px(style: &str) -> Option<u64> {
    // style="width: 787px; height: 280px;"
    style
        .split(';')
        .find(|s| s.contains("width"))
        .and_then(|w| w.split(':').nth(1))
        .and_then(|v| v.trim().trim_end_matches("px").parse().ok())
}

