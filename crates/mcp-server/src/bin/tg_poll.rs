use anyhow::Result;
use tg_mcp_server::cli_common;

#[tokio::main]
async fn main() -> Result<()> {
    cli_common::init_tracing();
    // TODO: Implement event polling daemon with YAML config
    // Will mirror wa-poll architecture with event filters and actions
    eprintln!("tg-poll: Not yet implemented. Coming soon.");
    Ok(())
}
