use serenity::async_trait;

mod clear;
mod invite;
mod join;
mod leave;
mod ping;
mod play;
mod queue;
mod repeat;
mod select_channel;
mod skip;

// use serenity::builder::{CreateCommand, CreateEmbed};
use serenity::builder::{CreateCommand, CreateCommandOption, CreateEmbed};
use serenity::client::Context;
// use serenity::model::application::interaction::application_command::CommandInteraction;
use serenity::model::application::{CommandInteraction, CommandOptionType};

pub enum SlashCommandResult {
    Simple(Option<String>),
    Embed(CreateEmbed),
}

#[async_trait]
pub trait SlashCommand {
    async fn run(ctx: &Context, command: &CommandInteraction) -> SlashCommandResult;
    fn register(command: CreateCommand) -> CreateCommand;
}

pub enum SlashCommands {
    Clear,
    Join,
    Leave,
    Ping,
    Play,
    Invite,
    Skip,
    Queue,
    Repeat,
    SelectChannel,
}

impl SlashCommands {
    pub fn from_str(command: &str) -> Option<Self> {
        match command {
            "clear" => Some(Self::Clear),
            "join" => Some(Self::Join),
            "leave" => Some(Self::Leave),
            "ping" => Some(Self::Ping),
            "play" => Some(Self::Play),
            "invite" => Some(Self::Invite),
            "skip" => Some(Self::Skip),
            "queue" => Some(Self::Queue),
            "repeat" => Some(Self::Repeat),
            "select_channel" => Some(Self::SelectChannel),
            _ => None,
        }
    }

    pub async fn run(
        &self,
        ctx: &Context,
        command: &CommandInteraction,
    ) -> SlashCommandResult {
        match self {
            Self::Clear => clear::Clear::run(ctx, command).await,
            Self::Join => join::Join::run(ctx, command).await,
            Self::Leave => leave::Leave::run(ctx, command).await,
            Self::Ping => ping::Ping::run(ctx, command).await,
            Self::Play => play::Play::run(ctx, command).await,
            Self::Invite => invite::Invite::run(ctx, command).await,
            Self::Skip => skip::Skip::run(ctx, command).await,
            Self::Queue => queue::Queue::run(ctx, command).await,
            Self::Repeat => repeat::Repeat::run(ctx, command).await,
            Self::SelectChannel => select_channel::SelectChannel::run(ctx, command).await,
        }
    }

    pub fn register<'a>(
        &self,
        command: CreateCommand,
    ) -> CreateCommand {
        match self {
            Self::Clear => clear::Clear::register(command).name("clear"),
            Self::Join => join::Join::register(command).name("join"),
            Self::Leave => leave::Leave::register(command).name("leave"),
            Self::Ping => ping::Ping::register(command).name("ping"),
            Self::Play => play::Play::register(command).name("play"),
            Self::Invite => invite::Invite::register(command).name("invite"),
            Self::Skip => skip::Skip::register(command).name("skip"),
            Self::Queue => queue::Queue::register(command).name("queue"),
            Self::Repeat => repeat::Repeat::register(command).name("repeat"),
            Self::SelectChannel => {
                select_channel::SelectChannel::register(command).name("select_channel")
            }
        }
    }
}
