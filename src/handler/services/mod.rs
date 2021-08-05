use serenity::{client::Context, model::channel::Message};
use songbird::input::{error::Result, restartable::Restartable, Input};
use std::sync::Arc;

pub async fn get_handler_when_in_voice_channel(
    ctx: &Context,
    msg: &Message,
) -> Option<Arc<serenity::prelude::Mutex<songbird::Call>>> {
    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();
    return manager.get(msg.guild(&ctx.cache).await.unwrap().id);
}

pub async fn play_input(
    handler_lock: &std::sync::Arc<serenity::prelude::Mutex<songbird::Call>>,
    input: Input,
) {
    let mut handler = handler_lock.lock().await;
    // if let Some(handler_lock) = manager.get(guild_id) {
    //     let mut handler = handler_lock.lock().await;

    handler.enqueue_source(input);
}

pub async fn get_bgm_input() -> Result<Input> {
    let url = "https://youtu.be/16Bj6aPi1A8";
    match Restartable::ytdl(url, true).await {
        Ok(source) => Ok(source.into()),
        Err(why) => {
            println!("Err get input source: {:?}", why);

            Err(why)
        }
    }
}
