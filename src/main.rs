use std::{
    env,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::Duration,
};

use serenity::{
    async_trait,
    client::{Client, Context, EventHandler},
    framework::{
        standard::{
            macros::{command, group},
            Args, CommandResult,
        },
        StandardFramework,
    },
    http::Http,
    model::{channel::Message, gateway::Ready, misc::Mentionable, prelude::ChannelId},
    Result as SerenityResult,
};

use songbird::{
    input::{self, restartable::Restartable},
    Event, EventContext, EventHandler as VoiceEventHandler, SerenityInit, TrackEvent,
};

mod handler;
use handler::Handler;

mod services;
use services::check_msg;

mod commands;
use commands::GENERAL_GROUP;

// #[group]
// #[commands(
//     deafen, join, leave, mute, play_fade, queue, skip, stop, ping, undeafen, unmute
// )]
// pub struct General;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~"))
        .group(&GENERAL_GROUP);

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .framework(framework)
        .register_songbird()
        .await
        .expect("Err creating client");

    let _ = client
        .start()
        .await
        .map_err(|why| println!("Client ended: {:?}", why));
}
