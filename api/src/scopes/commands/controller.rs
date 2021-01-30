use actix_web::{
    get, web,
    web::{Data, Json, ServiceConfig},
    Responder,
};

use common::{db_connection::DbConnection, repositories::command_repository::CommandRepository};

pub fn register(cfg: &mut ServiceConfig) {
    cfg.service(web::scope("/commands").service(all));
}

#[get("/all")]
async fn all(connection: Data<DbConnection>) -> impl Responder {
    let commands = CommandRepository::new(&connection)
        .get_all_commands()
        .await
        .unwrap_or_default();

    Json(commands)
}
