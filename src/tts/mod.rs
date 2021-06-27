use std::env;

use std::ffi::OsStr;
use std::path::Path;

use polly::model::{OutputFormat, TextType, VoiceId};
use polly::{Client, Config, Region};

use aws_types::region::{EnvironmentProvider, ProvideRegion};

use tokio::io::AsyncWriteExt;

/// Generate a mp3 file and return the file path str
///
/// ## Examples
///
/// ```no_run
/// let result = generate_speech_file(
/// String::from("おはようございます"),
/// VoiceId::Mizuki,
/// "sample",
/// true,
/// )
/// .await;
/// Path::new(result.unwrap()).exists(); // true or false
/// ```
pub async fn generate_speech_file<P: AsRef<OsStr>>(
    content: String,
    voice_id: VoiceId,
    file_path: P,
    verbose: bool,
) -> Result<String, Box<dyn std::error::Error>> {
    let region = EnvironmentProvider::new().region().unwrap_or_else(|| {
        Region::new(env::var("AWS_REGION").expect("Expected a region string in the environment"))
    });

    if verbose {
        println!("polly client version: {}\n", polly::PKG_VERSION);
        println!("Region:   {:?}", &region);
        println!("Filename: {}", file_path.as_ref().to_str().unwrap());

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
    ssml_text.push_str("<speak><prosody pitch=\"+200%\"><amazon:effect phonation=\"soft\"><amazon:effect vocal-tract-length=\"-15%\"><prosody amazon:max-duration=\"5s\">");
    ssml_text.push_str(&content);
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

    let parts: Vec<&str> = file_path.as_ref().to_str().unwrap().split('.').collect();
    let mut file_name_builder = String::from(parts[0]);
    file_name_builder.push_str(".mp3");
    let file_name = file_name_builder.clone().to_string();
    let out_file = Path::new(&file_name);

    // create the dir before running this line.
    let mut file = tokio::fs::File::create(&out_file)
        .await
        .expect("failed to create file");

    file.write_all_buf(&mut blob)
        .await
        .expect("failed to write to file");

    Ok(file_name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_generate_sound() {
        let root = option_env!("CARGO_MANIFEST_DIR").unwrap();
        let path = Path::new(root);
        let file_path = path.join("binaries").join("sample");
        let result = generate_speech_file(
            String::from("おはようございます"),
            VoiceId::Mizuki,
            file_path.clone(),
            true,
        )
        .await;
        assert_eq!(result.is_ok(), true);
        let path = result.unwrap();
        assert_eq!(Path::new(&path).exists(), true);
        let mut right = String::from(file_path.clone().to_str().unwrap());
        right.push_str(".mp3");
        assert_eq!(path, right);
    }
}
