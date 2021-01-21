use crate::discord_requests::DiscordRequest;
use crate::scopes::auth::models::o_auth::AuthSession;
use crate::scopes::user::models::user::{DiscordUser, YuriUser};

pub struct UserRepository;

impl UserRepository {
    pub async fn get_yuri_user(auth: &AuthSession) -> Option<YuriUser> {
        if let Some(discord_user) = DiscordRequest.get::<DiscordUser>("/users/@me", auth).await {
            Some(YuriUser {
                username: discord_user.username,
                discriminator: discord_user.discriminator.parse().ok()?,
                // TODO: complete this
                icon_url: "".to_string(),
                servers: vec![],
            })
        } else {
            None
        }
    }
}
