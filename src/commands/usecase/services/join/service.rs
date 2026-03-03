use serenity::{
    self,
    client::Context,
    model::{id::ChannelId as SerenityChannelId, mention::Mention},
};

use crate::constants;
use crate::handler::usecase::text_to_speech::{config, speech_options};
use crate::infrastructure::SharedSoundPath;

use songbird::input::codecs;
use songbird::{self, Event, TrackEvent};

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

    if let Some(call) = manager.get(guild.id) {
        let handler = call.lock().await;
        if handler.current_connection().is_some() {
            return Err(Error::AlreadyJoined);
        }
        // Handler exists but no active connection — stale state from a previous
        // failed join.  Remove it so we can start fresh.
        drop(handler);
        let _ = manager.remove(guild.id).await;
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

    match manager.join(guild.id, connect_to).await {
        Ok(handle_lock) => {
            println!("join success to channel: {}", connect_to);
            _clear(&handle_lock).await;

            let mut handler = handle_lock.lock().await;
            if let Err(e) = handler.deafen(false).await {
                eprintln!("Error unmuting bot: {:?}", e);
            } else {
                println!("Successfully unmuted bot.");
            }

            _queue_join_message(&mut handler, ctx.http.clone(), called_channnel_id).await;

            Ok(format!("Joined {}", Mention::from(connect_to)))
        }
        Err(e) => {
            eprintln!("Failed to join voice channel: {e:?}");
            // Clean up the Call that songbird created internally to prevent
            // stale state from blocking future join attempts.
            let _ = manager.remove(guild.id).await;
            Err(Error::JoinError(e))
        }
    }
}

async fn _queue_join_message(
    handler: &mut songbird::Call,
    http: std::sync::Arc<serenity::http::Http>,
    text_channel_id: SerenityChannelId,
) {
    handler.add_global_event(
        Event::Track(TrackEvent::Play),
        TrackPlayNotifier::new(text_channel_id, http),
    );

    let input = welcome_audio().await;
    let audio = handler.enqueue_input(input).await;
    audio.set_volume(constants::volume::VOICE).unwrap();
}

async fn _clear(handle_lock: &std::sync::Arc<serenity::prelude::Mutex<songbird::Call>>) {
    let call = handle_lock.lock().await;
    call.queue().stop();
}

async fn welcome_audio() -> songbird::input::Input {
    let file_path = SharedSoundPath::new().welcome_audio_path();
    print!("file_path: {file_path:?}");

    let in_memory = match tokio::fs::read(file_path).await {
        Ok(in_memory) => in_memory,
        Err(e) => {
            println!("Error reading file: {e:?}");
            panic!();
        }
    };

    let in_memory_input: songbird::input::Input = songbird::input::Input::from(in_memory);
    match in_memory_input
        .make_playable_async(codecs::get_codec_registry(), codecs::get_probe())
        .await
    {
        Ok(playable) => playable,
        Err(e) => {
            println!("Error making input playable: {e:?}");
            panic!("Failed to make welcome audio playable");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_welcome_audio() {
        welcome_audio().await;
    }
}
