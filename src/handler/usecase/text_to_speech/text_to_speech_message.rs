use super::speech_options::SpeechOptions;
use super::SpeechMessage;
pub use crate::model::Message;
use regex::Regex;

impl Message {
    pub fn to_speech_message(&self, _options: SpeechOptions) -> SpeechMessage {
        let content = self.get_content();

        // urlをスキップ
        let str = content
            .split_whitespace()
            .filter(|word| !word.starts_with("http"))
            .collect::<Vec<&str>>()
            .join(" ");

        // convert discord styled string for speech
        let converted = convert_discord_string(&str);

        // 100文字を超えた場合は、トリムして語尾に「うぬんかんぬんうんぬんかんぬん」を追加
        let message = if converted.chars().count() > 100 {
            let trimmed = converted.chars().take(100).collect::<String>();
            format!("{}うぬんかんぬんうんぬんかんぬん", trimmed)
        } else {
            converted
        };

        SpeechMessage { value: message }
    }
}

#[derive(Debug)]
enum DiscordStringType {
    Channel,
    Role,
    Emoji,
    Animoji,
    Mention,
}
impl DiscordStringType {
    fn to_regex(&self) -> Regex {
        match self {
            DiscordStringType::Channel => Regex::new(r"<#[0-9]+?>").unwrap(),
            DiscordStringType::Role => Regex::new(r"<@&[0-9]+?>").unwrap(),
            DiscordStringType::Emoji => Regex::new(r"<:(.+?):[0-9]+?>").unwrap(),
            DiscordStringType::Animoji => Regex::new(r"<a:(.+?):[0-9]+?>").unwrap(),
            DiscordStringType::Mention => Regex::new(r"<@[0-9]+?>").unwrap(),
        }
    }
    fn to_convert_type(&self) -> ConvertType {
        match self {
            DiscordStringType::Channel => ConvertType::Empty,
            DiscordStringType::Role => ConvertType::Empty,
            DiscordStringType::Emoji => ConvertType::MatchString,
            DiscordStringType::Animoji => ConvertType::MatchString,
            DiscordStringType::Mention => ConvertType::Empty,
        }
    }
    fn from_str(s: &str) -> Option<DiscordStringType> {
        let type_ = DiscordStringType::Channel;
        if type_.to_regex().is_match(s) {
            return Some(type_);
        }
        let type_ = DiscordStringType::Role;
        if type_.to_regex().is_match(s) {
            return Some(type_);
        }
        let type_ = DiscordStringType::Emoji;
        if type_.to_regex().is_match(s) {
            return Some(type_);
        }
        let type_ = DiscordStringType::Animoji;
        if type_.to_regex().is_match(s) {
            return Some(type_);
        }
        let type_ = DiscordStringType::Mention;
        if type_.to_regex().is_match(s) {
            return Some(type_);
        }
        None
    }
}

enum ConvertType {
    Empty,
    MatchString,
}
impl ConvertType {
    fn convert(&self, regex: &Regex, str: &str) -> String {
        match self {
            ConvertType::Empty => regex.replace_all(str, "").to_string(),
            ConvertType::MatchString => regex
                .captures_iter(str)
                .collect::<Vec<_>>()
                .iter()
                .fold(str.to_string(), |acc, cap| acc.replace(&cap[0], &cap[1])),
        }
    }
}

fn convert_discord_string(str: &str) -> String {
    let (re, convert_type) = if let Some(type_) = DiscordStringType::from_str(str) {
        (type_.to_regex(), type_.to_convert_type())
    } else {
        return str.to_string();
    };
    convert_discord_string(&convert_type.convert(&re, str))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serenity::model::channel::Message as SerenityMessage;

    #[cfg(test)]
    mod convert_discord_string_test {
        use super::*;

        #[test]
        fn test_remove_mention_string() {
            let str = "aaa<@8379454856049>eeee";
            let result = convert_discord_string(str);
            assert_eq!("aaaeeee", result);
        }
        #[test]
        fn test_remove_double_mention_string() {
            let str = "aaa<@8379454856049>eeee<@8379454856049>uuu";
            let result = convert_discord_string(str);
            assert_eq!("aaaeeeeuuu", result);
        }

        #[test]
        fn test_remove_role_string() {
            let str = "aaa<@&8379454856049>eeee";
            let result = convert_discord_string(str);
            assert_eq!("aaaeeee", result);
        }
        #[test]
        fn test_remove_double_role_string() {
            let str = "aaa<@&8379454856049>eeee<@&8379454856049>uuu";
            let result = convert_discord_string(str);
            assert_eq!("aaaeeeeuuu", result);
        }

        #[test]
        fn test_remove_emoji_string() {
            let str = "<:butter:872873394570424340>";
            let result = convert_discord_string(str);
            assert_eq!("butter", result);
        }

        #[test]
        fn test_remove_animoji_string() {
            let str = "<a:sanma:872873394570424340>";
            let result = convert_discord_string(str);
            assert_eq!("sanma", result);
        }

        #[test]
        fn test_remove_double_emoji_string() {
            let content = "<:butter:872873394570424340>さんま<:sanma:872873394570424340>";
            let result = convert_discord_string(content);
            assert_eq!("butterさんまsanma", result);
        }

        #[test]
        fn test_replace_channel_string() {
            let str = "aaa<#795680552845443113>rrr";
            let result = convert_discord_string(str);
            assert_eq!("aaarrr", result);
        }
        #[test]
        fn test_replace_double_channel_string() {
            let str = "aaa<#795680552845443113>rrr<#795680552845443113>sss";
            let result = convert_discord_string(str);
            assert_eq!("aaarrrsss", result);
        }
    }

    #[cfg(test)]
    mod to_speech_message_tests {
        use super::*;
        #[test]
        fn test_message() {
            let message = message_factory("https://example.com");
            assert_eq!(
                "",
                &message
                    .to_speech_message(SpeechOptions {
                        read_channel_id: None
                    })
                    .value
            );
        }

        #[test]
        fn test_not_ssl() {
            let message = message_factory("http://example.com");
            assert_eq!(
                "",
                &message
                    .to_speech_message(SpeechOptions {
                        read_channel_id: None
                    })
                    .value
            );
        }

        #[test]
        fn test_url_in_text() {
            let message = message_factory("おはよう https://example.com こんにちは");
            assert_eq!(
                "おはよう こんにちは",
                &message
                    .to_speech_message(SpeechOptions {
                        read_channel_id: None
                    })
                    .value
            );
        }

        #[test]
        fn test_mix() {
            let message = message_factory("<@8379454856049>おはよう<:sanma:872873394570424340>こんにちは<#795680552845443113>でも<@&8379454856049>これは<@&8379454856049><:butter:872873394570424340>です");
            assert_eq!(
                "おはようsanmaこんにちはでもこれはbutterです",
                &message
                    .to_speech_message(SpeechOptions {
                        read_channel_id: None
                    })
                    .value
            );
        }

        #[test]
        fn test_trimmed_message() {
            let long_text = "あ".repeat(100);
            let message = message_factory(format!("{}いいいい", long_text).as_str());
            assert_eq!(
                format!("{}うぬんかんぬんうんぬんかんぬん", long_text),
                message
                    .to_speech_message(SpeechOptions {
                        read_channel_id: None
                    })
                    .value
            );
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
