use std::{sync::Arc, time::Duration};

use serenity::{http::Http, model::prelude::ChannelId};
use serenity::builder::{CreateEmbed, CreateMessage};

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
    _metadata: Option<&songbird::input::AuxMetadata>,
    channel_id: ChannelId,
    http: Arc<Http>,
) {
    // TODO: 再生曲の情報を埋め込む
    let builder = CreateMessage::default()
        .embed(
            CreateEmbed::default()
                .title(match timing {
                    TrackTiming::Added => "Added Queue",
                    TrackTiming::NowPlaying => "Now Playing",
                })
        );
    channel_id
        .send_message(http, builder)
        .await
        .unwrap();
}
