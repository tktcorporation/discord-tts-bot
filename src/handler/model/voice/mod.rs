mod path;
mod tts;
use super::super::usecase::interface::Speaker;
use super::text_to_speech_message::SpeechMessage;
use crate::infrastructure;
use polly::model::VoiceId;
use serenity::{async_trait, client::Context, model::id};
use songbird::ffmpeg;
use songbird::input::Input;
use std::ffi::OsStr;
use tts::generate_speech_file;

use songbird::{self, Songbird};

pub struct Voice {
    manager: std::sync::Arc<Songbird>,
    guild_id: id::GuildId,
}

// pub struct Leaved(bool);
// impl Into<bool> for Leaved {
//     fn into(self) -> bool {
//         self.0
//     }
// }

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

    pub async fn is_alone(&self, ctx: &Context) -> Result<bool, String> {
        match self.members(ctx).await {
            Ok(members) => Ok(members.len() <= 1),
            Err(str) => Err(str),
        }
    }

    pub async fn leave(&self) -> std::result::Result<(), songbird::error::JoinError> {
        self.manager.leave(self.guild_id).await
    }
}

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
