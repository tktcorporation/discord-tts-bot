use super::super::usecase::interface::Speaker;
use super::text_to_speech_message::SpeechMessage;
use crate::infrastructure;
pub use crate::model::Voice;
use polly::model::VoiceId;
use serenity::async_trait;
use songbird::ffmpeg;
use songbird::input::Input;
use std::ffi::OsStr;
mod tts;
use tts::generate_speech_file;

#[async_trait]
#[cfg_attr(feature = "mock", mockall::automock)]
impl Speaker for Voice {
    async fn speech(&self, msg: SpeechMessage) {
        match self.handler().await {
            Ok(handler) => {
                let root = env!("CARGO_MANIFEST_DIR");
                let file_path =
                    infrastructure::SoundFile::new(root).speech_file_path(&self.guild_id);
                let speech_file =
                    generate_speech_file(msg.value, VoiceId::Mizuki, file_path, false)
                        .await
                        .unwrap();
                let input = get_input_from_local(speech_file).await;
                play_input(&handler, input).await;
            }
            Err(str) => println!("{}", str),
        }
    }
}

async fn get_input_from_local<P: AsRef<OsStr>>(file_path: P) -> Input {
    return ffmpeg(file_path)
        .await
        .expect("This might fail: handle this error!");
}

async fn play_input(
    handler_lock: &std::sync::Arc<serenity::prelude::Mutex<songbird::Call>>,
    input: Input,
) {
    let mut handler = handler_lock.lock().await;
    handler.enqueue_source(input);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[tokio::test]
    async fn create_tts_file() {
        let root = env!("CARGO_MANIFEST_DIR");
        let path = Path::new(root);
        let file_path: infrastructure::SpeechFilePath = path.join("sounds").join("tts").into();
        let speech_file = generate_speech_file(
            "おはようございます".to_string(),
            VoiceId::Mizuki,
            file_path,
            false,
        )
        .await
        .unwrap();
        get_input_from_local(speech_file).await;
    }
}
