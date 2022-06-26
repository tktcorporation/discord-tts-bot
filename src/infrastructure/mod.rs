mod path_router;

// root/sounds
pub use path_router::SharedSoundPath;

// root/tmp/guild_id
pub use path_router::GuildPath;
mod sound_path;
// root/tmp/guild_id/sounds
pub use sound_path::SoundPath;
mod speech_file_path;
// root/tmp/guild_id/sounds/speech_file
pub use speech_file_path::SpeechFilePath;
