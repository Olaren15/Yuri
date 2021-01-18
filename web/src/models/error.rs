use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorModel {
    pub what: String,
}
