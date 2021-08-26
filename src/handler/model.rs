use serenity::{
    client::Context,
    model::{id, prelude::User, voice},
};
use std::result::Result;

pub struct CurrentVoiceState {
    state: voice::VoiceState,
}

pub struct NewSpeaker {
    pub guild_id: id::GuildId,
    pub user: User,
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
    ) -> Result<NewSpeaker, &str> {
        return match previous_voice_state {
            // 他サーバーに反応しないように
            Some(_) => Err("This is not a new speaker. The previous is not None."),
            None => {
                if let Some(guild_id) = self.state.guild_id {
                    let current_user_id = ctx.cache.current_user_id().await;
                    let user = self.state.member.unwrap().user;

                    if current_user_id == user.id {
                        return Err("This is me(bot). My entering is ignored.");
                    }

                    Ok(NewSpeaker { guild_id, user })
                } else {
                    Err("The guild_id is None")
                }
            }
        };
    }
}
