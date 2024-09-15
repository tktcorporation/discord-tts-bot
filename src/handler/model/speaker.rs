use serenity::{
    client::Context,
    model::{id, prelude::User, voice},
};
use std::result::Result;

#[derive(Debug, Clone)]
pub struct CurrentVoiceState {
    state: voice::VoiceState,
}

pub struct VoiceMember {
    pub guild_id: id::GuildId,
    pub user: User,
}

impl VoiceMember {
    pub async fn role(&self, ctx: &Context) -> Role {
        let current_user_id = ctx.cache.current_user().id;
        if current_user_id == self.user.id {
            return Role::Me;
        }
        Role::Other
    }
}

impl CurrentVoiceState {
    pub fn new(state: voice::VoiceState) -> Self {
        CurrentVoiceState { state }
    }

    pub async fn voice_member(self) -> Result<VoiceMember, String> {
        let guild_id = if let Some(guild_id) = self.state.guild_id {
            guild_id
        } else {
            return Err("The guild_id is None".to_string());
        };
        let user = self.state.member.unwrap().user;

        Ok(VoiceMember { guild_id, user })
    }

    pub fn change_of_states(
        &self,
        previous_voice_state: Option<&voice::VoiceState>,
    ) -> ChangeOfStates {
        match previous_voice_state {
            // 他サーバーに反応しないように
            Some(_) => {
                if self.state.channel_id.is_none() {
                    ChangeOfStates::Leave
                } else {
                    ChangeOfStates::Stay
                }
            }
            None => ChangeOfStates::Join,
        }
    }
}

pub enum ChangeOfStates {
    Join,
    Leave,
    Stay,
}

pub enum Role {
    Me,
    Other,
}
