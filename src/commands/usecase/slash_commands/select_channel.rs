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
        let channel_id = match resolved_options.first() {
            Some(ResolvedOption {
                value: ResolvedValue::Channel(channel),
                ..
            }) => channel.id,
            None => command.channel_id,
            _ => return SlashCommandResult::Simple(Some("Invalid channel provided".to_string())),
        };

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
                    "channel to speech (defaults to current channel if not specified)",
                )
                .required(false),
            )
    }
}
