use serenity::async_trait;
use serenity::builder::CreateCommand;
use serenity::client::Context;
use serenity::model::application::CommandInteraction;

use super::super::services;
use super::{SlashCommand, SlashCommandResult};

pub struct Leave {}
#[async_trait]
impl SlashCommand for Leave {
    async fn run(ctx: &Context, command: &CommandInteraction) -> SlashCommandResult {
        SlashCommandResult::Simple(
            services::leave(ctx, command.guild_id.unwrap())
                .await
                .unwrap()
                .into(),
        )
    }
    fn register(command: CreateCommand) -> CreateCommand {
        command.description("Leave the voice channel.")
    }
}
