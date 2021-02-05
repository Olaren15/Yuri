use serenity::http::{GuildPagination, Http};
use serenity::model::Permissions;

use common::db_conntext::DbContext;
use common::repositories::guild_repository::GuildRepository;

use crate::scopes::servers::models::YuriServer;

pub struct ServerRepository;

impl ServerRepository {
    pub async fn get_servers_in_common_with_yuri(
        client: &Http,
        conn: &DbContext,
    ) -> Option<Vec<YuriServer>> {
        if let Ok(guilds) = client
            .get_guilds(&GuildPagination::After(0.into()), 100)
            .await
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
                            if guild.id.0 == *yuri_guild {
                                return true;
                            }
                        }
                        false
                    })
                    .map(|guild_info| YuriServer {
                        id: guild_info.id.to_string(),
                        name: guild_info.name.clone(),
                        icon: guild_info.icon_url().unwrap_or_default(),
                        user_is_mod: guild_info.permissions.contains(Permissions::ADMINISTRATOR),
                    })
                    .collect(),
            )
        } else {
            None
        }
    }
}
