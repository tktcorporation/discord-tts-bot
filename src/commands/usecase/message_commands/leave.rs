use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
};

use super::command_reply;
use super::services::{self};

#[command]
#[description = "Leave from your voice channel."]
#[only_in(guilds)]
async fn leave(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    command_reply(services::leave(ctx, guild.id).await, ctx, msg).await;
    Ok(())
}
