use serenity::async_trait;

mod clear;
mod deafen;
mod invite;
mod join;
mod leave;
mod mute;
mod ojoin;
mod ping;
mod play;
mod queue;
mod skip;
mod undeafen;
mod unmute;

use serenity::builder::{CreateApplicationCommand, CreateEmbed};
use serenity::client::Context;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;

pub enum SlashCommandResult {
    Simple(Option<String>),
    Embed(CreateEmbed),
}

#[async_trait]
pub trait SlashCommand {
    async fn run(ctx: &Context, command: &ApplicationCommandInteraction) -> SlashCommandResult;
    fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand;
}

pub enum SlashCommands {
    Clear,
    Join,
    Leave,
    Ping,
    Play,
    Deafen,
    Mute,
    Invite,
    Skip,
    Queue,
    Undeafen,
    Unmute,
    Ojoin,
}

impl SlashCommands {
    pub fn from_str(command: &str) -> Option<Self> {
        match command {
            "clear" => Some(Self::Clear),
            "join" => Some(Self::Join),
            "leave" => Some(Self::Leave),
            "ping" => Some(Self::Ping),
            "play" => Some(Self::Play),
            "deafen" => Some(Self::Deafen),
            "mute" => Some(Self::Mute),
            "invite" => Some(Self::Invite),
            "skip" => Some(Self::Skip),
            "queue" => Some(Self::Queue),
            _ => None,
        }
    }

    pub async fn run(
        &self,
        ctx: &Context,
        command: &ApplicationCommandInteraction,
    ) -> SlashCommandResult {
        match self {
            Self::Clear => clear::Clear::run(ctx, command).await,
            Self::Join => join::Join::run(ctx, command).await,
            Self::Leave => leave::Leave::run(ctx, command).await,
            Self::Ping => ping::Ping::run(ctx, command).await,
            Self::Play => play::Play::run(ctx, command).await,
            Self::Deafen => deafen::Deafen::run(ctx, command).await,
            Self::Mute => mute::Mute::run(ctx, command).await,
            Self::Invite => invite::Invite::run(ctx, command).await,
            Self::Skip => skip::Skip::run(ctx, command).await,
            Self::Queue => queue::Queue::run(ctx, command).await,
            Self::Undeafen => undeafen::Undeafen::run(ctx, command).await,
            Self::Unmute => unmute::Unmute::run(ctx, command).await,
            Self::Ojoin => ojoin::Ojoin::run(ctx, command).await,
        }
    }

    pub fn register<'a>(
        &self,
        command: &'a mut CreateApplicationCommand,
    ) -> &'a mut CreateApplicationCommand {
        match self {
            Self::Clear => clear::Clear::register(command).name("clear"),
            Self::Join => join::Join::register(command).name("join"),
            Self::Leave => leave::Leave::register(command).name("leave"),
            Self::Ping => ping::Ping::register(command).name("ping"),
            Self::Play => play::Play::register(command).name("play"),
            Self::Deafen => deafen::Deafen::register(command).name("deafen"),
            Self::Mute => mute::Mute::register(command).name("mute"),
            Self::Invite => invite::Invite::register(command).name("invite"),
            Self::Skip => skip::Skip::register(command).name("skip"),
            Self::Queue => queue::Queue::register(command).name("queue"),
            Self::Undeafen => undeafen::Undeafen::register(command).name("undeafen"),
            Self::Unmute => unmute::Unmute::register(command).name("unmute"),
            Self::Ojoin => ojoin::Ojoin::register(command).name("ojoin"),
        }
    }
}
