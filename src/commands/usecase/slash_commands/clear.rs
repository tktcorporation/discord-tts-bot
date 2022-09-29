use serenity::async_trait;
use serenity::builder::CreateApplicationCommand;
use serenity::{
    client::Context,
    model::application::interaction::application_command::ApplicationCommandInteraction,
};

use super::super::services;
use super::{SlashCommand, SlashCommandResult};

pub struct Clear {}
#[async_trait]
impl SlashCommand for Clear {
    async fn run(ctx: &Context, command: &ApplicationCommandInteraction) -> SlashCommandResult {
        match services::clear(ctx, command.guild_id.unwrap()).await {
            Ok(s) => SlashCommandResult::Simple(Some(s)),
            Err(e) => SlashCommandResult::Simple(Some(format!("Error: {:?}", e))),
        }
    }

    fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
        command.description("Clear the queue.")
    }
}
