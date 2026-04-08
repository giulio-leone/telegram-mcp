use anyhow::Result;
use std::time::Duration;
use tg_mcp_server::cli_common;

#[tokio::main]
async fn main() -> Result<()> {
    cli_common::init_tracing();

    let args: Vec<String> = std::env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: tg-broadcast [--stealth] [--delay <ms>] <message> <recipient1> [recipient2] ...");
        eprintln!();
        eprintln!("  message:       Text message to broadcast");
        eprintln!("  recipients:    Phone numbers or @usernames");
        eprintln!();
        eprintln!("Options:");
        eprintln!("  --delay <ms>   Delay between sends (default: 1000ms)");
        eprintln!("  --stealth, -s  Suppress read receipts and online status");
        std::process::exit(1);
    }

    let client = cli_common::create_client().await?;
    cli_common::apply_stealth_flag(&client, &args);

    if !client.is_authorized().await? {
        anyhow::bail!("Not logged in. Run tg-pair first.");
    }

    // Parse delay
    let mut delay_ms: u64 = 1000;
    let mut skip_next = false;
    let mut non_flag_args: Vec<String> = Vec::new();

    for (i, arg) in args[1..].iter().enumerate() {
        if skip_next {
            skip_next = false;
            continue;
        }
        if arg == "--delay" {
            if let Some(val) = args.get(i + 2) {
                delay_ms = val.parse().unwrap_or(1000);
                skip_next = true;
            }
            continue;
        }
        if arg.starts_with('-') {
            continue;
        }
        non_flag_args.push(arg.clone());
    }

    if non_flag_args.len() < 2 {
        anyhow::bail!("Need at least <message> and one <recipient>");
    }

    let message = &non_flag_args[0];
    let recipients = &non_flag_args[1..];

    println!(
        "📤 Broadcasting to {} recipients (delay={}ms)",
        recipients.len(),
        delay_ms
    );

    let mut success = 0;
    let mut failures: Vec<(String, String)> = Vec::new();

    for (i, recipient) in recipients.iter().enumerate() {
        let result = if recipient.starts_with('@') {
            client.send_message_to(&recipient[1..], message).await
        } else if recipient.starts_with('+') || recipient.chars().all(|c| c.is_ascii_digit()) {
            client.send_message_by_phone(recipient, message).await
        } else {
            client.send_message_to(recipient, message).await
        };

        match result {
            Ok(id) => {
                success += 1;
                println!("  ✅ {}/{} → {} (id={})", i + 1, recipients.len(), recipient, id);
            }
            Err(e) => {
                failures.push((recipient.clone(), e.to_string()));
                println!("  ❌ {}/{} → {} — {}", i + 1, recipients.len(), recipient, e);
            }
        }

        if i < recipients.len() - 1 {
            tokio::time::sleep(Duration::from_millis(delay_ms)).await;
        }
    }

    println!();
    println!("═══ Broadcast complete: {}/{} sent ═══", success, recipients.len());
    if !failures.is_empty() {
        println!("Failed:");
        for (r, e) in &failures {
            println!("  {} — {}", r, e);
        }
    }

    Ok(())
}
