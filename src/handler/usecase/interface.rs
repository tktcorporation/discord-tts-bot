use super::text_to_speech::SpeechMessage;
use serenity::async_trait;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait Speaker {
    #[cfg(feature = "aws")]
    async fn speech(&self, msg: SpeechMessage);
    fn guild_id(&self) -> serenity::model::id::GuildId;
}

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait FilePath {
    async fn path(&self);
}
