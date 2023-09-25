use super::bgm::BGM_COMMAND;
use super::clear::CLEAR_COMMAND;
use super::deafen::DEAFEN_COMMAND;
use super::invite::INVITE_COMMAND;
use super::join::JOIN_COMMAND;
use super::leave::LEAVE_COMMAND;
use super::mute::MUTE_COMMAND;
use super::ping::PING_COMMAND;
use super::play::PLAY_COMMAND;
use super::play_fade::PLAY_FADE_COMMAND;
use super::queue::QUEUE_COMMAND;
use super::skip::SKIP_COMMAND;
use super::undeafen::UNDEAFEN_COMMAND;
use super::unmute::UNMUTE_COMMAND;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::macros::group;
use serenity::framework::standard::CommandResult;
use serenity::framework::standard::{Args, Delimiter};

use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::Colour;

#[command]
#[description("Shows all commands")]
#[help_available(true)]
async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    let _args = Args::new(&msg.content, &[Delimiter::Single(' ')]);
    msg.author
        .dm(&ctx.http, |m| {
            m.content("Here are all the commands for you :)")
                .embed(|e| {
                    e.title("Help")
                        .colour(Colour::from_rgb(0, 251, 255))
                        .description("All commands available");
                    for command in COMMANDS_GROUP_OPTIONS.commands.to_vec().iter() {
                        let desc = command.options.desc;
                        let name = command.options.names;
                        e.field(
                            name.to_vec()[0].to_string(),
                            desc.unwrap().to_string(),
                            true,
                        );
                    }
                    e.footer(|f| f.text("https://github.com/tktcorporation/discord-tts-bot"))
                })
        })
        .await
        .unwrap();
    msg.channel_id
        .say(
            &ctx.http,
            format!("Command list sent to user: {}", msg.author),
        )
        .await?;

    Ok(())
}

#[cfg(any(feature = "tts", feature = "music"))]
#[group]
#[commands(
    deafen, join, leave, mute, play_fade, play, queue, skip, clear, ping, undeafen, unmute,
    bgm, invite
)]
pub(crate) struct Commands;

#[cfg(not(any(feature = "tts", feature = "music")))]
#[group]
#[commands(join, leave, ping, invite)]
pub(crate) struct Commands;
