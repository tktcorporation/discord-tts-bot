use super::error::Error;
use serenity::{client::Context, model};
use songbird::tracks::{LoopState, TrackHandle};

pub async fn repeat(ctx: &Context, guild_id: model::id::GuildId) -> Result<bool, Error> {
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

    let handler = handler_lock.lock().await;
    let track = match handler.queue().current() {
        Some(track) => track,
        None => return Err(Error::NotInVoiceChannel),
    };

    let was_looping = match track.get_info().await {
        Ok(info) => info.loops == LoopState::Infinite,
        Err(_) => false,
    };
    let toggler = if was_looping {
        TrackHandle::disable_loop
    } else {
        TrackHandle::enable_loop
    };

    match toggler(&track) {
        Ok(_) if was_looping => Ok(false),
        Ok(_) => Ok(true),
        Err(e) => Err(Error::ControlError(e)),
    }
}
