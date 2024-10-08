use std::fmt::Display;

use super::error::Error as ServiceError;
use serenity::{client::Context, model};

#[derive(Debug)]
pub enum SkipError {
    Error(ServiceError),
    ControlError(songbird::error::ControlError),
}
impl Display for SkipError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SkipError::Error(e) => e.fmt(f),
            SkipError::ControlError(e) => e.fmt(f),
        }
    }
}

pub async fn skip(ctx: &Context, guild_id: model::id::GuildId) -> Result<String, SkipError> {
    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;
        let queue = handler.queue();
        let m = match queue.current() {
            Some(_track) => "Skipped".to_string(),
            None => "Nothing to skip.".to_string(),
        };
        match queue.skip() {
            Ok(_) => Ok(m),
            Err(e) => Err(SkipError::ControlError(e)),
        }
    } else {
        Err(SkipError::Error(ServiceError::NotInVoiceChannel))
    }
}
