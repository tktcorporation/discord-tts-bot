use google_texttospeech1::Texttospeech;
use google_texttospeech1::{api::ListVoicesResponse, client, Error, Result};

use std::default::Default;
use yup_oauth2;

pub async fn get_speech() -> client::Result<(hyper::Response<hyper::body::Body>, ListVoicesResponse)>
{
    let secret: yup_oauth2::ApplicationSecret = Default::default();
    let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
        secret,
        yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
    )
    .build()
    .await
    .unwrap();
    let mut hub = Texttospeech::new(
        hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()),
        auth,
    );
    hub.voices().list().language_code("magna").doit().await
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[tokio::test]
    // async fn get() {
    //     let result = get_speech().await;
    //     match result {
    //         Err(e) => match e {
    //             // The Error enum provides details about what exactly happened.
    //             // You can also just use its `Debug`, `Display` or `Error` traits
    //             Error::HttpError(_)
    //             | Error::Io(_)
    //             | Error::MissingAPIKey
    //             | Error::MissingToken(_)
    //             | Error::Cancelled
    //             | Error::UploadSizeLimitExceeded(_, _)
    //             | Error::Failure(_)
    //             | Error::BadRequest(_)
    //             | Error::FieldClash(_)
    //             | Error::JsonDecodeError(_, _) => println!("{}", e),
    //         },
    //         Ok(res) => println!("Success: {:?}", res),
    //     }
    // }

    extern crate hyper;

    use hyper::Client;
    use hyper_tls::HttpsConnector;


    // #[tokio::test]
    // async fn https_access() {
    //     let https = HttpsConnector::new();
    //     let client = Client::new();
    //     let uri = "https://www.google.co.jp/".parse().unwrap();
    //     let resp = client.get(uri).await.unwrap();
    //     println!("status={}", resp.status());
    // }

    #[tokio::test]
    async fn https_access_requwest() {
        let url = String::from("https://www.google.co.jp/");
        let client = reqwest::Client::new();
        let res = client
            .get(&url)
            .send()
            .await
            .unwrap();
        assert_eq!(&res.status().as_str()[..], "200");
        let body = &res.text().await.unwrap();
        assert!(&body.len() > &(100 as usize));
    }
}
