use std::ops::Add;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use actix_session::Session;
use actix_web::{
    get,
    http::header,
    web,
    web::{Data, Query, ServiceConfig},
    HttpRequest, HttpResponse, Responder,
};

use common::{db_connection::DbConnection, repositories::settings_repository::SettingsRepository};

use crate::scopes::auth::{
    models::o_auth::{AuthSession, CallbackData},
    repositories::auth_repository::AuthRepository,
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
async fn authorize(connection: Data<DbConnection>, req: HttpRequest) -> impl Responder {
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
    connection: Data<DbConnection>,
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
                    AuthSession {
                        access_token: token_response.access_token,
                        refresh_token: token_response.refresh_token,
                        expire_time: SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .unwrap()
                            .add(Duration::from_secs(token_response.expires_in as u64))
                            .as_secs(),
                    },
                )
                .ok()
                .unwrap();

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
