use serenity::{builder::CreateEmbed, client::Context, model};

use super::error::Error;

use songbird::tracks::TrackHandle;

use std::fmt::Write;

pub async fn queue(
    ctx: &Context,
    guild_id: model::id::GuildId,
) -> Result<std::vec::Vec<songbird::tracks::TrackHandle>, Error> {
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

pub fn create_queue_embed(tracks: &[songbird::tracks::TrackHandle], page: usize) -> CreateEmbed {
    let description = if !tracks.is_empty() {
        "TODO: implement queue embed".to_string()
    } else {
        String::from("Nothing is playing!")
    };

    CreateEmbed::default()
        .field("🔊 Now playing", &description, false)
        .field("⌛ Up next", build_queue_page(tracks, page), false)
}

fn build_queue_page(tracks: &[TrackHandle], page: usize) -> String {
    let embed_page_size = 10;
    let start_idx = embed_page_size * page;
    let queue: Vec<&TrackHandle> = tracks
        .iter()
        .skip(start_idx + 1)
        .take(embed_page_size)
        .collect();

    if queue.is_empty() {
        return String::from("There's no songs up next!");
    }

    let mut description = String::new();

    for (i, t) in queue.iter().enumerate() {
        // let title = t.metadata().title.as_ref();
        // let url = t.metadata().source_url.as_ref();
        // let duration = super::voice_utils::get_human_readable_timestamp(t.metadata().duration);

        let _ = writeln!(
            description,
            "`{}.` [{}]({}) • `{}`",
            i + start_idx + 1,
            &String::from("Unknown"),
            &String::from("Unknown"),
            &String::from("Unknown"),
        );
    }

    description
}
