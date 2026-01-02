use reqwest::Client;

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
