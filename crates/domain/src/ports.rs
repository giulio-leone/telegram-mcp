use anyhow::Result;

/// Port trait for Telegram operations — implemented by tg-client
pub trait TelegramPort: Send + Sync {
    fn send_message(&self, chat_id: i64, text: &str) -> impl std::future::Future<Output = Result<i32>> + Send;
    fn is_connected(&self) -> bool;
}
