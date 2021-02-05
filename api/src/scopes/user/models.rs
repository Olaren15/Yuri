use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct DiscordUser {
    pub id: String,
    pub username: String,
    pub discriminator: String,
    pub avatar: Option<String>,
    pub bot: Option<bool>,
    pub system: Option<bool>,
    pub mfa_enabled: Option<bool>,
    pub locale: Option<String>,
    pub verified: Option<bool>,
    pub email: Option<String>,
    pub flags: Option<i32>,
    pub premium_type: Option<i32>,
    pub public_flags: Option<i32>,
}

#[derive(Serialize)]
pub struct YuriUser {
    pub username: String,
    pub discriminator: u16,
    pub icon_url: String,
}
