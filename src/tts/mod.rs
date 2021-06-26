use std::env;

use std::ffi::OsStr;
use std::path::Path;

use polly::model::{OutputFormat, VoiceId};
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

    let resp = client
        .synthesize_speech()
        .output_format(OutputFormat::Mp3)
        .text(content)
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
    async fn https_access_requwest() {
        let url = String::from("https://www.google.co.jp/");
        let client = reqwest::Client::new();
        let res = client.get(&url).send().await.unwrap();
        assert_eq!(&res.status().as_str()[..], "200");
        let body = &res.text().await.unwrap();
        assert!(&body.len() > &(100 as usize));
    }

    #[tokio::test]
    async fn test_generate_sound() {
        let result = generate_speech_file(
            String::from("おはようございます"),
            VoiceId::Mizuki,
            "sample",
            true,
        )
        .await;
        assert_eq!(result.is_ok(), true);
        let path = result.unwrap();
        assert_eq!(Path::new(&path).exists(), true);
        assert_eq!(path, "sample.mp3".to_string());
    }
}
