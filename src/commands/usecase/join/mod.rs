use super::check_msg;
use serenity::{
    client::Context,
    model::{
        channel::Message as SerenityMessage, id, id::ChannelId as SerenityChannelId,
        misc::Mentionable,
    },
};
mod voice_event_handler;

use crate::infrastructure::SharedSoundPath;
pub use crate::model::{Message, Voice};

use songbird::{self, ffmpeg, Event, TrackEvent};

impl Voice {
    async fn join(
        &self,
        connect_to: id::ChannelId,
    ) -> (
        std::sync::Arc<tokio::sync::Mutex<songbird::Call>>,
        songbird::error::JoinResult<()>,
    ) {
        self.manager.join(self.guild_id, connect_to).await
    }
}

pub async fn join(ctx: &Context, msg: &SerenityMessage, joiner: Voice) -> Result<(), String> {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    let channel_id = guild
        .voice_states
        .get(&msg.author.id)
        .and_then(|voice_state| voice_state.channel_id);

    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            check_msg(msg.reply(ctx, "Not in a voice channel").await);

            return Ok(());
        }
    };

    let (handle_lock, success) = joiner.join(connect_to).await;
    let comment = match success {
        Ok(()) => {
            _clear(&handle_lock).await;
            _queue_join_message(handle_lock, ctx.http.clone(), msg.channel_id).await;
            format!("Joined {}", connect_to.mention())
        }
        Err(e) => e.to_string(),
    };

    check_msg(msg.channel_id.say(&ctx.http, &comment).await);
    Ok(())
}

async fn _queue_join_message(
    handle_lock: std::sync::Arc<serenity::prelude::Mutex<songbird::Call>>,
    http: std::sync::Arc<serenity::http::Http>,
    text_channel_id: SerenityChannelId,
) {
    let mut handle = handle_lock.lock().await;

    handle.add_global_event(
        Event::Track(TrackEvent::Play),
        voice_event_handler::TrackPlayNotifier::new(text_channel_id, http),
    );

    let input = welcome_audio().await;
    handle.enqueue_source(input)
}

async fn _clear(handle_lock: &std::sync::Arc<serenity::prelude::Mutex<songbird::Call>>) {
    let call = handle_lock.lock().await;
    call.queue().stop();
}

async fn welcome_audio() -> songbird::input::Input {
    let file_path = SharedSoundPath::new().welcome_audio_path();
    ffmpeg(file_path)
        .await
        .expect("This might fail: handle this error!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_welcome_audio() {
        welcome_audio().await;
    }
}

// #[async_trait]
// pub trait Joiner {
//     async fn join(
//         &self,
//         connect_to: id::ChannelId,
//     ) -> (
//         std::sync::Arc<tokio::sync::Mutex<songbird::Call>>,
//         songbird::error::JoinResult<()>,
//     );
//     async fn piin(
//         &self,
//     ) -> (String,
//         String
//     );
// }
