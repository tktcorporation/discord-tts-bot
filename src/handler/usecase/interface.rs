use super::super::model::text_to_speech_message::SpeechMessage;
use serenity::async_trait;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait Speaker {
    async fn speech(&self, msg: SpeechMessage);
}
