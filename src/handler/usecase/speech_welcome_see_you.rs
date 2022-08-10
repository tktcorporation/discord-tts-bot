use super::interface::Speaker;

use super::super::model::{speaker::ChangeOfStates, voice::Voice};
use super::text_to_speech::SpeechMessage;
#[cfg(feature = "tts")]
use serenity::model::prelude::User;

use serenity::client::Context;

pub async fn speech_greeting(ctx: &Context, voice: &Voice, change: &ChangeOfStates, user: &User) {
    let name = match user.nick_in(ctx, voice.guild_id()).await {
        Some(n) => n,
        None => user.name.clone(),
    };
    if let Some(message) = greeting_word(change, &name) {
        voice.speech(message).await
    }
}

pub async fn leave_if_alone(ctx: &Context, voice: &Voice) {
    if voice.is_alone(ctx).await.unwrap() {
        voice.leave().await.unwrap()
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
