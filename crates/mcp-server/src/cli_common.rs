use anyhow::Result;
use tg_client::config::TelegramConfig;
use tg_client::TelegramClient;
use tracing_subscriber::EnvFilter;

/// Initialize tracing with RUST_LOG support
pub fn init_tracing() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .compact()
        .init();
}

/// Create a TelegramClient from env vars
pub async fn create_client() -> Result<TelegramClient> {
    let config = TelegramConfig::from_env()?;
    TelegramClient::new(config).await
}

/// Apply stealth flag from --stealth CLI arg or TG_STEALTH=1 env var
pub fn apply_stealth_flag(client: &TelegramClient, args: &[String]) {
    let from_flag = args.iter().any(|a| a == "--stealth" || a == "-s");
    let from_env = std::env::var("TG_STEALTH")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false);

    if from_flag || from_env {
        client.set_stealth(true);
    }
}

/// Check if --fresh flag is present
pub fn has_flag(args: &[String], flag: &str) -> bool {
    args.iter().any(|a| a == flag)
}
