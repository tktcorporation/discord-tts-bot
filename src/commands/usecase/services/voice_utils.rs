use std::{sync::Arc, time::Duration};

use serenity::{http::Http, model::prelude::ChannelId};

pub enum TrackTiming {
    Added,
    NowPlaying,
}

pub fn get_human_readable_timestamp(duration: Option<Duration>) -> String {
    match duration {
        Some(duration) if duration == Duration::MAX => "∞".to_string(),
        Some(duration) => {
            let seconds = duration.as_secs() % 60;
            let minutes = (duration.as_secs() / 60) % 60;
            let hours = duration.as_secs() / 3600;

            if hours < 1 {
                format!("{minutes:02}:{seconds:02}")
            } else {
                format!("{hours}:{minutes:02}:{seconds:02}")
            }
        }
        None => "∞".to_string(),
    }
}

pub async fn send_track_info_message(
    timing: TrackTiming,
    metadata: &songbird::input::AuxMetadata,
    channel_id: ChannelId,
    http: Arc<Http>,
) {
    if let Some(source_url) = &metadata.source_url {
        if source_url.starts_with("soundboard_") {
            return;
        }
    } else {
        return;
    }
    channel_id
        .send_message(http, |m| {
            match timing {
                TrackTiming::Added => added_queue_embed(m, metadata.clone()),
                TrackTiming::NowPlaying => now_playing_embed(m, metadata.clone()),
            }
            m
        })
        .await
        .unwrap();
}

fn now_playing_embed(m: &mut serenity::builder::CreateMessage, np: songbird::input::Metadata) {
    m.embed(|e| {
        e.title("Now Playing");
        e.field("Title", np.title.clone().unwrap(), false);
        // if let Some(t) = np.source_url {
        //     e.field("URL", t, false);
        // }
        e.field("Duration", get_human_readable_timestamp(np.duration), true);
        // e.field("Requester", np.requester.mention(), true);
        if let Some(t) = np.thumbnail {
            e.thumbnail(t);
        }
        e
    });
}

fn added_queue_embed(m: &mut serenity::builder::CreateMessage, np: songbird::input::Metadata) {
    m.embed(|e| {
        e.title("Added Queue");
        e.field("Title", np.title.clone().unwrap(), false);
        // if let Some(t) = np.source_url {
        //     e.field("URL", t, false);
        // }
        e.field("Duration", get_human_readable_timestamp(np.duration), true);
        // e.field("Requester", np.requester.mention(), true);
        if let Some(t) = np.thumbnail {
            e.thumbnail(t);
        }
        e
    });
}
