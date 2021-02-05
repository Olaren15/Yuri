use std::sync::Mutex;

use actix_session::Session;
use actix_web::web::Data;
use actix_web::{
    get, web,
    web::{Json, ServiceConfig},
    HttpRequest, Responder,
};
use serenity::http::Http;

use common::db_conntext::DbContext;

use crate::discord_client_from_request::FromSession;
use crate::scopes::user::repository::UserRepository;

pub fn register(cfg: &mut ServiceConfig) {
    cfg.service(web::scope("/user").service(current));
}

#[get("/current")]
async fn current(
    ctx: Data<DbContext>,
    client: Data<Mutex<Http>>,
    session: Session,
    req: HttpRequest,
) -> impl Responder {
    let mut client = client.lock().unwrap();
    let user = if client
        .set_token_from_session(&session, &ctx, &req.connection_info())
        .is_ok()
    {
        UserRepository::get_yuri_user(&client).await
    } else {
        None
    };

    Json(user)
}
