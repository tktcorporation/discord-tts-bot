use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
};

use super::services::check_msg;

#[command]
#[only_in(guilds)]
async fn bgm(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    use songbird::input::{error::Result, restartable::Restartable, Input};
    pub async fn get_bgm_input() -> Result<Input> {
        let url = "https://youtu.be/16Bj6aPi1A8";
        match Restartable::ytdl(url, true).await {
            Ok(source) => Ok(source.into()),
            Err(why) => {
                println!("Err get input source: {why:?}");

                Err(why)
            }
        }
    }
    let input = get_bgm_input().await.unwrap();

    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;

        handler.enqueue_source(input);

        check_msg(
            msg.channel_id
                .say(
                    &ctx.http,
                    format!("Added song to queue: position {}", handler.queue().len()),
                )
                .await,
        );
    } else {
        check_msg(
            msg.channel_id
                .say(&ctx.http, "Not in a voice channel to play in")
                .await,
        );
    }

    Ok(())
}
