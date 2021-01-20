use actix_files::Files;
use actix_session::CookieSession;
use actix_web::{web, web::Data, App, HttpServer};
use rand::RngCore;

use common::db_connection::DbConnection;
use scopes::auth::auth_controller;

mod scopes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_connection = Data::new(DbConnection::new().await);

    let mut seed: [u8; 32] = [0; 32];
    rand::thread_rng().fill_bytes(&mut seed.as_mut());

    HttpServer::new(move || {
        App::new()
            .wrap(CookieSession::private(&seed).secure(false))
            .app_data(db_connection.clone())
            .service(
                web::scope("/api")
                    .configure(auth_controller::register)
                    .service(
                        Files::new("/public", "src/public")
                            .show_files_listing()
                            .use_last_modified(true),
                    ),
            )
    })
    .bind("127.0.0.1:6969")?
    .run()
    .await
}
