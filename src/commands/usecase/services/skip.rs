use std::fmt::Display;

use super::error::Error as ServiceError;
use serenity::{client::Context, model};

#[derive(Debug)]

pub enum SkipError {
    Error(ServiceError),
    TrackError(songbird::tracks::TrackError),
}
impl Display for SkipError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SkipError::Error(e) => e.fmt(f),
            SkipError::TrackError(e) => e.fmt(f),
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
        let m = format!("Song skipped: {:?}", queue.current());
        match queue.skip() {
            Ok(_) => Ok(m),
            Err(e) => Err(SkipError::TrackError(e)),
        }
    } else {
        Err(SkipError::Error(ServiceError::NotInVoiceChannel))
    }
}
