use serenity::async_trait;
use serenity::client::Context;
use serenity::builder::CreateCommand;
use serenity::model::application::CommandInteraction;

use super::super::services;
use super::{SlashCommand, SlashCommandResult};

pub struct Invite {}
#[async_trait]
impl SlashCommand for Invite {
    async fn run(ctx: &Context, _command: &CommandInteraction) -> SlashCommandResult {
        match services::invite(ctx).await {
            Ok(s) => SlashCommandResult::Simple(Some(s)),
            Err(e) => SlashCommandResult::Simple(Some(e.to_string())),
        }
    }
    fn register(command: CreateCommand) -> CreateCommand {
        command.description("Invite the bot to your server.")
    }
}
