use serenity::async_trait;
use serenity::client::Context;
use serenity::builder::CreateCommand;
use serenity::model::application::CommandInteraction;

use super::super::services;
use super::{SlashCommand, SlashCommandResult};

pub struct Skip {}
#[async_trait]
impl SlashCommand for Skip {
    async fn run(ctx: &Context, command: &CommandInteraction) -> SlashCommandResult {
        match services::skip(ctx, command.guild_id.unwrap()).await {
            Ok(s) => SlashCommandResult::Simple(Some(s)),
            Err(e) => SlashCommandResult::Simple(Some(e.to_string())),
        }
    }
    fn register(command: CreateCommand) -> CreateCommand {
        command.description("Skip the current queue.")
    }
}
