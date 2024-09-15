use serenity::async_trait;
use serenity::{http::Http, model};
use songbird::{Event, EventContext, EventHandler as VoiceEventHandler};

use std::sync::Arc;

struct SongEndNotifier {
    channel_id: model::id::ChannelId,
    http: Arc<Http>,
}
#[async_trait]
impl VoiceEventHandler for SongEndNotifier {
    async fn act(&self, _ctx: &EventContext<'_>) -> Option<Event> {
        self.channel_id
            .say(&self.http, "Song faded out completely!")
            .await
            .unwrap();

        None
    }
}

struct SongFader {
    channel_id: model::id::ChannelId,
    http: Arc<Http>,
}

#[async_trait]
impl VoiceEventHandler for SongFader {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        if let EventContext::Track(&[(state, track)]) = ctx {
            let _ = track.set_volume(state.volume / 2.0);

            if state.volume < 1e-2 {
                let _ = track.stop();
                self.channel_id
                    .say(&self.http, "Stopping song...")
                    .await
                    .unwrap();
                Some(Event::Cancel)
            } else {
                self.channel_id
                    .say(&self.http, "Volume reduced.")
                    .await
                    .unwrap();
                None
            }
        } else {
            None
        }
    }
}
