use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
};

use super::services::{self, check_msg};

#[command]
#[description = "Play a song from youtube."]
#[aliases("p")]
#[only_in(guilds)]
async fn play(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    use services::error::Error;
    match services::play(ctx, msg, args).await {
        Ok(_) => {}
        Err(s) => match s {
            Error::NotInVoiceChannel => {
                use crate::handler::usecase::text_to_speech::speech_options;
                let guild = msg.guild(&ctx.cache).unwrap();
                match services::join::join(
                    ctx,
                    guild,
                    &msg.author.id,
                    msg.channel_id,
                    speech_options::SpeechOptions { is_ojosama: true },
                )
                .await
                {
                    Ok(comment) => {
                        check_msg(msg.reply(&ctx, comment).await);
                        check_msg(
                            msg.reply(ctx, "I joined the channel. Please use play command again.")
                                .await,
                        );
                    }
                    Err(e) => {
                        check_msg(msg.reply(&ctx, format!("{:?}", e)).await);
                    }
                }
            }
            Error::ErrorSourcingFfmpeg => {
                check_msg(msg.reply(ctx, Error::ErrorSourcingFfmpeg).await);
            }
        },
    };
    Ok(())
}
