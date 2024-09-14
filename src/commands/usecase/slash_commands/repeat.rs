use serenity::async_trait;
use serenity::client::Context;
use serenity::builder::CreateCommand;
use serenity::model::application::CommandInteraction;

use super::super::services;
use super::{SlashCommand, SlashCommandResult};

pub struct Repeat {}
#[async_trait]
impl SlashCommand for Repeat {
    async fn run(ctx: &Context, command: &CommandInteraction) -> SlashCommandResult {
        match services::repeat(ctx, command.guild_id.unwrap()).await {
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
