use anyhow::Result;
use tg_mcp_server::cli_common;

#[tokio::main]
async fn main() -> Result<()> {
    cli_common::init_tracing();
    // TODO: Implement online presence tracking via Telegram API
    // Telegram's presence API is more restricted than WhatsApp's —
    // users must have "last seen" enabled for you to see their status.
    eprintln!("tg-track: Not yet implemented. Coming soon.");
    Ok(())
}
