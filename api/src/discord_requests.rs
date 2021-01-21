use std::time::{SystemTime, UNIX_EPOCH};

use actix_web::{client::Client, http::header};
use serde::de::DeserializeOwned;

use crate::scopes::auth::{
    models::o_auth::AuthSession, repositories::auth_repository::AuthRepository,
};

pub struct DiscordRequest;

impl DiscordRequest {
    pub const BASE_URI: &'static str = "https://discord.com/api/v8";

    pub async fn get<T: DeserializeOwned>(&self, path: &str, auth: &AuthSession) -> Option<T> {
        if auth.expire_time
            < SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
        {
            AuthRepository::refresh_token(auth);
        }

        Client::new()
            .get(format!("{}{}", Self::BASE_URI, path))
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", auth.access_token),
            )
            .send()
            .await
            .ok()?
            .json::<T>()
            .await
            .ok()
    }
}
