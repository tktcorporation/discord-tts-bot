use std::env;

use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::{
        channel::Message,
        gateway::{Activity, Ready},
        voice,
    },
};
pub mod services;
use services::{get_handler_when_in_voice_channel, speech};
mod model;
use model::CurrentVoiceState;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        ctx.set_activity(Activity::playing(
            env::var("DISCORD_CMD_PREFIX").expect("Expected a command prefix in the environment")
                + "join で呼んでね",
        ))
        .await;
    }

    async fn message(&self, ctx: Context, msg: Message) {
        let guild_id = msg.guild(&ctx.cache).await.unwrap().id;
        let handler_lock = get_handler_when_in_voice_channel(&ctx, guild_id).await;
        if is_ignore_msg(&msg, &handler_lock).await {
            return;
        };

        let is_debug = false;
        if is_debug {
            debug_print(&msg, &ctx).await;
        };

        // url に反応しないようにする
        let text_for_speech = if msg.content.contains("http") {
            "url".to_string()
        } else {
            msg.content.clone()
        };

        speech(text_for_speech, guild_id, handler_lock.unwrap()).await;
    }

    async fn voice_state_update(
        &self,
        ctx: Context,
        _: Option<serenity::model::id::GuildId>,
        old_voice_state: Option<voice::VoiceState>,
        new_voice_state: voice::VoiceState,
    ) {
        let state = CurrentVoiceState::new(new_voice_state);
        match state.new_speaker(&ctx, old_voice_state).await {
            Ok(speaker) => {
                if let Some(handler_lock) =
                    get_handler_when_in_voice_channel(&ctx, speaker.guild_id).await
                {
                    let message = format!("{:?} さんいらっしゃい", speaker.user.name);

                    speech(message, speaker.guild_id, handler_lock).await;
                };
            }
            Err(str) => {
                println!("[DEBUG] {:?}", str)
            }
        }
    }
}

async fn is_ignore_msg(
    msg: &Message,
    handler_lock: &Option<std::sync::Arc<serenity::prelude::Mutex<songbird::Call>>>,
) -> bool {
    // botに反応しないようにする
    if msg.author.bot {
        return true;
    };

    // コマンドに反応しないようにする
    if msg.content.starts_with(
        &env::var("DISCORD_CMD_PREFIX").expect("Expected a command prefix in the environment"),
    ) {
        return true;
    };

    // voice channel にいない場合は動かさない
    if handler_lock.is_none() {
        return true;
    };

    false
}

async fn debug_print(msg: &Message, ctx: &Context) {
    // サーバーのID
    eprintln!("guild_id = {:?}", msg.guild_id);
    // チャンネル名
    let channel_name = msg.channel_id.name(&ctx.cache).await;
    eprintln!("channel_name = {:?}", channel_name);
    // メッセージの送信
    let content = msg.content.clone();
    eprintln!("message received: {:?}", content);
}
}
