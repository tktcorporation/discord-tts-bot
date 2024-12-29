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
    pub async fn role(&self, ctx: &Context) -> Result<Role, String> {
        let current_user_id = ctx.cache.current_user().id;
        if current_user_id == self.user.id {
            return Ok(Role::Me);
        }
        Ok(Role::Other)
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
        ctx: &Context,
    ) -> ChangeOfStates {
        match previous_voice_state {
            Some(prev_state) => {
                if self.state.channel_id.is_none() {
                    ChangeOfStates::Leave
                } else if let (Some(prev_channel), Some(curr_channel)) =
                    (prev_state.channel_id, self.state.channel_id)
                {
                    if prev_channel == curr_channel {
                        ChangeOfStates::Stay
                    } else if let Some(guild_id) = self.state.guild_id {
                        let afk_channel_id = ctx
                            .cache
                            .guild(guild_id)
                            .and_then(|g| g.afk_metadata.clone())
                            .map(|m| m.afk_channel_id);

                        if let Some(afk_channel_id) = afk_channel_id {
                            if prev_channel != afk_channel_id && curr_channel == afk_channel_id {
                                ChangeOfStates::EnterAFK
                            } else if prev_channel == afk_channel_id
                                && curr_channel != afk_channel_id
                            {
                                ChangeOfStates::LeaveAFK
                            } else {
                                ChangeOfStates::Stay
                            }
                        } else {
                            ChangeOfStates::Stay
                        }
                    } else {
                        ChangeOfStates::Stay
                    }
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
    EnterAFK,
    LeaveAFK,
}

pub enum Role {
    Me,
    Other,
}
