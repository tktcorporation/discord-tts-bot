use serenity::{model::channel::Message, Result as SerenityResult};
use std::time::Duration;

mod clear;
mod deafen;
pub mod error;
mod invite;
mod join;
mod leave;
mod mute;
mod play;
pub mod queue;
mod skip;

pub use clear::clear;
pub use deafen::deafen;
pub use invite::invite;
pub use join::join;
pub use leave::leave;
pub use mute::mute;
pub use play::play;
pub use skip::skip;

/// Checks that a message successfully sent; if not, then logs why to stdout.
pub fn check_msg(result: SerenityResult<Message>) {
    if let Err(why) = result {
        println!("Error sending message: {:?}", why);
    }
}

pub fn get_human_readable_timestamp(duration: Option<Duration>) -> String {
    match duration {
        Some(duration) if duration == Duration::MAX => "∞".to_string(),
        Some(duration) => {
            let seconds = duration.as_secs() % 60;
            let minutes = (duration.as_secs() / 60) % 60;
            let hours = duration.as_secs() / 3600;

            if hours < 1 {
                format!("{:02}:{:02}", minutes, seconds)
            } else {
                format!("{}:{:02}:{:02}", hours, minutes, seconds)
            }
        }
        None => "∞".to_string(),
    }
}
