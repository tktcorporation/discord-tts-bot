mod path;
use serenity::{client::Context, model::id};

use songbird::{self, Songbird};

pub struct Voice {
    pub manager: std::sync::Arc<Songbird>,
    pub guild_id: id::GuildId,
}

impl Voice {
    pub async fn from(ctx: &Context, guild_id: id::GuildId) -> Voice {
        let manager = songbird::get(ctx)
            .await
            .expect("Songbird Voice client placed in at initialisation.");
        Voice { manager, guild_id }
    }

    pub async fn handler(
        &self,
    ) -> Result<std::sync::Arc<serenity::prelude::Mutex<songbird::Call>>, &str> {
        match self.manager.get(self.guild_id) {
            Some(handler) => Ok(handler),
            None => Err("not in voice channel"),
        }
    }

    pub async fn members(
        &self,
        ctx: &Context,
    ) -> std::result::Result<std::vec::Vec<serenity::model::guild::Member>, String> {
        match self.guild_id_and_channel_id().await {
            Ok((guild_id, channel_id)) => _members(ctx, &guild_id, &channel_id.unwrap()).await,
            Err(str) => Err(str),
        }
    }

    pub async fn is_alone(&self, ctx: &Context) -> Result<bool, String> {
        let members = match self.members(ctx).await {
            Ok(members) => members,
            Err(str) => return Err(str),
        };
        // exclude bot members
        let members = members
            .iter()
            .filter(|member| !member.user.bot)
            .collect::<Vec<_>>();
        Ok(members.is_empty())
    }

    pub async fn leave(&self) -> std::result::Result<(), songbird::error::JoinError> {
        self.manager.leave(self.guild_id).await
    }

    pub async fn guild_id_and_channel_id(
        &self,
    ) -> Result<(songbird::id::GuildId, Option<songbird::id::ChannelId>), String> {
        let handler = match self.handler().await {
            Ok(handler) => handler,
            Err(str) => return Err(str.to_string()),
        };
        match get_guild_id_and_channel_id(&handler).await {
            Ok(ids) => Ok(ids),
            Err(str) => Err(str.to_string()),
        }
    }
}

async fn _members(
    ctx: &Context,
    guild_id: &songbird::id::GuildId,
    channel_id: &songbird::id::ChannelId,
) -> std::result::Result<std::vec::Vec<serenity::model::guild::Member>, String> {
    let guild_id = id::GuildId::from(guild_id.0);
    let channel_id = id::ChannelId::from(channel_id.0);
    let channels = guild_id.channels(&ctx.http.as_ref()).await.unwrap();
    match channels.get(&channel_id) {
        Some(guild_channel) => Ok(guild_channel.members(&ctx.cache).await.unwrap()),
        _ => Err("can't get a channel id".to_string()),
    }
}

async fn get_guild_id_and_channel_id(
    handler: &std::sync::Arc<serenity::prelude::Mutex<songbird::Call>>,
) -> Result<(songbird::id::GuildId, Option<songbird::id::ChannelId>), &'static str> {
    let handler_lock = handler.lock().await;
    if let Some(connection) = handler_lock.current_connection() {
        Ok((connection.guild_id, connection.channel_id))
    } else {
        Err("connection not found")
    }
}
