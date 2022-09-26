use serenity::{client::Context, model};

use super::super::service::{send_track_info_message, TrackTiming};
use super::error::Error;
use songbird::input::{restartable::Restartable, Input};
use songbird::tracks::create_player;

pub async fn play(
    ctx: &Context,
    guild_id: model::id::GuildId,
    channel_id: model::id::ChannelId,
    url: &str,
) -> Result<(), Error> {
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
                return Err(Error::ErrorSourcingFfmpeg);
            }
        };
        let input: Input = source.into();
        send_track_info_message(
            TrackTiming::Added,
            input.metadata.as_ref(),
            channel_id,
            ctx.http.clone(),
        )
        .await;

        let (mut audio, _audio_handle) = create_player(input);
        audio.set_volume(0.1);
        handler.enqueue(audio);

        Ok(())
    } else {
        Err(Error::NotInVoiceChannel)
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
