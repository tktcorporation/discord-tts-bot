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
