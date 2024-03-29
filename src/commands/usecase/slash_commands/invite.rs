use serenity::async_trait;
use serenity::builder::CreateApplicationCommand;
use serenity::client::Context;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;

use super::super::services;
use super::{SlashCommand, SlashCommandResult};

pub struct Invite {}
#[async_trait]
impl SlashCommand for Invite {
    async fn run(ctx: &Context, _command: &ApplicationCommandInteraction) -> SlashCommandResult {
        match services::invite(ctx).await {
            Ok(s) => SlashCommandResult::Simple(Some(s)),
            Err(e) => SlashCommandResult::Simple(Some(e.to_string())),
        }
    }
    fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
        command.description("Invite the bot to your server.")
    }
}
