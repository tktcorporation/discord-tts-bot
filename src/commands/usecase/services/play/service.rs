use serenity::{client::Context, model};

use super::utils;
use super::Error;
use super::TrackTiming;
use songbird::input::Input;
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
        let source = match utils::source_from_str(url.to_string(), true).await {
            Ok(source) => source,
            Err(why) => {
                println!("Err starting source: {:?}", why);
                return Err(Error::ErrorSourcingFfmpeg);
            }
        };
        let input: Input = source.into();
        super::send_track_info_message(
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
