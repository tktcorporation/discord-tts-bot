use serenity::{client::Context, model};

use crate::constants;
use crate::model::HttpKey;

use super::Error;
use super::TrackTiming;
use songbird::input::YoutubeDl;

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

        let client = {
            let data = ctx.data.read().await;
            data.get::<HttpKey>().cloned().unwrap()
        };

        let mut ytdl = YoutubeDl::new_search(client, url.to_string());
        let res: songbird::input::AuxMetadata = match ytdl.search(Some(1)).await {
            Ok(res) => res[0].clone(),
            Err(why) => {
                println!("Err starting source: {why:?}");
                return Err(Error::AudioStreamError(why));
            }
        };

        super::send_track_info_message(
            TrackTiming::Added,
            Some(&res),
            channel_id,
            ctx.http.clone(),
        )
        .await;

        let audio = handler.enqueue_input(ytdl.into()).await;
        audio.set_volume(constants::volume::MUSIC).unwrap();

        Ok(())
    } else {
        Err(Error::NotInVoiceChannel)
    }
}
