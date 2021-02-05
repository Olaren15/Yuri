use actix_session::Session;
use actix_web::{
    get,
    http::header,
    web,
    web::{Data, Query, ServiceConfig},
    HttpRequest, HttpResponse, Responder,
};

use common::{db_conntext::DbContext, repositories::settings_repository::SettingsRepository};

use crate::scopes::auth::{
    models::{AuthSession, CallbackData},
    repository::AuthRepository,
};

pub fn register(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .service(authorize)
            .service(callback)
            .service(revoke),
    );
}

#[get("/login")]
async fn authorize(connection: Data<DbContext>, req: HttpRequest) -> impl Responder {
    if let Ok(settings) = SettingsRepository::new(&connection)
        .get_highest_weight_settings()
        .await
    {
        HttpResponse::SeeOther()
            .header(
                header::LOCATION,
                AuthRepository::build_authorization_url(&settings, &req.connection_info()),
            )
            .finish()
    } else {
        HttpResponse::InternalServerError().finish()
    }
}

#[get("/callback")]
async fn callback(
    callback: Query<CallbackData>,
    connection: Data<DbContext>,
    session: Session,
    req: HttpRequest,
) -> impl Responder {
    let err_msg;

    if let Ok(settings) = SettingsRepository::new(&connection)
        .get_highest_weight_settings()
        .await
    {
        if let Some(token_response) =
            AuthRepository::exchange_access_token(&settings, &callback.code, &req.connection_info())
                .await
        {
            session
                .set(
                    "auth",
                    AuthSession::from_access_token_response(token_response),
                )
                .ok();

            return HttpResponse::SeeOther()
                .header(header::LOCATION, "/")
                .finish();
        } else {
            err_msg = "Failed to connect to discord servers";
        }
    } else {
        err_msg = "Internal Server Error";
    }

    HttpResponse::InternalServerError().body(err_msg)
}

#[get("/logout")]
async fn revoke(session: Session) -> impl Responder {
    session.remove("auth");

    HttpResponse::SeeOther()
        .header(header::LOCATION, "/")
        .finish()
}
