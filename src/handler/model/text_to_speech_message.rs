use serenity::model::channel::Message as SerenityMessage;
use std::env;

pub struct Message {
    msg: SerenityMessage,
}

pub struct SpeechMessage {
    pub value: String,
}

impl Message {
    pub fn new(msg: SerenityMessage) -> Message {
        Message { msg }
    }
    pub fn is_ignore(&self) -> bool {
        // botに反応しないようにする
        if self.msg.author.bot {
            return true;
        };

        // コマンドに反応しないようにする
        if self.msg.content.starts_with(
            &env::var("DISCORD_CMD_PREFIX").expect("Expected a command prefix in the environment"),
        ) {
            return true;
        };

        false
    }

    pub fn to_speech_text(&self) -> SpeechMessage {
        // urlはそのまま読まない
        let str = if self.msg.content.contains("http") {
            "url".to_string()
        } else {
            self.msg.content.clone()
        };
        SpeechMessage { value: str }
    }
}
