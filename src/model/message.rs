use serenity::model::channel::Message as SerenityMessage;
use std::env;

pub struct Message {
    pub msg: SerenityMessage,
}

impl Message {
    pub fn new(msg: SerenityMessage) -> Message {
        Message { msg }
    }
    pub fn is_command(&self) -> bool {
        if self.msg.content.starts_with(
            (env::var("DISCORD_CMD_PREFIX").expect("Expected a command prefix in the environment") + " ")
                .as_str(),
        ) {
            return true;
        };

        false
    }
    pub fn is_from_bot(&self) -> bool {
        if self.msg.author.bot {
            return true;
        };

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(test)]
    mod is_command_tests {
        use super::*;

        #[test]
        fn test_is_command_msg() {
            let message = message_factory("a", false);
            assert!(!message.is_command());
        }

        #[test]
        fn test_is_command_msg_and() {
            let message = message_factory("hogehoege&sa", false);
            assert!(!message.is_command());
        }

        #[test]
        fn test_is_command_msg_cmd_pref() {
            let content = &(env::var("DISCORD_CMD_PREFIX").unwrap() + " hogehoge")[..];
            let message = message_factory(content, true);
            assert!(message.is_command());
        }
    }

    #[cfg(test)]
    mod is_from_bot_tests {
        use super::*;

        #[test]
        fn test_is_from_bot_msg() {
            let message = message_factory("a", true);
            assert!(message.is_from_bot());
        }

        #[test]
        fn test_is_from_bot_msg_and() {
            let message = message_factory("hogehoege&sa", true);
            assert!(message.is_from_bot());
        }

        #[test]
        fn test_is_from_bot_msg_cmd_pref() {
            let content = &(env::var("DISCORD_CMD_PREFIX").unwrap() + " hogehoge")[..];
            let message = message_factory(content, false);
            assert!(!message.is_from_bot());
        }
    }

    use regex::Regex;

    #[test]
    fn test_factory() {
        let m = message_factory("message", true);
        assert!(m.is_from_bot());
        assert_eq!("message", m.msg.content);
    }

    fn message_factory(content: &str, from_bot: bool) -> Message {
        let message_json = r#"{
        "id":881482961801842698,
        "attachments":[],
        "author": {
            "id":502486808211357707,
            "avatar":"bfdafa09852e451e32f7ac1919bab46f",
            "bot":[FROM_BOT],
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
        let re_content = Regex::new(r"\[CONTENT\]").unwrap();
        let result = re_content.replace(message_json, content).to_string();
        let re_from_bot = Regex::new(r"\[FROM_BOT\]").unwrap();
        let result = re_from_bot
            .replace(&result, if from_bot { "true" } else { "false" })
            .to_string();
        let m: SerenityMessage = serde_json::from_str(&result[..]).unwrap();
        Message::new(m)
    }
}
