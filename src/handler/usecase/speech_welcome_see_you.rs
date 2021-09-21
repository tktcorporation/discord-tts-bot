use super::interface::Speaker;

use super::super::model::{
    speaker::{self, ChangeOfStates, CurrentVoiceState},
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
    let message_prefix = match state.change_of_states(old_voice_state) {
        ChangeOfStates::Stay => None,
        ChangeOfStates::Leave => Some("いってらっしゃい"),
        ChangeOfStates::Join => Some("いらっしゃい"),
    };
    let member = state.voice_member().await.expect("member is not received");
    let voice = Voice::from(ctx, member.guild_id).await;
    if voice.is_alone(&ctx).await.unwrap() {
        return voice.leave().await.unwrap();
    }
    if let speaker::Role::Bot = member.role(ctx).await {
        return println!("This is me(bot). My entering is ignored.");
    }
    if let Some(message_prefix) = message_prefix {
        voice
            .speech(SpeechMessage {
                value: format!("{:?} {:?}", message_prefix, member.user.name),
            })
            .await
    }
}
