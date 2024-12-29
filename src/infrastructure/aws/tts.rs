use aws_types::region::Region;
use polly::config::BehaviorVersion;
use polly::types::{OutputFormat, TextType, VoiceId};
use polly::Client;
use std::env;
use tokio::io::AsyncWriteExt;

use super::SpeechFilePath;

/// Generate a mp3 file and return the file path str
///
/// ## Examples
///
/// ```no_run
/// use polly::types::VoiceId;
/// use std::path::{Path, PathBuf};
/// use discord_tts_bot::infrastructure::tts::generate_speech_file;
/// use discord_tts_bot::infrastructure::SpeechFilePath;
///
/// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
/// let path = PathBuf::from("sample");
/// let file_path = SpeechFilePath::from(path);
/// let result = generate_speech_file(
///     "おはようございます",
///     VoiceId::Mizuki,
///     &file_path,
///     true,
/// ).await?;
/// assert!(Path::new(&result).exists());
/// # Ok(())
/// # }
/// ```
pub async fn generate_speech_file(
    content: &str,
    voice_id: VoiceId,
    file_path: &SpeechFilePath,
    verbose: bool,
) -> Result<String, Box<dyn std::error::Error>> {
    let region = Region::new(
        env::var("AWS_REGION").expect("AWS_REGION must be set in environment variables"),
    );
    let config = aws_config::defaults(BehaviorVersion::v2024_03_28())
        .region(region.clone())
        .load()
        .await;

    if verbose {
        println!("polly client version: {}\n", polly::meta::PKG_VERSION);
        println!("Region:   {:?}", &region);
    }

    let client = Client::new(&config);

    let mut ssml_text = String::new();
    ssml_text.push_str("<speak><prosody pitch=\"+200%\"><amazon:effect phonation=\"soft\"><amazon:effect vocal-tract-length=\"-15%\"><prosody amazon:max-duration=\"30s\">");
    ssml_text.push_str(&html_escape::encode_text(content));
    ssml_text.push_str("</prosody></amazon:effect></amazon:effect></prosody></speak>");

    let resp = client
        .synthesize_speech()
        .text_type(TextType::Ssml)
        .output_format(OutputFormat::Mp3)
        .text(ssml_text)
        .voice_id(voice_id)
        .send()
        .await?;

    let mut blob = resp
        .audio_stream
        .collect()
        .await
        .expect("failed to read data");

    let mut file = file_path.file().await;
    file.value
        .write_all_buf(&mut blob)
        .await
        .expect("failed to write to file");

    Ok(file.name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[ignore]
    #[tokio::test]
    async fn test_generate_sound() {
        let root = env!("CARGO_MANIFEST_DIR");
        let path = Path::new(root);
        let file_path: SpeechFilePath = path.join("sounds").join("sample").into();
        let right = file_path.file_name();
        let result =
            generate_speech_file("おはようございます", VoiceId::Mizuki, &file_path, true).await;
        let path = result.unwrap();
        assert!(Path::new(&path).exists());

        assert_eq!(path, right);
    }
}
