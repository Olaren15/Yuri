use common::db_connection::DbConnection;
use common::repositories::guild_repository::GuildRepository;

use crate::discord_requests::DiscordRequest;
use crate::scopes::auth::models::AuthSession;
use crate::scopes::servers::models::{DiscordGuildPartial, YuriServer};

pub struct ServerRepository;

impl ServerRepository {
    pub async fn get_servers_in_common_with_yuri(
        auth: &AuthSession,
        conn: &DbConnection,
    ) -> Option<Vec<YuriServer>> {
        if let Some(guilds) =
            DiscordRequest::get::<Vec<DiscordGuildPartial>>("/users/@me/guilds", auth).await
        {
            let yuri_guilds = GuildRepository::new(&conn)
                .get_registered_guild_ids()
                .await
                .unwrap_or_default();

            Some(
                guilds
                    .into_iter()
                    .filter(|guild| {
                        for yuri_guild in &yuri_guilds {
                            if let Ok(guild_id) = guild.id.parse::<u64>() {
                                if guild_id == *yuri_guild {
                                    return true;
                                }
                            }
                        }
                        false
                    })
                    .map(|partial| YuriServer {
                        id: partial.id.clone(),
                        name: partial.name,
                        icon: if let Some(icon) = partial.icon {
                            format!(
                                "{}/icons/{}/{}.png",
                                DiscordRequest::CDN_BASE_URI,
                                partial.id,
                                icon
                            )
                        } else {
                            String::from("")
                        },
                        user_is_mod: partial.owner
                            // can manage channels
                            || (partial.permissions.parse::<i32>().unwrap_or_default() & 16 == 16),
                    })
                    .collect(),
            )
        } else {
            None
        }
    }
}
