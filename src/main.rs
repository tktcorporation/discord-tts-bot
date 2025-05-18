use std::env;

use reqwest::Client as HttpClient;
use serenity::{client::Client, prelude::GatewayIntents};
use songbird::SerenityInit;

use discord_tts_bot::handler::Handler;
use discord_tts_bot::model::HttpKey;

mod commands;
mod constants;
mod handler;
mod infrastructure;
mod model;

#[tokio::main]
async fn main() {
    let _guard = sentry::init(sentry::ClientOptions {
        dsn: env::var("SENTRY_DSN").ok().and_then(|dsn| dsn.parse().ok()),
        release: sentry::release_name!(),
        auto_session_tracking: true,
        debug: true,
        sample_rate: 0.005,
        ..Default::default()
    });

    tracing_subscriber::fmt::init();

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    let mut client = build_client(&token, intents)
        .await
        .expect("Err creating client");

    let _ = client
        .start()
        .await
        .map_err(|why| println!("Client ended: {why:?}"));
}

async fn build_client(
    token: &str,
    intents: GatewayIntents,
) -> Result<serenity::Client, serenity::Error> {
    Client::builder(token, intents)
        .event_handler(Handler)
        .register_songbird()
        .type_map_insert::<HttpKey>(HttpClient::new())
        .await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn check_env_exists() {
        env::var("DISCORD_TOKEN").expect("Expected a DISCORD_TOKEN in the environment");
    }
}
