use serenity::all::{CommandOptionType, ResolvedOption, ResolvedValue};
use serenity::async_trait;
use serenity::builder::{CreateCommand, CreateCommandOption};
use serenity::client::Context;
use serenity::model::application::CommandInteraction;

use super::super::services;
use super::{SlashCommand, SlashCommandResult};

pub struct JoinSelect {}

#[async_trait]
impl SlashCommand for JoinSelect {
    async fn run(ctx: &Context, command: &CommandInteraction) -> SlashCommandResult {
        let resolved_options = command.data.options();
        let channel_id = match resolved_options.first() {
            Some(ResolvedOption {
                value: ResolvedValue::Channel(channel),
                ..
            }) => channel.id,
            None => command.channel_id,
            _ => return SlashCommandResult::Simple(Some("Invalid channel provided".to_string())),
        };

        let guild_id = match command.guild_id {
            Some(id) => id,
            None => return SlashCommandResult::Simple(Some("This command can only be used in a server.".to_string())),
        };
        let guild = match guild_id.to_guild_cached(ctx) {
            Some(g) => g.clone(),
            None => return SlashCommandResult::Simple(Some("Failed to find server info.".to_string())),
        };
        use crate::handler::usecase::text_to_speech::speech_options;
        match services::join(
            ctx,
            guild,
            &command.user.id,
            command.channel_id,
            speech_options::SpeechOptions::default(),
        )
        .await
        {
            Ok(join_msg) => {
                services::select_channel(&guild_id, channel_id).await;
                SlashCommandResult::Simple(Some(format!(
                    "{join_msg}\nChannel selected <#{channel_id}>"
                )))
            }
            Err(e) => SlashCommandResult::Simple(Some(e.to_string())),
        }
    }

    fn register(command: CreateCommand) -> CreateCommand {
        command
            .description("Join voice channel and select text channel")
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
