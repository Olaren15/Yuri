use actix_session::Session;
use actix_web::web::Json;
use actix_web::{
    get, web,
    web::{Data, ServiceConfig},
    Responder,
};

use common::db_connection::DbConnection;

use crate::scopes::servers::repository::ServerRepository;

pub fn register(cfg: &mut ServiceConfig) {
    cfg.service(web::scope("/servers").service(in_common));
}

#[get("/in_common")]
async fn in_common(connection: Data<DbConnection>, session: Session) -> impl Responder {
    let common_servers = if let Some(auth) = session.get("auth").unwrap_or(None) {
        ServerRepository::get_servers_in_common_with_yuri(&auth, &connection)
            .await
            .unwrap_or_default()
    } else {
        vec![]
    };

    Json(common_servers)
}
