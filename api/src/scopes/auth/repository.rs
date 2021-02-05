use actix_web::client::Client;
use actix_web::dev::ConnectionInfo;

use common::db_conntext::DbContext;
use common::models::settings::Settings;
use common::repositories::settings_repository::SettingsRepository;

use crate::scopes::auth::models::{
    AccessTokenExchange, AccessTokenRefresh, AccessTokenResponse, AuthSession,
};

pub struct AuthRepository;

impl AuthRepository {
    const API_BASE_URI: &'static str = "https://discord.com/api/v6";

    pub fn build_authorization_url(settings: &Settings, conn_info: &ConnectionInfo) -> String {
        format!(
            "{}/oauth2/authorize\
                ?client_id={}\
                &redirect_uri={}://{}/api/auth/callback\
                &response_type=code\
                &scope=identify guilds",
            Self::API_BASE_URI,
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
            .post(format!("{}/oauth2/token", Self::API_BASE_URI))
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

    pub async fn renew_token(
        auth: AuthSession,
        ctx: &DbContext,
        conn_info: &ConnectionInfo,
    ) -> AuthSession {
        if let Ok(settings) = SettingsRepository::new(&ctx)
            .get_highest_weight_settings()
            .await
        {
            if let Ok(mut response) = Client::new()
                .post(format!("{}/auth2/token", Self::API_BASE_URI))
                .send_form(&AccessTokenRefresh {
                    client_id: settings.oauth2_client_id,
                    client_secret: settings.oauth2_client_secret,
                    grant_type: String::from("refresh_token"),
                    refresh_token: auth.refresh_token.clone(),
                    redirect_uri: format!(
                        "{}://{}/api/auth/callback",
                        conn_info.scheme(),
                        conn_info.host()
                    ),
                    scope: String::from("identify guilds"),
                })
                .await
            {
                if let Ok(response) = response.json::<AccessTokenResponse>().await {
                    return AuthSession::from_access_token_response(response);
                }
            }
        }

        // don't replace the token and hope for the best
        auth
    }
}
