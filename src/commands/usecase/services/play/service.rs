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

        let mut ytdl_source = YoutubeDl::new_search(client.clone(), url.to_string());
        let search_results = match ytdl_source.search(Some(1)).await {
            Ok(iter) => iter.collect::<Vec<_>>(),
            Err(why) => {
                println!("Error searching with YoutubeDl: {why:?}");
                return Err(Error::PlayError(format!(
                    "Error searching with YoutubeDl: {why:?}"
                )));
            }
        };

        if search_results.is_empty() {
            println!("No search results found for url: {url}");
            return Err(Error::PlayError(format!(
                "No search results found for: {url}"
            )));
        }
        let metadata = search_results[0].clone();

        super::send_track_info_message(
            TrackTiming::Added,
            Some(&metadata),
            channel_id,
            ctx.http.clone(),
        )
        .await;

        let input_source = match metadata.source_url.as_ref() {
            Some(actual_url) => YoutubeDl::new(client.clone(), actual_url.to_string()).into(),
            None => {
                println!("No source_url found in metadata for: {url}. Falling back to original url/query for Input.");
                // If no direct URL, fall back to using the original query/URL with a new YoutubeDl instance
                // This is a simplified fallback; robust handling might require checking if 'url' is a playlist etc.
                YoutubeDl::new(client, url.to_string()).into()
            }
        };

        let audio_track = handler.enqueue_input(input_source).await;
        audio_track.set_volume(constants::volume::MUSIC).unwrap();

        Ok(())
    } else {
        Err(Error::NotInVoiceChannel)
    }
}
