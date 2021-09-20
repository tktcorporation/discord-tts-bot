use super::interface::Speaker;

use super::super::model::{
    speaker::{self, CurrentVoiceState, VoiceMember},
    text_to_speech_message::SpeechMessage,
    voice::Voice,
};
#[cfg(feature = "tts")]
use serenity::model::voice;

use serenity::client::Context;

pub async fn change_check(
    ctx: &Context,
    state: CurrentVoiceState,
    old_voice_state: Option<voice::VoiceState>,
) {
    match state.role(ctx).await {
        speaker::Role::Bot => return println!("This is me(bot). My entering is ignored."),
        _ => {},
    };
    match state.change_of_states(old_voice_state) {
        Join => {},
        Leave => {},
        Stay =>{},
    }
}

pub async fn speech_welcome_see_you(
    ctx: &Context,
    state: CurrentVoiceState,
    old_voice_state: Option<voice::VoiceState>,
) {
    match state.voice_member(ctx, old_voice_state).await {
        Ok(voice_member) => {
            let voice = Voice::from(ctx, voice_member.guild_id).await;

            // botしかいなかったら
            match voice.is_alone(ctx).await {
                Ok(is_alone) => {
                    if is_alone {
                        voice.leave().await.unwrap();
                    } else {
                        voice.speech(welcome_or_see_you_messsage(&voice_member)).await;
                    }
                }
                Err(str) => {
                    println!("[DEBUG] {:?}", str)
                }
            }
        }
        Err(str) => {
            println!("[DEBUG] {:?}", str)
        }
    }
}

fn welcome_or_see_you_messsage(voice_member: &VoiceMember) -> SpeechMessage {
    SpeechMessage {
        value: if voice_member.is_new {
            format!("{:?} さんいらっしゃい", voice_member.user.name)
        } else {
            format!("{:?} さんいってらっしゃい", voice_member.user.name)
        },
    }
}
