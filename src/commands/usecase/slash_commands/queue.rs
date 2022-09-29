use serenity::async_trait;
use serenity::builder::CreateApplicationCommand;
use serenity::client::Context;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;

use super::super::services;
use super::{SlashCommand, SlashCommandResult};

pub struct Queue {}
#[async_trait]
impl SlashCommand for Queue {
    async fn run(ctx: &Context, command: &ApplicationCommandInteraction) -> SlashCommandResult {
        match services::queue::queue(ctx, command.guild_id.unwrap()).await {
            Ok(queue) => SlashCommandResult::Embed(services::queue::create_queue_embed(&queue, 0)),
            Err(e) => SlashCommandResult::Simple(Some(e.to_string())),
        }
    }
    fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
        command.description("List the current queue.")
    }
}
