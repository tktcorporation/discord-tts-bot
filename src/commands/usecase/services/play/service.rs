use serenity::{client::Context, model};

use crate::constants;

use super::Error;
use super::TrackTiming;
use reqwest;
use songbird::{
    driver::Driver,
    input::{codecs::*, Compose, Input, MetadataError, YoutubeDl},
    tracks::Track,
};

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

        let mut ytdl = YoutubeDl::new_search(reqwest::Client::new(), url.to_string());
        let res: songbird::input::AuxMetadata = match ytdl.search(Some(1)).await {
            Ok(res) => res[0].clone(),
            Err(why) => {
                println!("Err starting source: {why:?}");
                return Err(Error::AudioStreamError(why));
            }
        };

        super::send_track_info_message(TrackTiming::Added, &res, channel_id, ctx.http.clone())
            .await;

        let audio = handler.enqueue_input(ytdl.into()).await;
        audio.set_volume(constants::volume::MUSIC);

        Ok(())
    } else {
        Err(Error::NotInVoiceChannel)
    }
}
