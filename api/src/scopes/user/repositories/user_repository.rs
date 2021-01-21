use actix_web::{client::Client, http::header};

use crate::scopes::user::models::user::{DiscordUser, YuriUser};

pub struct UserRepository {
    discord_token: String,
}

impl UserRepository {
    pub fn new(discord_token: String) -> UserRepository {
        UserRepository { discord_token }
    }

    pub async fn get_yuri_user(&self) -> Option<YuriUser> {
        let discord_user = Client::new()
            .get("https://discord.com/api/v8/users/@me")
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", self.discord_token),
            )
            .send()
            .await
            .ok()?
            .json::<DiscordUser>()
            .await
            .ok()?;

        Some(YuriUser {
            username: discord_user.username,
            discriminator: discord_user.discriminator.parse().ok()?,
            // TODO: complete this
            icon_url: "".to_string(),
            servers: vec![],
        })
    }
}
