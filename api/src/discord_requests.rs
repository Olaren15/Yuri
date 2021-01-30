use actix_web::{client::Client, http::header};
use serde::de::DeserializeOwned;

use crate::scopes::auth::models::AuthSession;

pub struct DiscordRequest;

impl DiscordRequest {
    pub const API_BASE_URI: &'static str = "https://discord.com/api/v8";
    pub const CDN_BASE_URI: &'static str = "https://cdn.discordapp.com";

    pub async fn get<T: DeserializeOwned>(path: &str, auth: &AuthSession) -> Option<T> {
        match Client::new()
            .get(format!("{}{}", Self::API_BASE_URI, path))
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", auth.access_token),
            )
            .send()
            .await
        {
            Ok(mut request) => match request.json::<T>().await {
                Ok(data) => Some(data),
                Err(e) => {
                    println!("Failed to parse json: {}", e);
                    None
                }
            },
            Err(e) => {
                println!("Failed to contact discord servers: {}", e);
                None
            }
        }
    }
}
