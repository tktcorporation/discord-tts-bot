use serenity::all::{CommandOptionType, ResolvedOption, ResolvedValue};
use serenity::async_trait;
use serenity::builder::{CreateCommand, CreateCommandOption};
use serenity::client::Context;
use serenity::model::application::CommandInteraction;

use super::super::services;
use super::{SlashCommand, SlashCommandResult};

pub struct SelectChannel {}
#[async_trait]
impl SlashCommand for SelectChannel {
    async fn run(_ctx: &Context, command: &CommandInteraction) -> SlashCommandResult {
        let resolved_options = command.data.options();
        let channel = match resolved_options.first().unwrap() {
            ResolvedOption {
                value: ResolvedValue::Channel(channel),
                ..
            } => channel.clone(),
            _ => return SlashCommandResult::Simple(Some("Must provide a channel".to_string())),
        };
        let channel_id = channel.id;

        let guild_id = command.guild_id.unwrap();
        services::select_channel(&guild_id, channel_id).await;
        SlashCommandResult::Simple(Some(format!("Channel selected <#{channel_id}>")))
    }

    fn register(command: CreateCommand) -> CreateCommand {
        command
            .description("select a channel to speech")
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::Channel,
                    "channel",
                    "channel to speech",
                )
                .required(true),
            )
    }
}
