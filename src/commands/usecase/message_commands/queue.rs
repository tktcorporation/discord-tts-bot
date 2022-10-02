use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
};

use super::services;

#[command]
#[description = "List the songs in the queue."]
#[aliases("q")]
#[only_in(guilds)]
async fn queue(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    match services::queue::queue(ctx, guild.id).await {
        Ok(queue) => {
            msg.channel_id
                .send_message(ctx.http.clone(), |m| {
                    m.set_embed(services::queue::create_queue_embed(&queue, 0))
                })
                .await
                .unwrap();
        }
        Err(e) => {
            msg.reply(&ctx.http, format!("Error: {:?}", e)).await?;
        }
    }
    Ok(())
}
