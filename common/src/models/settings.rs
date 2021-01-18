#[derive(sqlx::FromRow)]
pub struct Settings {
    pub id: i32,
    pub is_release: bool,
    pub weight: i32,
    pub command_prefix: String,
    pub bot_token: String,
    pub oauth2_client_id: String,
    pub oauth2_client_secret: String,
}
