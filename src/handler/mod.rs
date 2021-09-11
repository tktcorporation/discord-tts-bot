use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::{channel::Message, gateway::Ready, voice},
};

mod model;
use model::context::Context as Ctx;
use model::speaker::CurrentVoiceState;
use model::voice::Voice;
mod message;
use message::is_ignore_msg;
mod usecase;

pub struct Handler;

#[async_trait]
#[cfg_attr(feature = "mock", mockall::automock)]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        let cont = Ctx::new(ctx);
        usecase::set_help_message_to_activity(Box::new(cont)).await
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if is_ignore_msg(&msg) {
            return;
        };

        let guild_id = msg.guild(&ctx.cache).await.unwrap().id;
        let voice = Voice::from(&ctx, guild_id).await;
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

        voice.speech(text_for_speech).await;
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
                let voice = Voice::from(&ctx, speaker.guild_id).await;
                let message = if speaker.is_new {
                    format!("{:?} さんいらっしゃい", speaker.user.name)
                } else {
                    format!("{:?} さんいってらっしゃい", speaker.user.name)
                };

                // botしかいなかったら
                match voice.members(&ctx).await {
                    Ok(members) => {
                        if members.len() <= 1 {
                            voice.leave().await.unwrap();
                        } else {
                            voice.speech(message).await;
                        }
                    }
                    Err(str) => {
                        println!("[DEBUG] {:?}", str)
                    }
                }
            }
            Err(str) => {
                println!("[DEBUG] {:?}", str)
            }
        }
    }
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
