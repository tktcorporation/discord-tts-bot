use serenity::async_trait;
use serenity::builder::CreateCommand;
use serenity::client::Context;
use serenity::model::application::CommandInteraction;

use super::super::services;
use super::{SlashCommand, SlashCommandResult};

pub struct Repeat {}
#[async_trait]
impl SlashCommand for Repeat {
    async fn run(ctx: &Context, command: &CommandInteraction) -> SlashCommandResult {
        let guild_id = match command.guild_id {
            Some(id) => id,
            None => return SlashCommandResult::Simple(Some("This command can only be used in a server.".to_string())),
        };
        match services::repeat(ctx, guild_id).await {
            Ok(is_looping) => SlashCommandResult::Simple(Some(format!(
                "Repeat is now {}",
                if is_looping { "on" } else { "off" }
            ))),
            Err(e) => SlashCommandResult::Simple(Some(e.to_string())),
        }
    }
    fn register(command: CreateCommand) -> CreateCommand {
        command.description("Repeating the current queue.")
    }
}
