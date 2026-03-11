use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ChatType {
    Sender,
    Private,
    Group,
    Supergroup,
    Channel,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Chat {
    pub id: i64,
    #[serde(rename = "type")]
    pub chat_type: ChatType,
    pub title: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(rename = "photo_url", default, skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
}
