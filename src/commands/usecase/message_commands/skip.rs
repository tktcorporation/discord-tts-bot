use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
};

use super::services;

#[command]
#[description = "Skip the current queue."]
#[aliases("s")]
#[only_in(guilds)]
async fn skip(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    match services::skip(ctx, guild_id).await {
        Ok(m) => {
            msg.reply(&ctx.http, m).await?;
        }
        Err(e) => {
            msg.reply(&ctx.http, format!("Error: {:?}", e)).await?;
        }
    }
    Ok(())
}
