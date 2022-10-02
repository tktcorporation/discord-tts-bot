use super::super::usecase::{interface::Speaker, text_to_speech::SpeechMessage};
#[cfg(feature = "aws")]
use crate::infrastructure::tts::generate_speech_file;
use crate::infrastructure::{GuildPath, SoundPath, SpeechFilePath};
pub use crate::model::{voice::Error, Voice};
use polly::model::VoiceId;
use serenity::async_trait;
use songbird::ffmpeg;
use songbird::input::Input;
use songbird::tracks::create_player;
use std::ffi::OsStr;

#[async_trait]
#[cfg_attr(feature = "mock", mockall::automock)]
impl Speaker for Voice {
    #[cfg(feature = "aws")]
    async fn speech(&self, msg: SpeechMessage) {
        match self.handler().await {
            Ok(handler) => {
                let file_path = SpeechFilePath::new(SoundPath::new(GuildPath::new(&self.guild_id)));
                let speech_file =
                    generate_speech_file(msg.value, VoiceId::Mizuki, file_path, false)
                        .await
                        .unwrap();
                let input = get_input_from_local(speech_file).await;
                play_input(&handler, input).await;
            }
            Err(e) => match e {
                Error::ConnectionNotFound => (),
                Error::NotInVoiceChannel => (),
            },
        }
    }
    fn guild_id(&self) -> serenity::model::id::GuildId {
        self.guild_id
    }
}

async fn get_input_from_local<P: AsRef<OsStr>>(file_path: P) -> Input {
    ffmpeg(file_path)
        .await
        .expect("This might fail: handle this error!")
}

async fn play_input(
    handler_lock: &std::sync::Arc<serenity::prelude::Mutex<songbird::Call>>,
    input: Input,
) {
    let mut handler = handler_lock.lock().await;
    let (mut audio, _audio_handle) = create_player(input);
    audio.set_volume(1.0);
    handler.enqueue(audio);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::SharedSoundPath;

    #[tokio::test]
    async fn test_get_input_from_local() {
        let file_path = SharedSoundPath::new().welcome_audio_path();
        get_input_from_local(file_path).await;
    }
}
