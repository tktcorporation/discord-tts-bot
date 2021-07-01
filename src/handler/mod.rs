use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::{channel::Message, gateway::Ready},
};
use songbird::{ffmpeg, input::Input};
use std::ffi::OsStr;
use std::path::Path;
pub mod services;
use crate::tts::generate_speech_file;
use polly::model::VoiceId;
use services::{get_handler_when_in_voice_channel, play_input};
use tiger::digest::Digest;
use tiger::Tiger;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }

    async fn message(&self, ctx: Context, msg: Message) {
        // botに反応しないようにする
        if msg.author.bot {
            return;
        }

        // voice channel にいない場合は動かさない
        if get_handler_when_in_voice_channel(&ctx, &msg)
            .await
            .is_none()
        {
            return;
        }

        let is_debug = false;
        if is_debug {
            // サーバーのID
            eprintln!("guild_id = {:?}", msg.guild_id);
            // チャンネル名
            let channel_name = msg.channel_id.name(&ctx.cache).await;
            eprintln!("channel_name = {:?}", channel_name);
            // メッセージの送信
            let content = msg.content.clone();
            println!("message received: {:?}", content);
        }

        let handler_lock = get_handler_when_in_voice_channel(&ctx, &msg).await.unwrap();

        let root = env!("CARGO_MANIFEST_DIR");
        let path = Path::new(root);

        // url に反応しないようにする
        let text_for_speech = if msg.content.starts_with("http") {
            "url".to_string()
        } else {
            msg.content.clone()
        };

        let input = match text_for_speech.as_str() {
            "BGM" => services::get_bgm_input().await.unwrap(),
            _ => {
                // 同じファイル名だと複数サーバーで利用した場合に競合しそう
                let id = msg.guild_id.unwrap().0.to_string();
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
