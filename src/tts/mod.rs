use polly::model::{Engine, Voice};
use std::error::Error;

async fn describe() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    let client = polly::Client::from_env();
    let mut tok = None;
    let mut voices: Vec<Voice> = vec![];
    // Below is an an example of how pagination can be implemented manually.
    loop {
        let mut req = client.describe_voices();
        if let Some(tok) = tok {
            req = req.next_token(tok);
        }
        let resp = req.send().await?;
        for voice in resp.voices.unwrap_or_default() {
            println!(
                "I can speak as: {} in {:?}",
                voice.name.as_ref().unwrap(),
                voice.language_name.as_ref().unwrap()
            );
            voices.push(voice);
        }
        tok = match resp.next_token {
            Some(next) => Some(next),
            None => break,
        };
    }
    let neural_voices = voices
        .iter()
        .filter(|voice| {
            voice
                .supported_engines
                .as_deref()
                .unwrap_or_default()
                .contains(&Engine::Neural)
        })
        .map(|voice| voice.id.as_ref().unwrap())
        .collect::<Vec<_>>();

    println!("Voices supporting a neural engine: {:?}", neural_voices);
    Ok(())
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
    async fn test_describe() {
        let result = describe().await;
        result.unwrap();
        assert_eq!(true, true);
    }
}
