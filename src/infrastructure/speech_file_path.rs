use super::SoundPath;
use std::path::Path;
use tokio::fs::File;

/// SpeechFileを表す構造体
#[derive(Debug)]
pub struct SpeechFile {
    /// ファイル名
    pub name: String,
    /// ファイルの実体
    pub value: File,
}

impl From<SpeechFile> for File {
    /// SpeechFile から File への変換
    ///
    /// # Arguments
    ///
    /// * `item` - 変換元のSpeechFile
    ///
    /// # Returns
    ///
    /// 変換後のFile
    fn from(item: SpeechFile) -> File {
        item.value
    }
}

/// SpeechFilePathを表す構造体
#[derive(Debug)]
pub struct SpeechFilePath {
    /// ファイルパス
    value: std::path::PathBuf,
}

impl SpeechFilePath {
    /// 新しい SpeechFilePath を作成します。
    ///
    /// guild ごとに最大5ファイル持つように、ランダムな数値をパスに含めます。
    ///
    /// # Arguments
    ///
    /// * `sound_path` - SoundPath
    ///
    /// # Returns
    ///
    /// 新しい SpeechFilePath
    pub fn new(sound_path: SoundPath) -> SpeechFilePath {
        use rand::Rng;

        // guild ごとに最大5ファイル持つ
        let rand_num: i32 = rand::rng().random_range(0..4);
        std::path::PathBuf::from(sound_path)
            .join(rand_num.to_string())
            .into()
    }

    /// SpeechFile を非同期に取得します。
    ///
    /// # Returns
    ///
    /// SpeechFile
    pub async fn file(&self) -> SpeechFile {
        let file_name = self.file_name();

        SpeechFile {
            value: File::create(&Path::new(&file_name)).await.unwrap(),
            name: file_name,
        }
    }

    /// ファイル名を取得します。
    ///
    /// 拡張子を .mp3 に変更します。
    ///
    /// # Returns
    ///
    /// ファイル名
    pub fn file_name(&self) -> String {
        let parts: Vec<&str> = self.value.to_str().unwrap().split('.').collect();
        let mut file_name_builder = String::from(parts[0]);
        file_name_builder.push_str(".mp3");
        file_name_builder
    }
}

impl From<std::path::PathBuf> for SpeechFilePath {
    /// std::path::PathBuf から SpeechFilePath への変換
    ///
    /// # Arguments
    ///
    /// * `item` - 変換元のstd::path::PathBuf
    ///
    /// # Returns
    ///
    /// 変換後のSpeechFilePath
    fn from(item: std::path::PathBuf) -> SpeechFilePath {
        SpeechFilePath { value: item }
    }
}

impl From<SpeechFilePath> for std::path::PathBuf {
    /// SpeechFilePath から std::path::PathBuf への変換
    ///
    /// # Arguments
    ///
    /// * `item` - 変換元のSpeechFilePath
    ///
    /// # Returns
    ///
    /// 変換後のstd::path::PathBuf
    fn from(item: SpeechFilePath) -> std::path::PathBuf {
        item.value
    }
}
