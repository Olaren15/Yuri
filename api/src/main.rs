use std::sync::Mutex;

use actix_session::CookieSession;
use actix_web::{cookie::SameSite, web, web::Data, App, HttpServer};
use rand::RngCore;
use serenity::http::Http;

use common::db_conntext::DbContext;
use scopes::{auth, commands, servers, user};

mod discord_client_from_request;
mod scopes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // session key
    let mut seed: [u8; 32] = [0; 32];
    rand::thread_rng().fill_bytes(&mut seed.as_mut());

    // app data
    let db_connection = Data::new(DbContext::new().await);
    let http = Http::new_with_token("");
    let discord_client = Data::new(Mutex::new(http)); // dummy token

    HttpServer::new(move || {
        App::new()
            .wrap(
                CookieSession::private(&seed)
                    .same_site(SameSite::Strict)
                    .path("/api")
                    .secure(false),
            )
            .app_data(db_connection.clone())
            .app_data(discord_client.clone())
            .service(
                web::scope("/api")
                    .configure(auth::controller::register)
                    .configure(user::controller::register)
                    .configure(commands::controller::register)
                    .configure(servers::controller::register),
            )
    })
    .bind("127.0.0.1:6969")?
    // keep alive for one day
    .keep_alive(86400)
    .run()
    .await
}
