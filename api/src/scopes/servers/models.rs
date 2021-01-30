use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct DiscordGuildPartial {
    pub id: String,
    pub name: String,
    pub icon: Option<String>,
    pub owner: bool,
    pub permissions: String,
    pub features: Vec<String>,
}

#[derive(Serialize)]
pub struct YuriServer {
    pub id: String,
    pub name: String,
    pub icon: String,
    pub user_is_mod: bool,
}
