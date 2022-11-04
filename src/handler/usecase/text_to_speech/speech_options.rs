use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SpeechOptions {
    pub is_ojosama: bool,
    pub read_channel_id: Option<u64>,
}
