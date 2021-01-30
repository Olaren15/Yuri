use actix_web::client::Client;
use actix_web::dev::ConnectionInfo;

use common::models::settings::Settings;

use crate::{
    discord_requests::DiscordRequest,
    scopes::auth::models::{AccessTokenExchange, AccessTokenResponse},
};

pub struct AuthRepository;

impl AuthRepository {
    pub fn build_authorization_url(settings: &Settings, conn_info: &ConnectionInfo) -> String {
        format!(
            "{}/oauth2/authorize\
                ?client_id={}\
                &redirect_uri={}://{}/api/auth/callback\
                &response_type=code\
                &scope=identify guilds",
            DiscordRequest::API_BASE_URI,
            settings.oauth2_client_id,
            conn_info.scheme(),
            conn_info.host()
        )
    }

    pub async fn exchange_access_token(
        settings: &Settings,
        code: &str,
        conn_info: &ConnectionInfo,
    ) -> Option<AccessTokenResponse> {
        Client::new()
            .post(format!(
                "{}{}",
                DiscordRequest::API_BASE_URI,
                "/oauth2/token"
            ))
            .send_form(&AccessTokenExchange {
                client_id: settings.oauth2_client_id.clone(),
                client_secret: settings.oauth2_client_secret.clone(),
                grant_type: String::from("authorization_code"),
                code: String::from(code),
                redirect_uri: format!(
                    "{}://{}/api/auth/callback",
                    conn_info.scheme(),
                    conn_info.host()
                ),
                scope: String::from("identify guilds"),
            })
            .await
            .ok()?
            .json::<AccessTokenResponse>()
            .await
            .ok()
    }
}
