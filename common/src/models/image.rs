#[derive(sqlx::FromRow)]
pub struct Image {
    pub id: i32,
    pub command_id: i32,
    pub url: String,
}
