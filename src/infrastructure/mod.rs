use serenity::model::id;
use std::convert::From;
use std::path::Path;
use tiger::{digest::Digest, Tiger};
use tokio::fs::File;

#[derive(Debug)]
pub struct SoundPath {
    value: std::path::PathBuf,
}

impl From<std::path::PathBuf> for SoundPath {
    fn from(item: std::path::PathBuf) -> SoundPath {
        SoundPath { value: item }
    }
}

impl From<SoundPath> for std::path::PathBuf {
    fn from(item: SoundPath) -> std::path::PathBuf {
        item.value
    }
}

#[derive(Debug)]
pub struct GuildPath {
    value: std::path::PathBuf,
}

impl From<std::path::PathBuf> for GuildPath {
    fn from(item: std::path::PathBuf) -> GuildPath {
        GuildPath { value: item }
    }
}

impl From<GuildPath> for std::path::PathBuf {
    fn from(item: GuildPath) -> std::path::PathBuf {
        item.value
    }
}

#[derive(Debug)]
pub struct SpeechFilePath {
    value: std::path::PathBuf,
}

impl SpeechFilePath {
    pub async fn file(self) -> SpeechFile {
        let parts: Vec<&str> = self.value.to_str().unwrap().split('.').collect();
        let mut file_name_builder = String::from(parts[0]);
        file_name_builder.push_str(".mp3");
        let file_name = file_name_builder.clone().to_string();
        
        SpeechFile {
            value: File::create(&Path::new(&file_name))
            .await
            .expect("failed to create file"),
            name: file_name,
        }
    }
}

impl From<std::path::PathBuf> for SpeechFilePath {
    fn from(item: std::path::PathBuf) -> SpeechFilePath {
        SpeechFilePath { value: item }
    }
}

impl From<SpeechFilePath> for std::path::PathBuf {
    fn from(item: SpeechFilePath) -> std::path::PathBuf {
        item.value
    }
}

#[derive(Debug)]
pub struct SpeechFile {
    pub name: String,
    pub value: File,
}

impl From<SpeechFile> for File {
    fn from(item: SpeechFile) -> File {
        item.value
    }
}

pub struct SoundFile {
    base: String,
}

impl SoundFile {
    pub fn new() -> Self {
        SoundFile {
            base: env!("CARGO_MANIFEST_DIR").to_string(),
        }
    }
    pub fn root_path(self) -> SoundPath {
        let base = env!("CARGO_MANIFEST_DIR");
        let path = Path::new(base);
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
