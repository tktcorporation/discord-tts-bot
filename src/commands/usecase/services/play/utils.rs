use songbird::input::Restartable;

pub async fn source_from_str(
    value: String,
    lazy: bool,
) -> Result<Restartable, songbird::input::error::Error> {
    if value.starts_with("http") {
        Restartable::ytdl(value.clone(), lazy).await
    } else {
        Restartable::ytdl_search(value, lazy).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_source_from_str() {
        source_from_str("Sample".to_string(), false).await.unwrap();
    }

    #[tokio::test]
    async fn test_source_from_url() {
        source_from_str(
            "https://www.youtube.com/watch?v=rvkxtVkvawc".to_string(),
            false,
        )
        .await
        .unwrap();
    }

    #[tokio::test]
    async fn test_source_from_str_lazy() {
        source_from_str("Sample".to_string(), false).await.unwrap();
    }

    #[tokio::test]
    async fn test_source_from_url_lazy() {
        source_from_str(
            "https://www.youtube.com/watch?v=rvkxtVkvawc".to_string(),
            false,
        )
        .await
        .unwrap();
    }
}
