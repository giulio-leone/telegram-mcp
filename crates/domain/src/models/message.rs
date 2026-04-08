use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: i32,
    pub chat_id: i64,
    pub text: Option<String>,
    pub sender_name: Option<String>,
    pub date: chrono::DateTime<chrono::Utc>,
    pub outgoing: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendResult {
    pub message_id: i32,
    pub chat_id: i64,
    pub success: bool,
}
