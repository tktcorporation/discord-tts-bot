use crate::tts::generate_speech_file;
use polly::model::VoiceId;
use serenity::{client::Context, model::id};
use songbird::ffmpeg;
use songbird::input::Input;
use std::ffi::OsStr;
use std::path::Path;
use tiger::digest::Digest;
use tiger::Tiger;

use songbird::{self, Songbird};

pub struct Voice {
    manager: std::sync::Arc<Songbird>,
    guild_id: id::GuildId,
}

impl Voice {
    pub async fn from(ctx: &Context, guild_id: id::GuildId) -> Voice {
        let manager = songbird::get(ctx)
            .await
            .expect("Songbird Voice client placed in at initialisation.");
        Voice { manager, guild_id }
    }

    async fn handler(
        &self,
    ) -> Result<std::sync::Arc<serenity::prelude::Mutex<songbird::Call>>, &str> {
        match self.manager.get(self.guild_id) {
            Some(handler) => Ok(handler),
            None => Err("not in voice channel"),
        }
    }

    pub async fn speech(&self, text: String) {
        match self.handler().await {
            Ok(handler) => {
                let file_path = _speech_file_path(&self.guild_id).await;
                let speech_file = generate_speech_file(
                    remove_mention_string(&text),
                    VoiceId::Mizuki,
                    file_path,
                    false,
                )
                .await
                .unwrap();
                let input = get_input_from_local(speech_file).await;
                play_input(&handler, input).await;
            }
            Err(str) => println!("{}", str),
        }
    }

    pub async fn members(
        &self,
        ctx: &Context,
    ) -> std::result::Result<std::vec::Vec<serenity::model::guild::Member>, String> {
        // TODO: nestが深いのを直したい
        match self.handler().await {
            Ok(handler) => match get_channel_id_and_guild_id(&handler).await {
                Ok(ids) => _members(ctx, &ids.0, &ids.1.unwrap()).await,
                Err(str) => Err(str.to_string()),
            },
            Err(str) => Err(str.to_string()),
        }
    }

    pub async fn leave(self) -> std::result::Result<(), songbird::error::JoinError> {
        self.manager.leave(self.guild_id).await
    }
}

async fn _speech_file_path(guild_id: &id::GuildId) -> std::path::PathBuf {
    use rand::Rng;

    let root = env!("CARGO_MANIFEST_DIR");
    let path = Path::new(root);
    let digest = Tiger::digest(guild_id.to_string().as_bytes());
    let guild_id_digest_str = format!("{:X}", digest);
    std::fs::create_dir_all(path.join("sounds").join(guild_id_digest_str.clone()))
        .expect("fail to create a dir of guild path");
    // guild ごとに最大5ファイル持つ
    let rand_num: i32 = rand::thread_rng().gen_range(0..4);
    path.join("sounds")
        .join(guild_id_digest_str)
        .join(rand_num.to_string())
}

async fn _members(
    ctx: &Context,
    guild_id: &songbird::id::GuildId,
    channel_id: &songbird::id::ChannelId,
) -> std::result::Result<std::vec::Vec<serenity::model::guild::Member>, String> {
    let guild_id = id::GuildId::from(guild_id.0);
    let channel_id = id::ChannelId::from(channel_id.0);
    let channels = guild_id.channels(&ctx.http.as_ref()).await.unwrap();
    match channels.get(&channel_id) {
        Some(guild_channel) => Ok(guild_channel.members(&ctx.cache).await.unwrap()),
        _ => Err("can't get a channel id".to_string()),
    }
}

async fn get_channel_id_and_guild_id(
    handler: &std::sync::Arc<serenity::prelude::Mutex<songbird::Call>>,
) -> Result<(songbird::id::GuildId, Option<songbird::id::ChannelId>), &'static str> {
    let handler_lock = handler.lock().await;
    if let Some(connection) = handler_lock.current_connection() {
        Ok((connection.guild_id, connection.channel_id))
    } else {
        Err("connection not found")
    }
}

fn remove_mention_string(content: &str) -> String {
    use regex::Regex;
    let re = Regex::new(r"<@![0-9]+>").unwrap();
    re.replace_all(content, "").to_string()
}

async fn play_input(
    handler_lock: &std::sync::Arc<serenity::prelude::Mutex<songbird::Call>>,
    input: Input,
) {
    let mut handler = handler_lock.lock().await;
    handler.enqueue_source(input);
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
