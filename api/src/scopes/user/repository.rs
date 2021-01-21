use crate::{
    discord_requests::DiscordRequest,
    scopes::auth::models::AuthSession,
    scopes::user::models::{DiscordUser, YuriUser},
};

pub struct UserRepository;

impl UserRepository {
    pub async fn get_yuri_user(auth: &AuthSession) -> Option<YuriUser> {
        if let Some(discord_user) = DiscordRequest::get::<DiscordUser>("/users/@me", auth).await {
            Some(YuriUser {
                username: discord_user.username,
                discriminator: discord_user.discriminator.parse().ok()?,
                icon_url: if let Some(avatar) = discord_user.avatar {
                    format!(
                        "{}/avatars/{}/{}.png",
                        DiscordRequest::CDN_BASE_URI,
                        discord_user.id,
                        avatar
                    )
                } else {
                    format!(
                        "{}/embed/avatars/{}.png",
                        DiscordRequest::CDN_BASE_URI,
                        discord_user
                            .discriminator
                            .parse::<i32>()
                            .unwrap_or_default()
                            % 5
                    )
                },
            })
        } else {
            None
        }
    }
}
