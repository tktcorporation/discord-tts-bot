use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
};

use super::services;

#[command]
#[description = "List of queue."]
#[aliases("q")]
#[only_in(guilds)]
async fn queue(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    match services::queue(ctx, guild.id).await {
        Ok(queue) => {
            msg.channel_id
                .send_message(ctx.http.clone(), |m| {
                    m.embed(|e| {
                        e.title("List Queue");
                        for (i, val) in queue.iter().enumerate() {
                            e.field(
                                ".",
                                format!(
                                    "`{}` {}",
                                    i + 1,
                                    val.metadata().title.as_ref().unwrap_or(&String::from(""))
                                ),
                                false,
                            );
                        }
                        e
                    });
                    m
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
