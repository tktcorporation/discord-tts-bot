use serenity::{client::Context, model};

use super::error::Error;

pub async fn queue(
    ctx: &Context,
    guild_id: model::id::GuildId,
) -> Result<std::vec::Vec<songbird::tracks::TrackHandle>, Error> {
    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;

        // Here, we use lazy restartable sources to make sure that we don't pay
        // for decoding, playback on tracks which aren't actually live yet.

        Ok(handler.queue().current_queue())
    } else {
        Err(Error::NotInVoiceChannel)
    }
}
