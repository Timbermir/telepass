use serde;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    pub first_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(rename = "photo_url", default, skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_premium: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_bot: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub added_to_attachment_menu: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allows_write_to_pm: Option<bool>,
}
