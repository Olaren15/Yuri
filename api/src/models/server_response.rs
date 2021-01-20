use serde::Serialize;

#[derive(Serialize)]
pub struct ServerResponse {
    pub code: i32,
    pub message: String,
}
