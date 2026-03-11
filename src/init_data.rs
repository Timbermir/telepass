use crate::chat::{Chat, ChatType};
use crate::error::TelepassError;
use crate::sign::sign;
use crate::user::User;
use hmac::Mac;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::collections::HashSet;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InitData {
    #[serde(rename = "auth_date")]
    pub authentication_date_raw: u64,
    #[serde(
        rename = "can_send_after",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub seconds_to_send_after_raw: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chat: Option<Chat>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chat_type: Option<ChatType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chat_instance: Option<i64>,
    pub hash: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub receiver: Option<User>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_param: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
}

impl InitData {
    pub fn validate(
        init_data: &str,
        telegram_bot_token: &str,
        expires_in: Duration,
    ) -> Result<String, TelepassError> {
        let params: Vec<(String, String)> =
            serde_urlencoded::from_str(init_data).map_err(|_| TelepassError::UnexpectedFormat)?;

        let mut hash = String::new();
        let mut auth_date: Option<u64> = None;
        let mut pairs: Vec<String> = Vec::new();
        for (k, v) in &params {
            if k == "hash" {
                hash = v.clone();
                continue;
            }
            if k == "auth_date" {
                auth_date = Some(
                    v.parse::<u64>()
                        .map_err(|_| TelepassError::AuthDateInvalid)?,
                );
            }
            pairs.push(format!("{}={}", k, v));
        }

        if hash.is_empty() {
            return Err(TelepassError::SignMissing);
        }

        let date = auth_date.ok_or(TelepassError::AuthDateMissing)?;
        let auth_time = UNIX_EPOCH + Duration::from_secs(date);
        let expires_at = auth_time + expires_in;
        if SystemTime::now() > expires_at {
            return Err(TelepassError::Expired);
        }

        pairs.sort();

        if sign(&pairs.join("\n"), telegram_bot_token) != hash {
            return Err(TelepassError::SignInvalid);
        }

        Ok(init_data.to_string())
    }

    pub fn parse(init_data: &str) -> Result<InitData, TelepassError> {
        let params: Vec<(String, String)> =
            serde_urlencoded::from_str(init_data).map_err(|_| TelepassError::UnexpectedFormat)?;

        let string_props = HashSet::from(["hash", "start_param"]);

        let mut map = Map::new();

        for (key, val) in params {
            let json_val = if string_props.contains(key.as_str()) {
                Value::String(val)
            } else {
                serde_json::from_str::<Value>(&val).unwrap_or(Value::String(val))
            };
            map.insert(key, json_val);
        }

        serde_json::from_value(Value::Object(map)).map_err(|_| TelepassError::UnexpectedFormat)
    }

    pub fn get_auth_date(&self) -> SystemTime {
        UNIX_EPOCH + Duration::from_secs(self.authentication_date_raw)
    }

    pub fn get_can_send_after(&self) -> Option<SystemTime> {
        self.seconds_to_send_after_raw
            .map(|secs| self.get_auth_date() + Duration::from_secs(secs))
    }
}

pub struct InitDataRequest {
    pub raw: String,
    pub token: String,
}
