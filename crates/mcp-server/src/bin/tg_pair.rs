use anyhow::{Context, Result};
use grammers_client::SignInError;
use std::io::{self, BufRead, Write};
use tg_mcp_server::cli_common;

fn prompt(message: &str) -> io::Result<String> {
    print!("{}", message);
    io::stdout().flush()?;
    let mut line = String::new();
    io::stdin().lock().read_line(&mut line)?;
    Ok(line.trim().to_string())
}

#[tokio::main]
async fn main() -> Result<()> {
    cli_common::init_tracing();

    println!("═══════════════════════════════════════════");
    println!("  Telegram MCP — Pairing / Login");
    println!("═══════════════════════════════════════════");
    println!();
    println!("Ensure TG_API_ID and TG_API_HASH are set.");
    println!("Get them from: https://my.telegram.org");
    println!();

    let client = cli_common::create_client().await?;

    if client.is_authorized().await? {
        println!("✅ Already logged in!");
        println!("Session file is valid. You can use tg-send, tg-broadcast, etc.");
        return Ok(());
    }

    let phone = prompt("📱 Enter phone number (international format, e.g. +393661410914): ")?;

    let token = client
        .inner()
        .request_login_code(&phone, client.api_hash())
        .await
        .context("Failed to request login code — check API credentials")?;

    let code = prompt("🔑 Enter the code you received: ")?;

    match client.inner().sign_in(&token, &code).await {
        Ok(user) => {
            println!();
            println!("✅ Signed in as: {}", user.first_name().unwrap_or("user"));
            if let Some(username) = user.username() {
                println!("   Username: @{}", username);
            }
            println!("   Session saved — you can now use all tg-* commands.");
        }
        Err(SignInError::PasswordRequired(password_token)) => {
            let hint = password_token.hint().unwrap_or("none");
            let password = prompt(&format!("🔒 2FA required (hint: {}): ", hint))?;

            let user = client
                .inner()
                .check_password(password_token, &password)
                .await
                .context("2FA check failed")?;

            println!();
            println!(
                "✅ Signed in with 2FA as: {}",
                user.first_name().unwrap_or("user")
            );
            println!("   Session saved.");
        }
        Err(SignInError::InvalidCode) => {
            anyhow::bail!("❌ Invalid code. Please try again.");
        }
        Err(SignInError::SignUpRequired) => {
            anyhow::bail!("❌ This phone number is not registered on Telegram. Sign up on the official app first.");
        }
        Err(e) => return Err(e.into()),
    }

    Ok(())
}
