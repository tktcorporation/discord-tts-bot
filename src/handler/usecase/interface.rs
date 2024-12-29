use super::text_to_speech::SpeechMessage;
use serenity::async_trait;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait Speaker {
    #[cfg(feature = "aws")]
    async fn speech(&self, msg: SpeechMessage) -> Result<(), String>;
    fn guild_id(&self) -> serenity::model::id::GuildId;
}

#[cfg_attr(test, mockall::automock)]
#[async_trait]
#[allow(dead_code)]
pub trait FilePath {
    async fn path(&self);
}
