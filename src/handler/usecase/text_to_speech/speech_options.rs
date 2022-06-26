use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SpeechOptions {
    pub is_ojosama: bool,
}
