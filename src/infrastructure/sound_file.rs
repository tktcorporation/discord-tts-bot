use super::{GuildPath, SoundPath, SpeechFilePath};
use serenity::model::id;
use std::path::Path;
use tiger::{digest::Digest, Tiger};

pub struct SoundFile {
    base: String,
}

impl SoundFile {
    /// base: env!("CARGO_MANIFEST_DIR")
    pub fn new(base: &str) -> Self {
        SoundFile {
            base: base.to_string(),
        }
    }
    pub fn root_path(self) -> SoundPath {
        let path = Path::new(&self.base);
        let sound_path = path.join("sounds");
        std::fs::create_dir_all(&sound_path).expect("fail to create a dir of guild path");
        sound_path.into()
    }

    pub fn guild_path(self, guild_id: &id::GuildId) -> GuildPath {
        let root: std::path::PathBuf = self.root_path().into();
        let digest = Tiger::digest(guild_id.to_string().as_bytes());
        let guild_id_digest_str = format!("{:X}", digest);
        let guild_path = root.join(guild_id_digest_str);
        std::fs::create_dir_all(&guild_path).expect("fail to create a dir of guild path");
        guild_path.into()
    }

    pub fn speech_file_path(self, guild_id: &id::GuildId) -> SpeechFilePath {
        use rand::Rng;

        let guild_path: std::path::PathBuf = self.guild_path(guild_id).into();

        // guild ごとに最大5ファイル持つ
        let rand_num: i32 = rand::thread_rng().gen_range(0..4);
        guild_path.join(rand_num.to_string()).into()
    }
}
