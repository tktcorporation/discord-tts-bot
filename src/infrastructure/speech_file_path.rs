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
