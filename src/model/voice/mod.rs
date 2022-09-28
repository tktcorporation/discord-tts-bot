mod path;
use serenity::{client::Context, model::id};

use songbird::{self, Songbird};

pub struct Voice {
    pub manager: std::sync::Arc<Songbird>,
    pub guild_id: id::GuildId,
}

#[derive(Debug)]
pub enum Error {
    NotInVoiceChannel,
    ConnectionNotFound,
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::NotInVoiceChannel => f.write_str("Not in a voice channel."),
            Error::ConnectionNotFound => f.write_str("Connection not found."),
        }
    }
}

impl Voice {
    pub async fn from(ctx: &Context, guild_id: id::GuildId) -> Voice {
        let manager = songbird::get(ctx)
            .await
            .expect("Songbird Voice client placed in at initialisation.");
        Voice { manager, guild_id }
    }

    pub async fn join(
        &self,
        connect_to: id::ChannelId,
    ) -> (
        std::sync::Arc<tokio::sync::Mutex<songbird::Call>>,
        songbird::error::JoinResult<()>,
    ) {
        self.manager.join(self.guild_id, connect_to).await
    }

    pub async fn handler(
        &self,
    ) -> Result<std::sync::Arc<serenity::prelude::Mutex<songbird::Call>>, Error> {
        match self.manager.get(self.guild_id) {
            Some(handler) => Ok(handler),
            None => Err(Error::NotInVoiceChannel),
        }
    }

    pub async fn members(
        &self,
        ctx: &Context,
    ) -> std::result::Result<std::vec::Vec<serenity::model::guild::Member>, Error> {
        match self.guild_id_and_channel_id().await {
            Ok((guild_id, channel_id)) => Ok(_members(ctx, &guild_id, &channel_id.unwrap()).await),
            Err(e) => Err(e),
        }
    }

    pub async fn is_alone(&self, ctx: &Context) -> Result<bool, Error> {
        let members = match self.members(ctx).await {
            Ok(members) => members,
            Err(e) => return Err(e),
        };
        // exclude bot members
        Ok(!members.iter().any(|member| !member.user.bot))
    }

    pub async fn remove(&self) -> std::result::Result<(), songbird::error::JoinError> {
        match self.manager.remove(self.guild_id).await {
            Ok(_) => Ok(()),
            Err(e) => match e {
                songbird::error::JoinError::Dropped => Ok(()),
                songbird::error::JoinError::NoCall => Ok(()),
                _ => Err(e),
            },
        }
    }

    pub async fn guild_id_and_channel_id(
        &self,
    ) -> Result<(songbird::id::GuildId, Option<songbird::id::ChannelId>), Error> {
        let handler = match self.handler().await {
            Ok(handler) => handler,
            Err(e) => return Err(e),
        };
        match get_guild_id_and_channel_id(&handler).await {
            Ok(ids) => Ok(ids),
            Err(e) => Err(e),
        }
    }
}

async fn _members(
    ctx: &Context,
    guild_id: &songbird::id::GuildId,
    channel_id: &songbird::id::ChannelId,
) -> std::vec::Vec<serenity::model::guild::Member> {
    let guild_id = id::GuildId::from(guild_id.0);
    let channel_id = id::ChannelId::from(channel_id.0);
    let channels = guild_id.channels(&ctx.http.as_ref()).await.unwrap();
    let guild_channel = channels.get(&channel_id).unwrap();
    guild_channel.members(&ctx.cache).await.unwrap()
}

async fn get_guild_id_and_channel_id(
    handler: &std::sync::Arc<serenity::prelude::Mutex<songbird::Call>>,
) -> Result<(songbird::id::GuildId, Option<songbird::id::ChannelId>), Error> {
    let handler_lock = handler.lock().await;
    if let Some(connection) = handler_lock.current_connection() {
        Ok((connection.guild_id, connection.channel_id))
    } else {
        Err(Error::ConnectionNotFound)
    }
}
