use actix_web::client::Client;
use actix_web::dev::ConnectionInfo;

use common::models::settings::Settings;

use crate::scopes::auth::models::o_auth::{AccessTokenExchange, AuthSession};
use crate::{discord_requests::DiscordRequest, scopes::auth::models::o_auth::AccessTokenResponse};

pub struct AuthRepository;

impl AuthRepository {
    pub fn build_authorization_url(settings: &Settings, conn_info: &ConnectionInfo) -> String {
        format!(
            "{}/oauth2/authorize\
                ?client_id={}\
                &redirect_uri={}://{}/api/auth/callback\
                &response_type=code\
                &scope=identify guilds",
            DiscordRequest::BASE_URI,
            settings.oauth2_client_id,
            conn_info.scheme(),
            conn_info.host()
        )
    }

    pub async fn exchange_access_token(
        settings: &Settings,
        code: &String,
        conn_info: &ConnectionInfo,
    ) -> Option<AccessTokenResponse> {
        Client::new()
            .post(format!("{}{}", DiscordRequest::BASE_URI, "/oauth2/token"))
            .send_form(&AccessTokenExchange {
                client_id: settings.oauth2_client_id.clone(),
                client_secret: settings.oauth2_client_secret.clone(),
                grant_type: String::from("authorization_code"),
                code: code.clone(),
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

    pub fn refresh_token(_: &AuthSession) {
        unimplemented!()
    }
}
