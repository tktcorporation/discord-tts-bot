use serenity::model::{id::ChannelId, prelude::GuildId};

use crate::handler::usecase::text_to_speech::config;
use crate::handler::usecase::text_to_speech::speech_options;

pub async fn select_channel(guild_id: &GuildId, read_channel_id: ChannelId) {
    let client = config::client::new(crate::infrastructure::GuildPath::new(guild_id));
    let existing = match client.read() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Failed to read config: {e}");
            None
        }
    };
    let new_config = match existing {
        Some(config) => {
            let mut new = config;
            new.speech_options.read_channel_id = Some(read_channel_id.get());
            new
        }
        None => {
            let mut new = config::Config {
                speech_options: speech_options::SpeechOptions::default(),
            };
            new.speech_options.read_channel_id = Some(read_channel_id.get());
            new
        }
    };

    client.write(new_config);
}
