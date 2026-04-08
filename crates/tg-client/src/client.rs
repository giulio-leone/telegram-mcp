use crate::config::TelegramConfig;
use anyhow::{Context, Result};
use grammers_client::Client;
use grammers_mtsender::SenderPool;
use grammers_session::storages::SqliteSession;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::info;

pub struct TelegramClient {
    client: Client,
    config: TelegramConfig,
    stealth: Arc<AtomicBool>,
    _pool_handle: tokio::task::JoinHandle<()>,
    updates_rx: Option<mpsc::UnboundedReceiver<grammers_session::updates::UpdatesLike>>,
}

impl TelegramClient {
    pub async fn new(config: TelegramConfig) -> Result<Self> {
        crate::config::ensure_data_dir()?;

        let session = Arc::new(
            SqliteSession::open(config.session_path.to_str().unwrap_or("telegram.session"))
                .await
                .context("Failed to open session file")?,
        );

        let SenderPool {
            runner,
            handle,
            updates,
        } = SenderPool::new(Arc::clone(&session), config.api_id);

        let client = Client::new(handle);

        let pool_handle = tokio::spawn(async move {
            runner.run().await;
        });

        Ok(Self {
            client,
            config,
            stealth: Arc::new(AtomicBool::new(false)),
            _pool_handle: pool_handle,
            updates_rx: Some(updates),
        })
    }

    pub async fn is_authorized(&self) -> Result<bool> {
        self.client
            .is_authorized()
            .await
            .context("Failed to check authorization")
    }

    pub fn inner(&self) -> &Client {
        &self.client
    }

    pub fn api_hash(&self) -> &str {
        &self.config.api_hash
    }

    pub fn take_updates(
        &mut self,
    ) -> Option<mpsc::UnboundedReceiver<grammers_session::updates::UpdatesLike>> {
        self.updates_rx.take()
    }

    // --- Stealth mode ---

    pub fn set_stealth(&self, enabled: bool) {
        self.stealth.store(enabled, Ordering::Relaxed);
        info!("Stealth mode: {}", if enabled { "ON" } else { "OFF" });
    }

    pub fn is_stealth(&self) -> bool {
        self.stealth.load(Ordering::Relaxed)
    }

    // --- Messaging ---

    /// Send a text message by @username
    pub async fn send_message_to(&self, username: &str, text: &str) -> Result<i32> {
        let peer = self
            .client
            .resolve_username(username)
            .await
            .context("Failed to resolve username")?
            .ok_or_else(|| anyhow::anyhow!("User/chat '@{}' not found", username))?;

        let peer_ref = peer
            .to_ref()
            .await
            .ok_or_else(|| anyhow::anyhow!("Cannot get reference for '@{}'", username))?;

        let sent = self
            .client
            .send_message(peer_ref, text)
            .await
            .context("Failed to send message")?;

        info!("Message sent to @{} (id={})", username, sent.id());
        Ok(sent.id())
    }

    /// Send a text message by phone number — searches contacts list
    pub async fn send_message_by_phone(&self, phone: &str, text: &str) -> Result<i32> {
        // Iterate dialogs to find a user matching the phone number
        // This is the simplest approach that doesn't require raw TL API
        let mut dialogs = self.client.iter_dialogs();
        let normalized = phone.replace('+', "").replace(' ', "").replace('-', "");

        while let Some(dialog) = dialogs.next().await? {
            let peer = dialog.peer();
            if let grammers_client::peer::Peer::User(ref user) = peer {
                if let Some(user_phone) = user.phone() {
                    let user_normalized = user_phone.replace('+', "").replace(' ', "").replace('-', "");
                    if user_normalized == normalized {
                        let peer_ref = peer
                            .to_ref()
                            .await
                            .ok_or_else(|| anyhow::anyhow!("Cannot get reference for phone {}", phone))?;

                        let sent = self
                            .client
                            .send_message(peer_ref, text)
                            .await
                            .context("Failed to send message")?;

                        info!("Message sent to {} (id={})", phone, sent.id());
                        return Ok(sent.id());
                    }
                }
            }
        }

        anyhow::bail!(
            "Phone {} not found in your conversations. Start a chat with them first or use @username.",
            phone
        )
    }
}
