use std::ops::Add;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CallbackData {
    pub code: String,
}

#[derive(Serialize)]
pub struct AccessTokenExchange {
    pub client_id: String,
    pub client_secret: String,
    pub grant_type: String,
    pub code: String,
    pub redirect_uri: String,
    pub scope: String,
}

#[derive(Serialize)]
pub struct AccessTokenRefresh {
    pub client_id: String,
    pub client_secret: String,
    pub grant_type: String,
    pub refresh_token: String,
    pub redirect_uri: String,
    pub scope: String,
}

#[derive(Deserialize)]
pub struct AccessTokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i32,
    pub refresh_token: String,
    pub scope: String,
}

#[derive(Serialize)]
pub struct AccessTokenFormData {
    pub access_token: String,
}

#[derive(Serialize, Deserialize)]
pub struct AuthSession {
    pub access_token: String,
    pub refresh_token: String,
    pub expire_time: u64,
}

impl AuthSession {
    pub fn from_access_token_response(response: AccessTokenResponse) -> AuthSession {
        AuthSession {
            access_token: response.access_token,
            refresh_token: response.refresh_token,
            expire_time: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .add(Duration::from_secs(response.expires_in as u64))
                .as_secs(),
        }
    }
}
