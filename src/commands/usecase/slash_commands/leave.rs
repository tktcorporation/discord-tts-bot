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
        let guild_id = match command.guild_id {
            Some(id) => id,
            None => return SlashCommandResult::Simple(Some("This command can only be used in a server.".to_string())),
        };
        match services::leave(ctx, guild_id).await {
            Ok(msg) => SlashCommandResult::Simple(Some(msg)),
            Err(e) => SlashCommandResult::Simple(Some(e)),
        }
    }
    fn register(command: CreateCommand) -> CreateCommand {
        command.description("Leave the voice channel.")
    }
}
