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
