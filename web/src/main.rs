use actix_web::{web::Data, App, HttpServer};
use handlebars::Handlebars;

use crate::controllers::home_controller;

mod controllers;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut handlebars = Handlebars::new();

    handlebars
        .register_templates_directory(".html", "src/views")
        .unwrap();

    let handlebars_ref = Data::new(handlebars);

    HttpServer::new(move || {
        App::new()
            .app_data(handlebars_ref.clone())
            .configure(home_controller::register)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
