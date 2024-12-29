use reqwest::Client;
use serenity::prelude::TypeMapKey;

pub mod message;
pub mod voice;

#[cfg(feature = "tts")]
pub use message::Message;
pub use voice::Voice;

pub struct HttpKey;

impl TypeMapKey for HttpKey {
    type Value = Client;
}
