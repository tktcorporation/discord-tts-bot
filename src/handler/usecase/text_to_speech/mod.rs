pub mod config;
pub mod speech_options;
mod text_to_speech_base;
#[cfg(feature = "tts")]
mod text_to_speech_message;
#[cfg(feature = "tts")]
pub use text_to_speech_base::text_to_speech;
pub use text_to_speech_base::SpeechMessage;
