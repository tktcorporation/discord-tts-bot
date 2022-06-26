use super::interface::Speaker;

use super::super::model::{
    speaker::{self, ChangeOfStates, CurrentVoiceState},
    voice::Voice,
};
use super::text_to_speech::SpeechMessage;
#[cfg(feature = "tts")]
use serenity::model::voice;

use serenity::client::Context;

pub async fn change_check(
    ctx: &Context,
    state: CurrentVoiceState,
    old_voice_state: Option<voice::VoiceState>,
) {
    let change = state.change_of_states(old_voice_state);
    let member = state.voice_member().await.expect("member is not received");
    let voice = Voice::from(ctx, member.guild_id).await;
    if voice.is_alone(ctx).await.unwrap() {
        return voice.leave().await.unwrap();
    }
    if let speaker::Role::Bot = member.role(ctx).await {
        return println!("This is me(bot). My entering is ignored.");
    }
    if let Some(message) = greeting_word(&change, &member.user.name) {
        voice.speech(message).await
    }
}

fn greeting_word(change_of_states: &ChangeOfStates, name: &str) -> Option<SpeechMessage> {
    match change_of_states {
        ChangeOfStates::Stay => None,
        ChangeOfStates::Leave => Some("いってらっしゃい"),
        ChangeOfStates::Join => Some("いらっしゃい"),
    }
    .map(|message_prefix| SpeechMessage {
        value: format!("{}さん{}", name, message_prefix),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greeting_word() {
        let message = greeting_word(&ChangeOfStates::Join, "hoge").unwrap();
        let result = SpeechMessage {
            value: "hogeさんいらっしゃい".to_string(),
        };
        assert_eq!(result, message);
    }
}
