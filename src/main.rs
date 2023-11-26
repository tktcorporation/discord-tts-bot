use std::env;

use serenity::{client::Client, prelude::GatewayIntents};

use songbird::SerenityInit;

mod handler;
use handler::Handler;

mod infrastructure;

mod commands;

mod model;

mod constants;

mod framework;
use framework::build_framework;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let framework = build_framework();

    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    let mut client = build_client(&token, framework, intents)
        .await
        .expect("Err creating client");

    let _ = client
        .start()
        .await
        .map_err(|why| println!("Client ended: {why:?}"));
}

async fn build_client(
    token: &str,
    framework: serenity::framework::StandardFramework,
    intents: GatewayIntents,
) -> Result<serenity::Client, serenity::Error> {
    Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .register_songbird()
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
