use super::error::Error;
use serenity::{client::Context, model};

pub async fn mute(ctx: &Context, guild_id: model::id::GuildId) -> Result<String, Error> {
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

    if handler.is_mute() {
        return Ok("Already muted".to_string());
    }
    if let Err(e) = handler.mute(true).await {
        return Err(Error::JoinError(e));
    }

    Ok("Now muted".to_string())
}
