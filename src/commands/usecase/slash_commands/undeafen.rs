use serenity::async_trait;
use serenity::builder::CreateApplicationCommand;
use serenity::client::Context;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;

use super::super::services;
use super::{SlashCommand, SlashCommandResult};

pub struct Undeafen {}
#[async_trait]
impl SlashCommand for Undeafen {
    async fn run(ctx: &Context, command: &ApplicationCommandInteraction) -> SlashCommandResult {
        match services::undeafen(ctx, command.guild_id.unwrap()).await {
            Ok(_) => SlashCommandResult::Simple(Some("Undeafened".to_string())),
            Err(e) => SlashCommandResult::Simple(Some(e.to_string())),
        }
    }
    fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
        command.description("Deafen the bot.")
    }
}
