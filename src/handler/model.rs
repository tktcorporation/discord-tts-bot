use serenity::{
    client::Context,
    model::{id, prelude::User, voice},
};
use std::result::Result;

pub struct CurrentVoiceState {
    state: voice::VoiceState,
}

pub struct Speaker {
    pub guild_id: id::GuildId,
    pub user: User,
    /// true -> new speaker
    /// false -> leaved speaker
    pub is_new: bool,
    pub member_count: usize,
}

impl CurrentVoiceState {
    pub fn new(state: voice::VoiceState) -> Self {
        CurrentVoiceState { state }
    }

    /// [`previous_voice_state`] が空で、現在の [`voice::VoiceState`] も Seaker として動いている場合に [`NewSpeaker`] を返す
    pub async fn new_speaker(
        self,
        ctx: &Context,
        previous_voice_state: Option<voice::VoiceState>,
    ) -> Result<Speaker, &str> {
        let guild_id = if let Some(guild_id) = self.state.guild_id {
            guild_id
        } else {
            return Err("The guild_id is None");
        };
        let user = self.state.member.unwrap().user;
        let current_user_id = ctx.cache.current_user_id().await;
        if current_user_id == user.id {
            return Err("This is me(bot). My entering is ignored.");
        }

        let channel_id = if self.state.channel_id.is_some() {
            self.state.channel_id.unwrap()
        } else {
            previous_voice_state.clone().unwrap().channel_id.unwrap()
        };
        let members = members(ctx, &guild_id, &channel_id).await.unwrap().len();

        match previous_voice_state {
            // 他サーバーに反応しないように
            Some(_) => {
                if self.state.channel_id.is_none() {
                    Ok(Speaker {
                        guild_id,
                        user,
                        is_new: false,
                        member_count: members,
                    })
                } else {
                    Err("This is not a new speaker. The previous is not None.")
                }
            }
            None => Ok(Speaker {
                guild_id,
                user,
                is_new: true,
                member_count: members,
            }),
        }
    }
}

async fn members(
    ctx: &Context,
    guild_id: &id::GuildId,
    channel_id: &id::ChannelId,
) -> std::result::Result<std::vec::Vec<serenity::model::guild::Member>, String> {
    let channels = guild_id.channels(&ctx.http.as_ref()).await.unwrap();
    match channels.get(channel_id) {
        Some(guild_channel) => Ok(guild_channel.members(&ctx.cache).await.unwrap()),
        _ => Err("can't get a channel id".to_string()),
    }
}
