use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
};

use super::services::{self, check_msg};

#[command]
#[only_in(guilds)]
#[description = "Clear all queue."]
#[aliases("stop")]
async fn clear(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let guild_id = msg.guild_id.unwrap();
    match services::clear(ctx, guild_id).await {
        Ok(comment) => {
            check_msg(msg.reply(&ctx, comment).await);
        }
        Err(e) => {
            check_msg(msg.reply(&ctx, format!("{e:?}")).await);
        }
    }
    Ok(())
}
