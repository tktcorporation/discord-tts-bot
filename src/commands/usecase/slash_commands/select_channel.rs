use serenity::async_trait;
use serenity::builder::CreateApplicationCommand;
use serenity::{
    self,
    client::Context,
    model::application::{
        command::CommandOptionType,
        interaction::application_command::{ApplicationCommandInteraction, CommandDataOptionValue},
    },
};

use super::super::services;
use super::{SlashCommand, SlashCommandResult};

pub struct SelectChannel {}
#[async_trait]
impl SlashCommand for SelectChannel {
    async fn run(_ctx: &Context, command: &ApplicationCommandInteraction) -> SlashCommandResult {
        let channel_id_option = command
            .data
            .options
            .first()
            .expect("channel option is required")
            .resolved
            .clone()
            .unwrap();
        let channel_id = match channel_id_option {
            CommandDataOptionValue::Channel(channel) => channel.id,
            _ => {
                return SlashCommandResult::Simple(Some("Must provide a channel".to_string()));
            }
        };
        let guild_id = command.guild_id.unwrap();
        services::select_channel(&guild_id, channel_id).await;
        SlashCommandResult::Simple(Some(format!("Channel selected <#{channel_id}>")))
    }

    fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
        command
            .description("select a channel to speech")
            .create_option(|option| {
                option
                    .name("channel")
                    .description("put a channel")
                    .kind(CommandOptionType::Channel)
                    .required(true)
            })
    }
}
