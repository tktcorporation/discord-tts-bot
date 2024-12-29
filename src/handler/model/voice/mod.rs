use super::super::usecase::{interface::Speaker, text_to_speech::SpeechMessage};
use crate::constants;
#[cfg(feature = "aws")]
use crate::infrastructure::tts::generate_speech_file;
use crate::infrastructure::{GuildPath, SoundPath, SpeechFilePath};
pub use crate::model::Voice;
use polly::types::VoiceId;
use serenity::async_trait;
use songbird::input::Input;
use tracing;

#[async_trait]
#[cfg_attr(feature = "mock", mockall::automock)]
impl Speaker for Voice {
    #[cfg(feature = "aws")]
    async fn speech(&self, msg: SpeechMessage) {
        if let Ok(handler) = self.handler().await {
            let file_path = SpeechFilePath::new(SoundPath::new(GuildPath::new(&self.guild_id)));
            let speech_file =
                match generate_speech_file(&msg.value, VoiceId::Mizuki, &file_path, false).await {
                    Ok(file) => file,
                    Err(e) => {
                        tracing::error!("Failed to generate speech file: {:?}", e);
                        return;
                    }
                };
            let input = get_input_from_local(speech_file).await;
            println!("play_input: {:?}", msg.value);
            play_input(&handler, input).await;
        }
    }
    fn guild_id(&self) -> serenity::model::id::GuildId {
        self.guild_id
    }
}

async fn get_input_from_local(file_path: String) -> Input {
    use songbird::input::codecs::{CODEC_REGISTRY, PROBE};
    let in_memory = tokio::fs::read(file_path).await.unwrap();
    let in_memory_input: songbird::input::Input = songbird::input::Input::from(in_memory);
    in_memory_input
        .make_playable_async(&CODEC_REGISTRY, &PROBE)
        .await
        .unwrap()
}

async fn play_input(
    handler_lock: &std::sync::Arc<serenity::prelude::Mutex<songbird::Call>>,
    input: Input,
) {
    let mut handler = handler_lock.lock().await;

    let audio = handler.enqueue_input(input).await;
    audio.set_volume(constants::volume::VOICE).unwrap();
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::infrastructure::SharedSoundPath;

//     #[tokio::test]
//     async fn test_get_input_from_local() {
//         let file_path = SharedSoundPath::new().welcome_audio_path();
//         get_input_from_local(file_path).await;
//     }
// }
