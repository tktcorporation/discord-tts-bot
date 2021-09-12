use super::super::model::text_to_speech_message::{Message, SpeechMessage};
use serenity::async_trait;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait Speaker {
    async fn speech(&self, msg: SpeechMessage);
}

pub async fn text_to_speech(speaker: Box<dyn Speaker + Sync + Send>, msg: Message) {
    if msg.is_ignore() {
        return;
    };
    speaker.speech(msg.to_speech_text()).await;
}
