use std::time::{SystemTime, UNIX_EPOCH};

use actix_session::Session;
use actix_web::dev::ConnectionInfo;
use serenity::futures::executor;
use serenity::http::Http;

use common::db_conntext::DbContext;

use crate::scopes::auth::models::AuthSession;
use crate::scopes::auth::repository::AuthRepository;

pub trait FromSession {
    fn set_token_from_session(
        &mut self,
        session: &Session,
        ctx: &DbContext,
        conn_info: &ConnectionInfo,
    ) -> Result<(), ()>;
}

impl FromSession for Http {
    fn set_token_from_session(
        &mut self,
        session: &Session,
        ctx: &DbContext,
        conn_info: &ConnectionInfo,
    ) -> Result<(), ()> {
        if let Some(auth) = session.get::<AuthSession>("auth").unwrap() {
            let auth = if auth.expire_time
                <= SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs()
            {
                executor::block_on(async {
                    AuthRepository::renew_token(auth, &ctx, &conn_info).await
                })
            } else {
                auth
            };

            // this is the stupid line that needs a custom version of the serenity crate
            // but I'm not gonna lie... I *am* doing something stupid
            self.ratelimiter.token = format!("Bearer {}", auth.access_token);

            Ok(())
        } else {
            Err(())
        }
    }
}
