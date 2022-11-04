use super::speech_options::SpeechOptions;
use crate::infrastructure::GuildPath;
use serde::{Deserialize, Serialize};

pub struct Client {
    guild_path: GuildPath,
}

pub mod client {
    use super::{Client, Config, ConfigFilePath, GuildPath};
    pub fn new(guild_path: GuildPath) -> Client {
        Client { guild_path }
    }

    impl Client {
        pub fn write(&self, config: Config) {
            let config_path = ConfigFilePath::new(self.guild_path.clone());
            let config_str = serde_json::to_string(&config).expect("fail to serialize config");
            std::fs::write(config_path.value, config_str).expect("fail to write config");
        }

        pub fn read(&self) -> Result<Option<Config>, String> {
            let config_path = ConfigFilePath::new(self.guild_path.clone());
            // return empty config if config file does not exist
            if !config_path.value.exists() {
                return Ok(None);
            }
            let config_str = match std::fs::read_to_string(config_path.value) {
                Ok(config_str) => config_str,
                Err(err) => return Err(format!("fail to read config: {}", err)),
            };
            Ok(serde_json::from_str(&config_str).expect("fail to deserialize config"))
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub speech_options: SpeechOptions,
}

struct ConfigFilePath {
    value: std::path::PathBuf,
}
impl ConfigFilePath {
    pub fn new(guild_path: GuildPath) -> Self {
        let path = std::path::PathBuf::from(guild_path).join("config.json");
        Self { value: path }
    }
}
