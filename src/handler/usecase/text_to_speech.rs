use super::super::model::{text_to_speech_message::Message, voice::Voice};

pub async fn text_to_speech(voice: Voice, msg: Message) {
    if msg.is_ignore() {
        return;
    };
    voice.speech(msg.to_speech_text()).await;
}
