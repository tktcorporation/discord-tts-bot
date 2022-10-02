use serenity::{model, prelude::Context};

use super::error::Error;

pub async fn unmute(ctx: &Context, guild_id: model::id::GuildId) -> Result<(), Error> {
    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;
        if let Err(e) = handler.mute(false).await {
            return Err(Error::JoinError(e));
        }

        Ok(())
    } else {
        Err(Error::NotInVoiceChannel)
    }
}
