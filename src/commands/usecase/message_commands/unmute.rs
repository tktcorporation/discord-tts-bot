use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
};

use super::services::{self, check_msg};

#[command]
#[only_in(guilds)]
async fn unmute(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    match services::unmute(ctx, guild_id).await {
        Ok(_) => {
            check_msg(msg.channel_id.say(&ctx.http, "Unmuted").await);
        }
        Err(e) => {
            check_msg(msg.channel_id.say(&ctx.http, e).await);
        }
    };
    Ok(())
}
