#[derive(sqlx::FromRow)]
pub struct Settings {
    pub id: i32,
    pub weight: i32,
    pub debug_token: String,
    pub release_token: String,
    pub command_prefix: String,
}

impl Settings {
    #[cfg(debug_assertions)]
    pub fn get_token_from_config(&self) -> String {
        self.debug_token.clone()
    }

    #[cfg(not(debug_assertions))]
    pub fn get_token_from_config(&self) -> String {
        self.release_token.clone()
    }
}
