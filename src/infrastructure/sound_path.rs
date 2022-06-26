use super::path_router::GuildPath;

#[derive(Debug)]
pub struct SoundPath {
    value: std::path::PathBuf,
}

impl SoundPath {
    pub fn new(guild_path: GuildPath) -> Self {
        let sound_path = std::path::PathBuf::from(guild_path).join("sounds");
        std::fs::create_dir_all(&sound_path).expect("fail to create a dir of guild path");
        sound_path.into()
    }
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
