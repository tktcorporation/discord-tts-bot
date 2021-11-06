pub mod join;
use serenity::{model::channel::Message, Result as SerenityResult};
mod invite;
pub use invite::invite;
mod queue;
pub use queue::queue;
mod play;
pub use play::play;
mod clear;
pub use clear::clear;

/// Checks that a message successfully sent; if not, then logs why to stdout.
pub fn check_msg(result: SerenityResult<Message>) {
    if let Err(why) = result {
        println!("Error sending message: {:?}", why);
    }
}
