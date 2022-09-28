use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::{self, channel::Message, guild},
};

use super::services::{self, check_msg};

#[command]
#[description = "Play a song from youtube."]
#[aliases("p")]
#[only_in(guilds)]
async fn play(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let play_url = args.message();
    let comment = join_and_play(ctx, guild, msg.channel_id, &msg.author.id, play_url).await;
    check_msg(msg.reply(&ctx, comment).await);
    Ok(())
}

async fn join_and_play(
    ctx: &Context,
    guild: guild::Guild,
    called_channel_id: model::id::ChannelId,
    caller_user_id: &model::id::UserId,
    play_url: &str,
) -> String {
    use services::error::Error;
    match services::play(ctx, guild.id, called_channel_id, play_url).await {
        Ok(_) => format!("Queue {}", play_url),
        Err(e) => match e {
            Error::NotInVoiceChannel => {
                use crate::handler::usecase::text_to_speech::speech_options;
                let joined_message = match services::join(
                    ctx,
                    guild.clone(),
                    caller_user_id,
                    called_channel_id,
                    speech_options::SpeechOptions::default(),
                )
                .await
                {
                    Ok(s) => s,
                    Err(e) => return e.to_string(),
                };
                if let Err(e) = services::play(ctx, guild.id, called_channel_id, play_url).await {
                    return e.to_string();
                };
                joined_message + format!(" and Queue {}", play_url).as_str()
            }
            _ => e.to_string(),
        },
    }
}
