use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SpeechOptions {
    pub read_channel_id: Option<u64>,
}
