use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
};

use super::services::{self, check_msg};

#[command]
#[description = "Join your voice channel to use tts."]
#[only_in(guilds)]
async fn ojoin(ctx: &Context, msg: &Message) -> CommandResult {
    use crate::handler::usecase::text_to_speech::speech_options;
    let guild = msg.guild(&ctx.cache).unwrap();
    match services::join(
        ctx,
        guild,
        &msg.author.id,
        msg.channel_id,
        speech_options::SpeechOptions {
            is_ojosama: true,
            read_channel_id: None,
        },
    )
    .await
    {
        Ok(comment) => {
            check_msg(msg.reply(&ctx, comment).await);
        }
        Err(e) => {
            check_msg(msg.reply(&ctx, format!("{e:?}")).await);
        }
    }
    Ok(())
}
