use super::services::{self};

mod help;
pub use help::{COMMANDS_GROUP, COMMANDS_GROUP_OPTIONS};

mod bgm;
mod clear;
mod deafen;
mod invite;
mod join;
mod leave;
mod mute;
mod ping;
mod play;
mod play_fade;
mod queue;
mod skip;
mod undeafen;
mod unmute;

use super::services::check_msg;
use serenity::{model::channel::Message, prelude::Context};

pub async fn command_reply(result: Result<String, String>, ctx: &Context, msg: &Message) {
    match result {
        Ok(comment) => {
            check_msg(msg.reply(&ctx, comment).await);
        }
        Err(e) => {
            check_msg(msg.reply(&ctx, format!("{e:?}")).await);
        }
    }
}
