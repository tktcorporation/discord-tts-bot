use aws_types::region::{EnvironmentProvider, ProvideRegion};
use polly::model::{OutputFormat, TextType, VoiceId};
use polly::{Client, Config, Region};
use std::env;
use tokio::io::AsyncWriteExt;

use super::SpeechFilePath;

/// Generate a mp3 file and return the file path str
///
/// ## Examples
///
/// ```no_run
/// let result = generate_speech_file(
///   String::from("おはようございます"),
///   VoiceId::Mizuki,
///   "sample",
///   true,
/// )
/// .await;
/// Path::new(result.unwrap()).exists(); // true or false
/// ```
pub async fn generate_speech_file(
    content: String,
    voice_id: VoiceId,
    file_path: SpeechFilePath,
    verbose: bool,
) -> Result<String, Box<dyn std::error::Error>> {
    let region = EnvironmentProvider::new().region().unwrap_or_else(|| {
        Region::new(env::var("AWS_REGION").expect("Expected a region string in the environment"))
    });

    if verbose {
        println!("polly client version: {}\n", polly::PKG_VERSION);
        println!("Region:   {:?}", &region);
        // println!("Filename: {}", file_path.into().to_str().unwrap());

        // SubscriberBuilder::default()
        //     .with_env_filter("info")
        //     .with_span_events(FmtSpan::CLOSE)
        //     .init();
    }

    let config = Config::builder().region(region).build();

    let client = Client::from_conf(config);

    let mut ssml_text = String::new();
    // 声質の変更と最大再生秒数の設定
    // https://docs.aws.amazon.com/ja_jp/polly/latest/dg/supportedtags.html
    ssml_text.push_str("<speak><prosody pitch=\"+200%\"><amazon:effect phonation=\"soft\"><amazon:effect vocal-tract-length=\"-15%\"><prosody amazon:max-duration=\"30s\">");
    ssml_text.push_str(&html_escape::encode_text(&content));
    ssml_text.push_str("</prosody></amazon:effect></amazon:effect></prosody></speak>");

    let resp = client
        .synthesize_speech()
        .set_text_type(Option::Some(TextType::Ssml))
        .output_format(OutputFormat::Mp3)
        .text(ssml_text)
        .voice_id(voice_id)
        .send()
        .await?;

    // Get MP3 data from response and save it
    let mut blob = resp
        .audio_stream
        .collect()
        .await
        .expect("failed to read data");

    // create the dir before running this line.
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
        let result = generate_speech_file(
            String::from("おはようございます"),
            VoiceId::Mizuki,
            file_path,
            true,
        )
        .await;
        assert!(result.is_ok());
        let path = result.unwrap();
        assert!(Path::new(&path).exists());

        assert_eq!(path, right);
    }
}
