use actix_files::Files;
use actix_session::CookieSession;
use actix_web::{web::Data, App, HttpServer};
use handlebars::Handlebars;
use rand::RngCore;

use common::db_connection::DbConnection;
use scopes::{auth::controllers::auth_controller, home::controllers::home_controller};

mod controllers;
mod models;
mod scopes;

pub fn register_handlebars() -> Handlebars<'static> {
    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory(".html", "src/views")
        .unwrap();
    handlebars
        .register_templates_directory(".html", "src/scopes/home/views")
        .unwrap();
    handlebars
        .register_templates_directory(".html", "src/scopes/auth/views")
        .unwrap();

    handlebars
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let handlebars_ref = Data::new(register_handlebars());
    let db_connection = Data::new(DbConnection::new().await);

    let mut seed: [u8; 32] = [0; 32];
    rand::thread_rng().fill_bytes(&mut seed.as_mut());

    HttpServer::new(move || {
        App::new()
            .wrap(CookieSession::private(&seed).secure(false))
            .app_data(handlebars_ref.clone())
            .app_data(db_connection.clone())
            .configure(home_controller::register)
            .configure(auth_controller::register)
            .service(
                Files::new("/public", "src/public")
                    .show_files_listing()
                    .use_last_modified(true),
            )
    })
    .bind("127.0.0.1:6969")?
    .run()
    .await
}
