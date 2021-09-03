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
mod model;
pub mod services;
use model::speaker::CurrentVoiceState;
use model::voice::Voice;

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
                    },
                    Err(str) => {println!("[DEBUG] {:?}", str)},
                }
            }
            Err(str) => {
                println!("[DEBUG] {:?}", str)
            }
        }
    }
}

fn is_ignore_msg(msg: &Message) -> bool {
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

#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;
    use serenity::model::channel::Message;

    #[test]
    fn test_factory() {
        let m = message_factory("message");
        assert_eq!("message", m.content);
    }

    #[test]
    fn test_is_ignore_msg() {
        let message = message_factory("a");
        assert_eq!(false, is_ignore_msg(&message));
    }

    #[test]
    fn test_is_ignore_msg_and() {
        let message = message_factory("hogehoege&sa");
        assert_eq!(false, is_ignore_msg(&message));
    }

    #[test]
    fn test_is_ignore_msg_cmd_pref() {
        let content = &(env::var("DISCORD_CMD_PREFIX").unwrap() + "hogehoge")[..];
        let message = message_factory(content);
        assert_eq!(true, is_ignore_msg(&message));
    }

    fn message_factory(content: &str) -> Message {
        let message_json = r#"{
            "id":881482961801842698,
            "attachments":[],
            "author": {
                "id":502486808211357707,
                "avatar":"bfdafa09852e451e32f7ac1919bab46f",
                "bot":false,
                "discriminator":6539,
                "username":"tkt",
                "public_flags":0
            },
            "channel_id":713052877911752724,
            "content":"[CONTENT]",
            "edited_timestamp":null,
            "embeds":[],
            "guild_id":713052821850816604,
            "type":0,
            "member": {
                "deaf":false,
                "joined_at":"2020-05-21T15:37:20.702Z",
                "mute":false,
                "nick":null,
                "roles":[],
                "pending":false,
                "premium_since":null,
                "guild_id":null,
                "user":null
            },
            "mention_everyone":false,
            "mention_roles":[],
            "mention_channels":[],
            "mentions":[],
            "nonce":"881482961130618880",
            "pinned":false,
            "reactions":[],
            "timestamp":"2021-08-29T10:18:35.255Z",
            "tts":false,
            "webhook_id":null,
            "activity":null,
            "application":null,
            "message_reference":null,
            "flags":0,
            "stickers":[],
            "referenced_message":null
        }"#;
        let re = Regex::new(r"\[CONTENT\]").unwrap();
        let result = re.replace(message_json, content).to_string();
        let m: Message = serde_json::from_str(&result[..]).unwrap();
        m
    }
}
