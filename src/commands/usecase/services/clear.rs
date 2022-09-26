use serenity::{client::Context, framework::standard::CommandResult, model::channel::Message};

use super::{check_msg, error::Error};

pub async fn clear(ctx: &Context, msg: &Message) -> CommandResult {
    let message = match _clear(ctx, msg).await {
        Ok(s) => s,
        Err(s) => format!("Error: {}", s),
    };
    check_msg(msg.reply(ctx, message).await);
    Ok(())
}

async fn _clear(ctx: &Context, msg: &Message) -> Result<String, Error> {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

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
            removed => format!("Removed **{}** titles from the queue!", removed),
        })
    } else {
        Err(Error::NotInVoiceChannel)
    }
}
