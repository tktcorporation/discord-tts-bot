use serenity::{model::channel::Message, Result as SerenityResult};

mod clear;
mod deafen;
pub mod error;
mod invite;
mod join;
mod leave;
mod mute;
mod play;
mod queue;
mod skip;

pub use clear::clear;
pub use deafen::deafen;
pub use invite::invite;
pub use join::join;
pub use leave::leave;
pub use mute::mute;
pub use play::play;
pub use queue::queue;
pub use skip::skip;

/// Checks that a message successfully sent; if not, then logs why to stdout.
pub fn check_msg(result: SerenityResult<Message>) {
    if let Err(why) = result {
        println!("Error sending message: {:?}", why);
    }
}
