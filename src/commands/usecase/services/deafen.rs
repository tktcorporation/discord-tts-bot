use serenity::{client::Context, model};

use super::error::Error;

pub async fn deafen(ctx: &Context, guild_id: model::id::GuildId) -> Result<String, Error> {
    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    let handler_lock = match manager.get(guild_id) {
        Some(handler) => handler,
        None => {
            return Err(Error::NotInVoiceChannel);
        }
    };

    let mut handler = handler_lock.lock().await;

    if handler.is_deaf() {
        return Ok("Already deafened".to_string());
    }
    if let Err(e) = handler.deafen(true).await {
        return Err(Error::JoinError(e));
    }

    Ok("Deafened".to_string())
}
