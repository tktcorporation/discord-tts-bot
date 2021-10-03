use serenity::{client::Context, framework::standard::CommandResult, model::channel::Message};

use super::check_msg;

pub async fn queue(ctx: &Context, msg: &Message) -> CommandResult {
    let comment = match _queue(ctx, msg).await {
        Ok(s) => s,
        Err(s) => format!("Error: {}", s),
    };
    check_msg(msg.channel_id.say(&ctx.http, comment).await);
    Ok(())
}

async fn _queue(ctx: &Context, msg: &Message) -> Result<String, String> {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;

        // Here, we use lazy restartable sources to make sure that we don't pay
        // for decoding, playback on tracks which aren't actually live yet.

        Ok(format!("Queue: {:?}", handler.queue()))
    } else {
        Err(String::from("Not in a voice channel"))
    }
}
