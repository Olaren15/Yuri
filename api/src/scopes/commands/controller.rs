use actix_web::web::Path;
use actix_web::{
    get, web,
    web::{Data, Json, ServiceConfig},
    Responder,
};

use common::{db_conntext::DbContext, repositories::command_repository::CommandRepository};

pub fn register(cfg: &mut ServiceConfig) {
    cfg.service(web::scope("/commands").service(all));
}

#[get("/for_guild/{guild_id}")]
async fn all(connection: Data<DbContext>, guild_id: Path<u64>) -> impl Responder {
    let commands = CommandRepository::new(&connection)
        .get_all_commands_in_guild(*guild_id)
        .await
        .unwrap_or_default();

    Json(commands)
}
