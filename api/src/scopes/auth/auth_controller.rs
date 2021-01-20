use std::ops::Add;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use actix_session::Session;
use actix_web::{
    client::Client,
    get,
    http::header,
    web,
    web::{Data, Json, Query, ServiceConfig},
    HttpRequest, HttpResponse, Responder,
};

use common::{db_connection::DbConnection, repositories::settings_repository::SettingsRepository};

use crate::{
    models::server_response::ServerResponse,
    scopes::auth::models::o_auth::{
        AccessTokenExchange, AccessTokenFormData, AccessTokenResponse, CallbackData,
    },
};

pub fn register(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .service(authorize)
            .service(callback)
            .service(revoke),
    );
}

#[get("/authorize")]
async fn authorize(connection: Data<DbConnection>, req: HttpRequest) -> impl Responder {
    if let Ok(settings) = SettingsRepository::new(&connection)
        .get_highest_weight_settings()
        .await
    {
        let redirect = format!(
            "https://discord.com/api/v8/oauth2/authorize\
            ?client_id={}\
            &redirect_uri={}://{}/api/auth/callback\
            &response_type=code\
            &scope=identify guilds",
            settings.oauth2_client_id,
            req.connection_info().scheme(),
            req.connection_info().host()
        );

        HttpResponse::SeeOther()
            .header(header::LOCATION, redirect)
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
        let form_data = AccessTokenExchange {
            client_id: settings.oauth2_client_id,
            client_secret: settings.oauth2_client_secret,
            grant_type: String::from("authorization_code"),
            code: callback.code.clone(),
            redirect_uri: format!(
                "{}://{}/api/auth/callback",
                req.connection_info().scheme(),
                req.connection_info().host()
            ),
            scope: String::from("identify guilds"),
        };

        if let Ok(mut response) = Client::new()
            .post("https://discord.com/api/v8/oauth2/token")
            .send_form(&form_data)
            .await
        {
            if let Ok(response) = response.json::<AccessTokenResponse>().await {
                session.set("access_token", response.access_token).ok();
                session.set("refresh_token", response.refresh_token).ok();
                session
                    .set(
                        "token_expire_time",
                        SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .unwrap()
                            .add(Duration::from_secs(response.expires_in as u64))
                            .as_secs(),
                    )
                    .ok();

                // redirect home
                return HttpResponse::SeeOther()
                    .header(header::LOCATION, "/")
                    .finish();
            } else {
                err_msg = "Failed to parse Discord response";
            }
        } else {
            err_msg = "Failed to retrieve token from the Discord server";
        }
    } else {
        err_msg = "Server error";
    }

    HttpResponse::InternalServerError().body(err_msg)
}

#[get("/revoke")]
async fn revoke(session: Session) -> impl Responder {
    if let Some(access_token) = session.get::<String>("access_token").unwrap() {
        let form_data = AccessTokenFormData { access_token };

        if Client::new()
            .post("https://discord.com/api/v8/oauth2/revoke")
            .send_form(&form_data)
            .await
            .is_ok()
        {
            return Json(ServerResponse {
                code: 200,
                message: String::from(""),
            });
        }
    }

    Json(ServerResponse {
        code: 500,
        message: String::from("failed to revoke token"),
    })
}
