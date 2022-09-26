use serenity::{http::Http, model::prelude::ChannelId};
use std::sync::Arc;
use tokio::time::Duration;

pub enum TrackTiming {
    Added,
    NowPlaying,
}

pub async fn send_track_info_message(
    timing: TrackTiming,
    metadata: &songbird::input::Metadata,
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
        e.field("Duration", format_duration(np.duration), true);
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
        e.field("Duration", format_duration(np.duration), true);
        // e.field("Requester", np.requester.mention(), true);
        if let Some(t) = np.thumbnail {
            e.thumbnail(t);
        }
        e
    });
}

fn format_duration(duration: Option<Duration>) -> String {
    if let Some(d) = duration {
        if d != Duration::default() {
            return d.as_secs().to_string();
        }
    }
    "Live".to_string()
}
