use serenity::{client::Context, model::id};
use songbird::id::GuildId;
use songbird::input::{error::Result, restartable::Restartable, Input};
use std::env;
use std::path::Path;
use std::sync::Arc;
use tiger::Tiger;

use crate::tts::generate_speech_file;
use polly::model::VoiceId;
use songbird::ffmpeg;
use std::ffi::OsStr;
use tiger::digest::Digest;

pub async fn get_handler_when_in_voice_channel<G: Into<GuildId>>(
    ctx: &Context,
    guild_id: G,
) -> Option<Arc<serenity::prelude::Mutex<songbird::Call>>> {
    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.");
    manager.get(guild_id)
}

pub async fn play_input(
    handler_lock: &std::sync::Arc<serenity::prelude::Mutex<songbird::Call>>,
    input: Input,
) {
    let mut handler = handler_lock.lock().await;
    // if let Some(handler_lock) = manager.get(guild_id) {
    //     let mut handler = handler_lock.lock().await;

    handler.enqueue_source(input);
}

pub async fn get_bgm_input() -> Result<Input> {
    let url = "https://youtu.be/16Bj6aPi1A8";
    match Restartable::ytdl(url, true).await {
        Ok(source) => Ok(source.into()),
        Err(why) => {
            println!("Err get input source: {:?}", why);

            Err(why)
        }
    }
}

pub async fn speech(
    text_for_speech: String,
    guild_id: id::GuildId,
    handler_lock: Arc<serenity::prelude::Mutex<songbird::Call>>,
) {
    let root = env!("CARGO_MANIFEST_DIR");
    let path = Path::new(root);
    let input = match text_for_speech.as_str() {
        "BGM" => get_bgm_input().await.unwrap(),
        _ => {
            // 同じファイル名だと複数サーバーで利用した場合に競合しそうなので、ユニークなファイル名を割り当てる
            let id = guild_id.0.to_string();
            let digest = Tiger::digest(id.as_bytes());
            let digest_str = format!("{:X}", digest);

            let file_path = path.join("sounds").join(digest_str);
            let speech_file =
                generate_speech_file(text_for_speech, VoiceId::Mizuki, file_path, false)
                    .await
                    .unwrap();
            get_input_from_local(speech_file).await
        }
    };
    play_input(&handler_lock, input).await;
}

async fn get_input_from_local<P: AsRef<OsStr>>(file_path: P) -> Input {
    return ffmpeg(file_path)
        .await
        .expect("This might fail: handle this error!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn path_exists() {
        let root = option_env!("CARGO_MANIFEST_DIR").unwrap();
        println!("{}", root);
        let path = Path::new(root);
        let file_path = path.join("sounds").join("2_23_AM_2.mp3");
        println!("{}", file_path.display());
        assert_eq!(true, file_path.exists());
    }

    #[tokio::test]
    async fn create_tts_file() {
        let root = option_env!("CARGO_MANIFEST_DIR").unwrap();
        let path = Path::new(root);
        let file_path = path.join("sounds").join("tts");
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

    #[test]
    fn digest_str() {
        let id = "99999999999999999999999999";
        let digest = Tiger::digest(id.as_bytes());
        let digest_str = format!("{:X}", digest);
        assert_eq!(
            digest_str,
            "7EABF4E47410D6A9FCF10B802CE754E5357120F7081B840B"
        );
    }
}
