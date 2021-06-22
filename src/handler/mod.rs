use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::{channel::Message, gateway::Ready},
};
use songbird::{
    ffmpeg,
    input::{error::Result, restartable::Restartable, Input},
    tracks::Track,
};
use std::path::Path;
use std::sync::Arc;
pub mod services;
use services::{get_handler_when_in_voice_channel, play_input};

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

        // サーバーのID
        eprintln!("guild_id = {:?}", msg.guild_id);
        // チャンネル名
        let channel_name = msg.channel_id.name(&ctx.cache).await;
        eprintln!("channel_name = {:?}", channel_name);
        // メッセージの送信
        let content = msg.content.clone();
        println!("message received: {:?}", content);

        let handler_lock = get_handler_when_in_voice_channel(&ctx, &msg).await.unwrap();

        let ma = msg.content.clone();
        match ma.as_str() {
            "BGM" => {
                let input = get_input_from_local().await;
                play_input(&handler_lock, input).await;
            }
            _ => {}
        };
    }
}

async fn get_input_from_local() -> Input {
    let root = option_env!("CARGO_MANIFEST_DIR").unwrap();
    let path = Path::new(root);
    let file_path = path.join("binaries").join("2_23_AM_2.mp3");
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
        let file_path = path.join("binaries").join("2_23_AM_2.mp3");
        println!("{}", file_path.display());
        assert_eq!(true, file_path.exists());
    }
}
