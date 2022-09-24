use serenity::{client::Context, framework::standard::CommandResult, model::channel::Message};

use super::{check_msg, error::Error};

pub async fn queue(ctx: &Context, msg: &Message) -> CommandResult {
    match _queue(ctx, msg).await {
        Ok(queue) => {
            msg.channel_id
                .send_message(ctx.http.clone(), |m| {
                    m.embed(|e| {
                        e.title("List Queue");
                        for (i, val) in queue.iter().enumerate() {
                            e.field(
                                ".",
                                format!(
                                    "`{}` {}",
                                    i + 1,
                                    val.metadata().title.as_ref().unwrap_or(&String::from(""))
                                ),
                                false,
                            );
                        }
                        e
                    });
                    m
                })
                .await
                .unwrap();
        }
        Err(s) => check_msg(msg.channel_id.say(&ctx.http, format!("Error: {}", s)).await),
    };
    Ok(())
}

async fn _queue(
    ctx: &Context,
    msg: &Message,
) -> Result<std::vec::Vec<songbird::tracks::TrackHandle>, Error> {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

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
