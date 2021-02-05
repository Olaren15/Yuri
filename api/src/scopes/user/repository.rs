use serenity::http::Http;

use crate::scopes::user::models::YuriUser;

pub struct UserRepository;

impl UserRepository {
    pub async fn get_yuri_user(client: &Http) -> Option<YuriUser> {
        let user = client.get_current_user().await.ok()?;

        Some(YuriUser {
            username: user.name.clone(),
            discriminator: user.discriminator,
            icon_url: user
                .avatar_url()
                .unwrap_or_else(|| user.default_avatar_url()),
        })
    }
}
