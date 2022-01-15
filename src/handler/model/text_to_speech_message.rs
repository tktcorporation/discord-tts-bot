pub use crate::model::Message;

#[derive(Debug, PartialEq)]
pub struct SpeechMessage {
    pub value: String,
}

impl Message {
    pub fn to_speech_message(&self) -> SpeechMessage {
        // urlはそのまま読まない
        let str = if self.msg.content.contains("http") {
            "url".to_string()
        } else {
            self.msg.content.clone()
        };
        SpeechMessage {
            value: remove_role_string(&replace_channel_string(
                &remove_emoji_string(&remove_mention_string(&str[..])[..])[..],
            )),
        }
    }
}

fn remove_mention_string(content: &str) -> String {
    use regex::Regex;
    let re = Regex::new(r"<@![0-9]+>").unwrap();
    re.replace_all(content, "").to_string()
}
fn remove_role_string(content: &str) -> String {
    use regex::Regex;
    let re = Regex::new(r"<@&[0-9]+>").unwrap();
    re.replace_all(content, "").to_string()
}
fn remove_emoji_string(content: &str) -> String {
    use regex::Regex;
    let re = Regex::new(r"<:(.+):[0-9]+>").unwrap();
    if let Some(caps) = re.captures(content) {
        re.replace_all(content, caps.get(1).unwrap().as_str())
            .to_string()
    } else {
        content.to_string()
    }
}
fn replace_channel_string(content: &str) -> String {
    use regex::Regex;
    let re = Regex::new(r"<#[0-9]+>").unwrap();
    if re.captures(content).is_some() {
        "channel".to_string()
    } else {
        content.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serenity::model::channel::Message as SerenityMessage;

    #[cfg(test)]
    mod remove_mention_string_test {
        use super::*;

        #[test]
        fn test_remove_mention_string() {
            let str = "aaa<@!8379454856049>eeee";
            let result = remove_mention_string(str);
            assert_eq!("aaaeeee", result);
        }

        #[test]
        fn test_remove_role_string() {
            let str = "aaa<@&8379454856049>eeee";
            let result = remove_role_string(str);
            assert_eq!("aaaeeee", result);
        }

        #[test]
        fn test_remove_emoji_string() {
            let str = "<:butter:872873394570424340>";
            let result = remove_emoji_string(str);
            assert_eq!("butter", result);
        }

        #[test]
        fn test_replace_channel_string() {
            let str = "<#795680552845443113>";
            let result = replace_channel_string(str);
            assert_eq!("channel", result);
        }
    }

    #[cfg(test)]
    mod to_speech_message_tests {
        use super::*;

        #[test]
        fn test_to_speech_message() {
            let message = message_factory("https://example.com");
            assert_eq!("url", &message.to_speech_message().value);
        }

        #[test]
        fn test_to_speech_message_not_ssl() {
            let message = message_factory("http://example.com");
            assert_eq!("url", &message.to_speech_message().value);
        }

        #[test]
        fn test_to_speech_message_mix() {
            let message = message_factory("おはようhttps://example.comこんにちは");
            assert_eq!("url", &message.to_speech_message().value);
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
