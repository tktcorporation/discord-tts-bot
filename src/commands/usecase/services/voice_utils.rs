use std::{sync::Arc, time::Duration};

use serenity::builder::{CreateEmbed, CreateMessage};
use serenity::{http::Http, model::prelude::ChannelId};

pub enum TrackTiming {
    Added,
    NowPlaying,
}

#[allow(dead_code)]
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
    metadata: Option<&songbird::input::AuxMetadata>,
    channel_id: ChannelId,
    http: Arc<Http>,
) {
    if let Some(metadata) = metadata {
        let title = match &metadata.title {
            Some(title) => title,
            None => "Unknown",
        };
        let builder = CreateMessage::default().embed(CreateEmbed::default().title(match timing {
            TrackTiming::Added => format!("Added to queue - {}", title),
            TrackTiming::NowPlaying => format!("Now playing - {}", title),
        }));
        channel_id.send_message(http, builder).await.unwrap();
    }
}
