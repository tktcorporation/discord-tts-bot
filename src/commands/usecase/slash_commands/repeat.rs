use serenity::async_trait;
use serenity::builder::CreateApplicationCommand;
use serenity::client::Context;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;

use super::super::services;
use super::{SlashCommand, SlashCommandResult};

pub struct Repeat {}
#[async_trait]
impl SlashCommand for Repeat {
    async fn run(ctx: &Context, command: &ApplicationCommandInteraction) -> SlashCommandResult {
        match services::repeat(ctx, command.guild_id.unwrap()).await {
            Ok(is_looping) => SlashCommandResult::Simple(Some(format!(
                "Repeat is now {}",
                if is_looping { "on" } else { "off" }
            ))),
            Err(e) => SlashCommandResult::Simple(Some(e.to_string())),
        }
    }
    fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
        command.description("Repeating the current queue.")
    }
}
