use super::super::model::text_to_speech_message::Message;
use super::interface::Speaker;

pub async fn text_to_speech(speaker: Box<dyn Speaker + Sync + Send>, msg: Message) {
    if msg.is_ignore() {
        return;
    };
    speaker.speech(msg.to_speech_message()).await;
}

#[cfg(test)]
mod tests {
    #[cfg(test)]
    use super::super::interface::MockSpeaker;
    use super::*;
    use regex::Regex;
    use serenity::model::channel::Message as SerenityMessage;

    #[tokio::test]
    async fn test_text_to_speech() {
        let mut speaker = MockSpeaker::new();
        let msg = message_factory("some message");
        speaker.expect_speech().times(1).return_const(());

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
