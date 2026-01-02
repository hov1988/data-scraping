use std::env;

pub struct Config {
    pub base_url: String,
    pub start_page: u32,
    pub end_page: u32,
    pub delay_ms: u64,
    pub database_url: String,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Self {
            base_url: env::var("LISTAM_BASE_URL")?,
            start_page: env::var("START_PAGE")?.parse()?,
            end_page: env::var("END_PAGE")?.parse()?,
            delay_ms: env::var("DELAY_MS")?.parse()?,
            database_url: env::var("DATABASE_URL")?,
        })
    }
}
