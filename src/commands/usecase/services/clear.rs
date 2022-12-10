use serenity::{client::Context, model::prelude::GuildId};

use super::error::Error;

pub async fn clear(ctx: &Context, guild_id: GuildId) -> Result<String, Error> {
    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let cleared = {
            let call = handler_lock.lock().await;
            let queue = call.queue();
            let len = queue.len();

            queue.stop();
            len
        };

        Ok(match cleared {
            0 => "There is nothing playing!".into(),
            1 => "Removed **1** title from the queue!".into(),
            removed => format!("Removed **{removed}** titles from the queue!"),
        })
    } else {
        Err(Error::NotInVoiceChannel)
    }
}
