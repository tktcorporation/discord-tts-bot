use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
};

use super::services::{self};

#[command]
#[only_in(guilds)]
#[description = "Clear all queue."]
#[aliases("stop")]
async fn clear(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    services::clear(ctx, msg).await
}
