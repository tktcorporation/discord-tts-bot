use super::SoundPath;
use std::path::Path;
use tokio::fs::File;

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

#[derive(Debug)]
pub struct SpeechFilePath {
    value: std::path::PathBuf,
}

impl SpeechFilePath {
    pub fn new(sound_path: SoundPath) -> SpeechFilePath {
        use rand::Rng;

        // guild ごとに最大5ファイル持つ
        let rand_num: i32 = rand::thread_rng().gen_range(0..4);
        std::path::PathBuf::from(sound_path)
            .join(rand_num.to_string())
            .into()
    }

    pub async fn file(&self) -> SpeechFile {
        let file_name = self.file_name();

        SpeechFile {
            value: File::create(&Path::new(&file_name)).await.unwrap(),
            name: file_name,
        }
    }

    pub fn file_name(&self) -> String {
        let parts: Vec<&str> = self.value.to_str().unwrap().split('.').collect();
        let mut file_name_builder = String::from(parts[0]);
        file_name_builder.push_str(".mp3");
        file_name_builder
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
