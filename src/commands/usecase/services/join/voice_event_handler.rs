use super::{send_track_info_message, TrackTiming};
use serenity::{async_trait, http::Http, model::prelude::ChannelId};
use songbird::{Event, EventContext, EventHandler as VoiceEventHandler};
use std::sync::Arc;

pub struct TrackPlayNotifier {
    /// text channel id
    chan_id: ChannelId,
    http: Arc<Http>,
}

impl TrackPlayNotifier {
    pub fn new(chan_id: ChannelId, http: Arc<Http>) -> Self {
        // You can manage state here, such as a buffer of audio packet bytes so
        // you can later store them in intervals.
        Self { chan_id, http }
    }
}

#[async_trait]
impl VoiceEventHandler for TrackPlayNotifier {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        if let EventContext::Track([(_current_track_state, current_track_handle)]) = ctx {
            send_track_info_message(
                TrackTiming::NowPlaying,
                current_track_handle.action(
                    |track| track.meta
                ),
                self.chan_id,
                self.http.clone(),
            )
            .await;
        }
        None
    }
}
