use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
};

use super::services::{self, check_msg};

#[command]
#[description = "Give a URL to invite this bot."]
pub async fn invite(ctx: &Context, msg: &Message) -> CommandResult {
    let comment = match services::invite(ctx).await {
        Ok(s) => s,
        Err(e) => format!("{e:?}"),
    };
    check_msg(msg.reply(&ctx, comment).await);
    Ok(())
}
