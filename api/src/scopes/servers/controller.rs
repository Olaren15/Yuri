use std::sync::Mutex;

use actix_session::Session;
use actix_web::web::Json;
use actix_web::{
    get, web,
    web::{Data, ServiceConfig},
    HttpRequest, Responder,
};
use serenity::http::Http;

use common::db_conntext::DbContext;

use crate::discord_client_from_request::FromSession;
use crate::scopes::servers::repository::ServerRepository;

pub fn register(cfg: &mut ServiceConfig) {
    cfg.service(web::scope("/servers").service(in_common));
}

#[get("/in_common")]
async fn in_common(
    ctx: Data<DbContext>,
    client: Data<Mutex<Http>>,
    session: Session,
    req: HttpRequest,
) -> impl Responder {
    let mut client = client.lock().unwrap();
    client
        .set_token_from_session(&session, &ctx, &req.connection_info())
        .unwrap();

    let common_servers = ServerRepository::get_servers_in_common_with_yuri(&client, &ctx)
        .await
        .unwrap_or_default();

    Json(common_servers)
}
