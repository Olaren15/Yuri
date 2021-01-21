use actix_session::Session;
use actix_web::{
    get, web,
    web::{Json, ServiceConfig},
    Responder,
};

use crate::scopes::user::repositories::user_repository::UserRepository;

pub fn register(cfg: &mut ServiceConfig) {
    cfg.service(web::scope("/user").service(current));
}

#[get("/current")]
async fn current(session: Session) -> impl Responder {
    let user = if let Some(access_token) = session.get("access_token").unwrap() {
        UserRepository::new(access_token).get_yuri_user().await
    } else {
        None
    };

    Json(user)
}
