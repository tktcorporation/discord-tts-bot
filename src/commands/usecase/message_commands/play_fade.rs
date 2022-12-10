use std::sync::Arc;

use serenity::{
    async_trait,
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    http::Http,
    model::{channel::Message, prelude::ChannelId},
};

use songbird::{Event, EventContext, EventHandler as VoiceEventHandler};

use super::services::{self, check_msg};

#[async_trait]
impl VoiceEventHandler for SongEndNotifier {
    async fn act(&self, _ctx: &EventContext<'_>) -> Option<Event> {
        check_msg(
            self.chan_id
                .say(&self.http, "Song faded out completely!")
                .await,
        );

        None
    }
}

struct SongEndNotifier {
    chan_id: ChannelId,
    http: Arc<Http>,
}

struct SongFader {
    chan_id: ChannelId,
    http: Arc<Http>,
}

#[async_trait]
impl VoiceEventHandler for SongFader {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        if let EventContext::Track(&[(state, track)]) = ctx {
            let _ = track.set_volume(state.volume / 2.0);

            if state.volume < 1e-2 {
                let _ = track.stop();
                check_msg(self.chan_id.say(&self.http, "Stopping song...").await);
                Some(Event::Cancel)
            } else {
                check_msg(self.chan_id.say(&self.http, "Volume reduced.").await);
                None
            }
        } else {
            None
        }
    }
}

#[command]
#[only_in(guilds)]
async fn play_fade(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let play_url = args.message();
    if let Err(e) = services::play_fade(ctx, guild.id, msg.channel_id, play_url).await {
        check_msg(msg.reply(&ctx.http, format!("Error: {e:?}")).await);
        return Ok(());
    }
    check_msg(msg.reply(&ctx, "Playing song...").await);
    Ok(())
}
