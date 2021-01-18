use actix_web::{
    get,
    web::{Data, ServiceConfig},
    HttpResponse, Responder,
};
use handlebars::Handlebars;

use crate::scopes::home::models::home_model::HomeModel;

pub fn register(cfg: &mut ServiceConfig) {
    cfg.service(home);
}

#[get("/")]
async fn home(handlebars: Data<Handlebars<'_>>) -> impl Responder {
    HttpResponse::Ok().body(
        handlebars
            .render(
                "home",
                &HomeModel {
                    title: String::from("Yuri"),
                    logged_in: false,
                    name: String::from("ree"),
                },
            )
            .unwrap_or_default(),
    )
}
