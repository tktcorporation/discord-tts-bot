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
        let mension_removed_str = remove_mention_string(&str[..]);
        SpeechMessage {
            value: mension_removed_str,
        }
    }
}

fn remove_mention_string(content: &str) -> String {
    use regex::Regex;
    let re = Regex::new(r"<@![0-9]+>").unwrap();
    re.replace_all(content, "").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(test)]
    mod remove_mention_string_test {
        use super::*;

        #[test]
        fn test_remove_mention_string() {
            let str = "aaa<@!8379454856049>eeee";
            let result = remove_mention_string(str);
            assert_eq!("aaaeeee", result);
        }
    }

    #[cfg(test)]
    mod to_speech_text_tests {
        use super::*;

        #[test]
        fn test_to_speech_text() {
            let message = message_factory("https://example.com");
            assert_eq!("url", &message.to_speech_text().value);
        }

        #[test]
        fn test_to_speech_text_not_ssl() {
            let message = message_factory("http://example.com");
            assert_eq!("url", &message.to_speech_text().value);
        }

        #[test]
        fn test_to_speech_text_mix() {
            let message = message_factory("おはようhttps://example.comこんにちは");
            assert_eq!("url", &message.to_speech_text().value);
        }
    }

    #[cfg(test)]
    mod is_ignore_tests {
        use super::*;

        #[test]
        fn test_is_ignore_msg() {
            let message = message_factory("a");
            assert_eq!(false, message.is_ignore());
        }

        #[test]
        fn test_is_ignore_msg_and() {
            let message = message_factory("hogehoege&sa");
            assert_eq!(false, message.is_ignore());
        }

        #[test]
        fn test_is_ignore_msg_cmd_pref() {
            let content = &(env::var("DISCORD_CMD_PREFIX").unwrap() + "hogehoge")[..];
            let message = message_factory(content);
            assert_eq!(true, message.is_ignore());
        }
    }

    use regex::Regex;

    #[test]
    fn test_factory() {
        let m = message_factory("message");
        assert_eq!("message", m.msg.content);
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
        let m: SerenityMessage = serde_json::from_str(&result[..]).unwrap();
        Message::new(m)
    }
}
