#[cfg(feature = "tts")]
use super::super::interface::Speaker;
#[cfg(feature = "tts")]
use super::config;
#[cfg(feature = "tts")]
use super::text_to_speech_message::Message;
#[cfg(feature = "tts")]
use crate::handler::error::{format_err, report_error};
#[cfg(feature = "tts")]
use crate::infrastructure::GuildPath;

#[derive(Debug, PartialEq, Eq)]
pub struct SpeechMessage {
    pub value: String,
}

#[cfg(feature = "tts")]
pub async fn text_to_speech(speaker: Box<dyn Speaker + Sync + Send>, msg: Message) {
    // If it's a bot message or command, ignore it.
    if msg.is_from_bot() {
        return;
    };
    let config = config::client::new(GuildPath::new(&speaker.guild_id()))
        .read()
        .unwrap();
    let speech_options = config
        .map(|config| config.speech_options)
        .unwrap_or_default();
    if is_ignore_channel(speech_options.read_channel_id, &msg) {
        return;
    }
    if let Err(e) = speaker.speech(msg.to_speech_message(speech_options)).await {
        report_error(&format_err("Failed to speech message", e));
    }
}

#[cfg(feature = "tts")]
fn is_ignore_channel(read_channel_id: Option<u64>, msg: &Message) -> bool {
    if let Some(read_channel_id) = read_channel_id {
        if u64::from(msg.msg.channel_id) != read_channel_id {
            return true;
        };
    };
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::handler::usecase::interface::MockSpeaker;
    use regex::Regex;
    use serenity::model::{channel::Message as SerenityMessage, id::GuildId};

    #[tokio::test]
    async fn test_text_to_speech() {
        let mut speaker = MockSpeaker::new();
        let msg = message_factory("some message");
        speaker.expect_speech().times(1).return_const(Ok(()));
        speaker
            .expect_guild_id()
            .times(1)
            .return_const(GuildId::new(1));
        assert!(!msg.is_from_bot());
        text_to_speech(Box::new(speaker), msg).await;
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
