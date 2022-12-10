use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
};

use super::services::{self, check_msg};

#[command]
async fn deafen(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let comment = match services::deafen(ctx, guild_id).await {
        Ok(comment) => comment,
        Err(e) => {
            format!("{e:?}")
        }
    };
    check_msg(msg.reply(&ctx, comment).await);
    Ok(())
}
