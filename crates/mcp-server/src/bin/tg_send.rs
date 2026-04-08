use anyhow::Result;
use tg_mcp_server::cli_common;

#[tokio::main]
async fn main() -> Result<()> {
    cli_common::init_tracing();

    let args: Vec<String> = std::env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: tg-send [--stealth] <phone_or_username> <message>");
        eprintln!();
        eprintln!("  phone_or_username: Phone number (+393661410914) or @username");
        eprintln!("  message:           Text message to send");
        eprintln!();
        eprintln!("Options:");
        eprintln!("  --stealth, -s      Suppress read receipts and online status");
        eprintln!();
        eprintln!("Environment:");
        eprintln!("  TG_API_ID          Telegram API ID (from my.telegram.org)");
        eprintln!("  TG_API_HASH        Telegram API hash");
        eprintln!("  TG_STEALTH=1       Enable stealth mode via env");
        std::process::exit(1);
    }

    let client = cli_common::create_client().await?;
    cli_common::apply_stealth_flag(&client, &args);

    if !client.is_authorized().await? {
        anyhow::bail!("Not logged in. Run tg-pair first.");
    }

    // Parse args: skip flags to find recipient and message
    let non_flag_args: Vec<&str> = args[1..]
        .iter()
        .filter(|a| !a.starts_with('-'))
        .map(|a| a.as_str())
        .collect();

    if non_flag_args.len() < 2 {
        anyhow::bail!("Need at least <recipient> and <message>");
    }

    let recipient = non_flag_args[0];
    let message = non_flag_args[1..].join(" ");

    let msg_id = if recipient.starts_with('@') {
        client
            .send_message_to(&recipient[1..], &message)
            .await?
    } else if recipient.starts_with('+') || recipient.chars().all(|c| c.is_ascii_digit()) {
        client
            .send_message_by_phone(recipient, &message)
            .await?
    } else {
        // Try as username
        client.send_message_to(recipient, &message).await?
    };

    println!("✅ Message sent (id={})", msg_id);
    Ok(())
}
