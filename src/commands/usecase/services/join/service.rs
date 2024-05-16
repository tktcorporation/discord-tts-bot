use serenity::{
    self,
    client::Context,
    model::{id::ChannelId as SerenityChannelId, mention::Mention},
};

use crate::constants;
use crate::handler::usecase::text_to_speech::{config, speech_options};
use crate::infrastructure::SharedSoundPath;

use songbird::{self, create_player, ffmpeg, Event, TrackEvent};

use super::{voice_event_handler::TrackPlayNotifier, Error};

pub async fn join(
    ctx: &Context,
    guild: serenity::model::guild::Guild,
    caller_id: &serenity::model::id::UserId,
    called_channnel_id: serenity::model::id::ChannelId,
    speech_options: speech_options::SpeechOptions,
) -> Result<String, Error> {
    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.");

    let has_handler = manager.get(guild.id).is_some();
    if has_handler {
        return Err(Error::AlreadyJoined);
    }

    // voice settings
    let client = config::client::new(crate::infrastructure::GuildPath::new(&guild.id));
    client.write(config::Config { speech_options });

    let channel_id = guild
        .voice_states
        .get(caller_id)
        .and_then(|voice_state| voice_state.channel_id);

    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            return Err(Error::NotInVoiceChannel);
        }
    };

    let (handle_lock, success) = manager.join(guild.id, connect_to).await;
    match success {
        Ok(()) => {
            _clear(&handle_lock).await;
            _queue_join_message(handle_lock, ctx.http.clone(), called_channnel_id).await;
            Ok(format!("Joined {}", Mention::from(connect_to)))
        }
        Err(e) => Err(Error::JoinError(e)),
    }
}

async fn _queue_join_message(
    handle_lock: std::sync::Arc<serenity::prelude::Mutex<songbird::Call>>,
    http: std::sync::Arc<serenity::http::Http>,
    text_channel_id: SerenityChannelId,
) {
    let mut handle = handle_lock.lock().await;

    handle.add_global_event(
        Event::Track(TrackEvent::Play),
        TrackPlayNotifier::new(text_channel_id, http),
    );

    let input = welcome_audio().await;
    let (mut audio, _audio_handle) = create_player(input);
    audio.set_volume(constants::volume::VOICE);
    handle.enqueue(audio);
}

async fn _clear(handle_lock: &std::sync::Arc<serenity::prelude::Mutex<songbird::Call>>) {
    let call = handle_lock.lock().await;
    call.queue().stop();
}

async fn welcome_audio() -> songbird::input::Input {
    let file_path = SharedSoundPath::new().welcome_audio_path();
    print!("file_path: {:?}", file_path);
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
