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
        let guild_id = match command.guild_id {
            Some(id) => id,
            None => return SlashCommandResult::Simple(Some("This command can only be used in a server.".to_string())),
        };
        match services::clear(ctx, guild_id).await {
            Ok(s) => SlashCommandResult::Simple(Some(s)),
            Err(e) => SlashCommandResult::Simple(Some(format!("Error: {e:?}"))),
        }
    }

    fn register(command: CreateCommand) -> CreateCommand {
        command.description("Clear the queue.")
    }
}
