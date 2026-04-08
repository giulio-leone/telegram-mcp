use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelegramConfig {
    pub api_id: i32,
    pub api_hash: String,
    pub session_path: PathBuf,
}

impl TelegramConfig {
    pub fn from_env() -> anyhow::Result<Self> {
        let api_id: i32 = std::env::var("TG_API_ID")
            .map_err(|_| anyhow::anyhow!("TG_API_ID env var not set — get it from https://my.telegram.org"))?
            .parse()
            .map_err(|_| anyhow::anyhow!("TG_API_ID must be a number"))?;

        let api_hash = std::env::var("TG_API_HASH")
            .map_err(|_| anyhow::anyhow!("TG_API_HASH env var not set — get it from https://my.telegram.org"))?;

        let session_path = dirs_or_default().join("telegram.session");

        Ok(Self { api_id, api_hash, session_path })
    }
}

fn dirs_or_default() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".telegram-mcp")
}

pub fn ensure_data_dir() -> anyhow::Result<PathBuf> {
    let dir = dirs_or_default();
    std::fs::create_dir_all(&dir)?;
    Ok(dir)
}
