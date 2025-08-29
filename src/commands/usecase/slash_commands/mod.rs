use serenity::async_trait;
use std::str::FromStr;

mod clear;
mod invite;
mod join;
mod join_select;
mod leave;
mod ping;
mod play;
mod queue;
mod repeat;
mod select_channel;
mod skip;

// use serenity::builder::{CreateCommand, CreateEmbed};
use serenity::builder::{CreateCommand, CreateEmbed};
use serenity::client::Context;
// use serenity::model::application::interaction::application_command::CommandInteraction;
use serenity::model::application::CommandInteraction;

pub enum SlashCommandResult {
    Simple(Option<String>),
    Embed(Box<CreateEmbed>),
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
    JoinSelect,
}

impl SlashCommands {
    pub fn get_commands() -> Vec<CreateCommand> {
        vec![
            Self::Clear.register(),
            Self::Join.register(),
            Self::Leave.register(),
            Self::Ping.register(),
            Self::Play.register(),
            Self::Invite.register(),
            Self::Skip.register(),
            Self::Queue.register(),
            Self::Repeat.register(),
            Self::SelectChannel.register(),
            Self::JoinSelect.register(),
        ]
    }

    pub async fn run(&self, ctx: &Context, command: &CommandInteraction) -> SlashCommandResult {
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
            Self::JoinSelect => join_select::JoinSelect::run(ctx, command).await,
        }
    }

    pub fn register(&self) -> CreateCommand {
        match self {
            Self::Clear => clear::Clear::register(CreateCommand::new("clear")),
            Self::Join => join::Join::register(CreateCommand::new("join")),
            Self::Leave => leave::Leave::register(CreateCommand::new("leave")),
            Self::Ping => ping::Ping::register(CreateCommand::new("ping")),
            Self::Play => play::Play::register(CreateCommand::new("play")),
            Self::Invite => invite::Invite::register(CreateCommand::new("invite")),
            Self::Skip => skip::Skip::register(CreateCommand::new("skip")),
            Self::Queue => queue::Queue::register(CreateCommand::new("queue")),
            Self::Repeat => repeat::Repeat::register(CreateCommand::new("repeat")),
            Self::SelectChannel => {
                select_channel::SelectChannel::register(CreateCommand::new("select_channel"))
            }
            Self::JoinSelect => {
                join_select::JoinSelect::register(CreateCommand::new("join_select"))
            }
        }
    }
}

impl FromStr for SlashCommands {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "clear" => Ok(Self::Clear),
            "join" => Ok(Self::Join),
            "leave" => Ok(Self::Leave),
            "ping" => Ok(Self::Ping),
            "play" => Ok(Self::Play),
            "invite" => Ok(Self::Invite),
            "skip" => Ok(Self::Skip),
            "queue" => Ok(Self::Queue),
            "repeat" => Ok(Self::Repeat),
            "select_channel" => Ok(Self::SelectChannel),
            "join_select" => Ok(Self::JoinSelect),
            _ => Err(()),
        }
    }
}
