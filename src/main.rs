use std::env;

use serenity::{client::Client, prelude::GatewayIntents};
use reqwest::Client as HttpClient;
use songbird::typemap::TypeMapKey;

use songbird::SerenityInit;

mod handler;
use handler::Handler;

mod infrastructure;

mod commands;

mod model;

mod constants;

pub struct HttpKey;
impl TypeMapKey for HttpKey {
    type Value = HttpClient;
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // Configure the client with your Discord bot token in the environment.
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
        env::var("DISCORD_CMD_PREFIX").expect("Expected a DISCORD_CMD_PREFIX in the environment");
    }
}
