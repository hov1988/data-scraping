use reqwest::Client;
use serde::Deserialize;
use tokio::fs;
use tokio::io::AsyncWriteExt;

#[derive(Deserialize)]
struct PhoneResponse {
    phone: String,
    name: Option<String>,
}

pub fn build_client() -> Client {
    Client::builder()
        .user_agent("ListAm-Crawler/1.0 (approved)")
        .build()
        .expect("failed to build http client")
}

pub async fn fetch_html(client: &Client, url: &str) -> anyhow::Result<String> {
    let res = client.get(url).send().await?;
    Ok(res.text().await?)
}

pub async fn fetch_phone_popup_html(
    client: &Client,
    item_id: &str,
) -> anyhow::Result<String> {
    let url = format!(
        "https://www.list.am/rtam?i={}&_rtt=1",
        item_id
    );

    let res = client.get(url).send().await?;

    // 🔎 DEBUG — keep until confirmed working
    let text = res.text().await?;
    println!("{}", text);

    Ok(text)
}

pub async fn download_images(
    client: &Client,
    image_urls: &[String],
    item_id: &str,
) -> anyhow::Result<()> {
    let dir = format!("images/{}", item_id);
    fs::create_dir_all(&dir).await?;

    for (idx, url) in image_urls.iter().enumerate() {
        // 🔹 Convert protocol-less URL to absolute URL
        let full_url = if url.starts_with("http://") || url.starts_with("https://") {
            url.clone()
        } else {
            format!("https://{}", url)
        };

        let res = match client.get(&full_url).send().await {
            Ok(r) => r,
            Err(e) => {
                eprintln!("Request failed for {}: {}", full_url, e);
                break;
            }
        };

        // 🔹 Stop when images end
        if !res.status().is_success() {
            println!("Stopping image download at {}", full_url);
            break;
        }

        let bytes = res.bytes().await?;

        let filename = format!("{}/{}.webp", dir, idx + 1);
        let mut file = fs::File::create(&filename).await?;
        file.write_all(&bytes).await?;

        println!("Saved image {}", filename);
    }

    Ok(())
}