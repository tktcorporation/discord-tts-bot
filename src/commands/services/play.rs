use serenity::{
    client::Context,
    framework::standard::{Args, CommandResult},
    model::channel::Message,
};

use super::check_msg;
use songbird::input::restartable::Restartable;

pub async fn play(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let comment = match _play(ctx, msg, args).await {
        Ok(s) => s,
        Err(s) => format!("Error: {}", s),
    };
    check_msg(msg.channel_id.say(&ctx.http, comment).await);
    Ok(())
}

async fn _play(ctx: &Context, msg: &Message, args: Args) -> Result<String, String> {
    let url = args.message();

    let guild = msg.guild(&ctx.cache).await.unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;

        // Here, we use lazy restartable sources to make sure that we don't pay
        // for decoding, playback on tracks which aren't actually live yet.
        let source = match source_from_str(url.to_string(), true).await {
            Ok(source) => source,
            Err(why) => {
                println!("Err starting source: {:?}", why);
                return Err(String::from("Error sourcing ffmpeg"));
            }
        };

        handler.enqueue_source(source.into());

        Ok(format!(
            "Added song to queue: position {}",
            handler.queue().len()
        ))
    } else {
        Err(String::from("Not in a voice channel to play in"))
    }
}

async fn source_from_str(
    value: String,
    lazy: bool,
) -> Result<Restartable, songbird::input::error::Error> {
    if value.starts_with("http") {
        Restartable::ytdl(value.clone(), lazy).await
    } else {
        Restartable::ytdl_search(value, lazy).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_source_from_str() {
        source_from_str("Sample".to_string(), false).await.unwrap();
    }

    #[tokio::test]
    async fn test_source_from_url() {
        source_from_str(
            "https://www.youtube.com/watch?v=rvkxtVkvawc".to_string(),
            false,
        )
        .await
        .unwrap();
    }

    #[tokio::test]
    async fn test_source_from_str_lazy() {
        source_from_str("Sample".to_string(), false).await.unwrap();
    }

    #[tokio::test]
    async fn test_source_from_url_lazy() {
        source_from_str(
            "https://www.youtube.com/watch?v=rvkxtVkvawc".to_string(),
            false,
        )
        .await
        .unwrap();
    }
}
