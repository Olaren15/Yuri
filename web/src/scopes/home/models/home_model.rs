use serde::Serialize;

#[derive(Serialize)]
pub struct HomeModel {
    pub title: String,
    pub logged_in: bool,
    pub name: String,
}
