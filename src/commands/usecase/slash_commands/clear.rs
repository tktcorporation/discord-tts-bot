use serenity::async_trait;
use serenity::builder::CreateCommand;
use serenity::client::Context;
use serenity::model::application::CommandInteraction;

use super::super::services;
use super::{SlashCommand, SlashCommandResult};

pub struct Clear {}
#[async_trait]
impl SlashCommand for Clear {
    async fn run(ctx: &Context, command: &CommandInteraction) -> SlashCommandResult {
        match services::clear(ctx, command.guild_id.unwrap()).await {
            Ok(s) => SlashCommandResult::Simple(Some(s)),
            Err(e) => SlashCommandResult::Simple(Some(format!("Error: {e:?}"))),
        }
    }

    fn register(command: CreateCommand) -> CreateCommand {
        command.description("Clear the queue.")
    }
}
