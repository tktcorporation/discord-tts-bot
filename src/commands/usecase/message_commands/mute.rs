use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
};

use crate::commands::usecase::services;

use super::services::check_msg;

#[command]
#[only_in(guilds)]
async fn mute(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();

    let comment = match services::mute(ctx, guild.id).await {
        Ok(comment) => comment,
        Err(e) => {
            format!("{e:?}")
        }
    };
    check_msg(msg.reply(&ctx, comment).await);
    Ok(())
}
