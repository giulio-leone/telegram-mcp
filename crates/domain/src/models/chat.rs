use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chat {
    pub id: i64,
    pub name: String,
    pub chat_type: ChatType,
    pub username: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChatType {
    Private,
    Group,
    Supergroup,
    Channel,
}
