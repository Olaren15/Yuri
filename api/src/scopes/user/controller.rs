use actix_session::Session;
use actix_web::{
    get, web,
    web::{Json, ServiceConfig},
    Responder,
};

use crate::scopes::user::repository::UserRepository;

pub fn register(cfg: &mut ServiceConfig) {
    cfg.service(web::scope("/user").service(current));
}

#[get("/current")]
async fn current(session: Session) -> impl Responder {
    let user = if let Some(auth) = session.get("auth").unwrap() {
        UserRepository::get_yuri_user(&auth).await
    } else {
        None
    };

    Json(user)
}
