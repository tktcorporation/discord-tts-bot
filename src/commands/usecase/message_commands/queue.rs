use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
};

use super::services::{self};

#[command]
#[description = "List of queue."]
#[aliases("q")]
#[only_in(guilds)]
async fn queue(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    services::queue(ctx, msg).await
}
