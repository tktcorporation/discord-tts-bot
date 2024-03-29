use serenity::async_trait;
use serenity::{client::Context, http::Http, model};
use songbird::{Event, EventContext, EventHandler as VoiceEventHandler, TrackEvent};

use crate::constants;

use super::{check_msg, utils, Error};
use songbird::tracks::create_player;
use std::{sync::Arc, time::Duration};

struct SongEndNotifier {
    channel_id: model::id::ChannelId,
    http: Arc<Http>,
}
#[async_trait]
impl VoiceEventHandler for SongEndNotifier {
    async fn act(&self, _ctx: &EventContext<'_>) -> Option<Event> {
        check_msg(
            self.channel_id
                .say(&self.http, "Song faded out completely!")
                .await,
        );

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
                check_msg(self.channel_id.say(&self.http, "Stopping song...").await);
                Some(Event::Cancel)
            } else {
                check_msg(self.channel_id.say(&self.http, "Volume reduced.").await);
                None
            }
        } else {
            None
        }
    }
}

pub async fn play_fade(
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

        let source = match utils::source_from_str(url.to_string(), true).await {
            Ok(source) => source,
            Err(why) => {
                println!("Err starting source: {why:?}");
                return Err(Error::ErrorSourcingFfmpeg);
            }
        };

        // This handler object will allow you to, as needed,
        // control the audio track via events and further commands.
        let (mut audio, track_handle) = create_player(source.into());
        audio.set_volume(constants::volume::MUSIC);
        handler.enqueue(audio);
        let send_http = ctx.http.clone();

        // This shows how to periodically fire an event, in this case to
        // periodically make a track quieter until it can be no longer heard.
        let _ = track_handle.add_event(
            Event::Periodic(Duration::from_secs(5), Some(Duration::from_secs(7))),
            SongFader {
                channel_id,
                http: send_http,
            },
        );

        let send_http = ctx.http.clone();

        // This shows how to fire an event once an audio track completes,
        // either due to hitting the end of the bytestream or stopped by user code.
        let _ = track_handle.add_event(
            Event::Track(TrackEvent::End),
            SongEndNotifier {
                channel_id,
                http: send_http,
            },
        );

        Ok(())
    } else {
        Err(Error::NotInVoiceChannel)
    }
}
