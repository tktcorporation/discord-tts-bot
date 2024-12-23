use serenity::async_trait;
use serenity::builder::CreateCommand;
use serenity::client::Context;
use serenity::model::application::CommandInteraction;

use super::super::services;
use super::{SlashCommand, SlashCommandResult};

pub struct Queue {}
#[async_trait]
impl SlashCommand for Queue {
    async fn run(ctx: &Context, command: &CommandInteraction) -> SlashCommandResult {
        match services::queue::queue(ctx, command.guild_id.unwrap()).await {
            Ok(queue) => SlashCommandResult::Embed(Box::new(services::queue::create_queue_embed(&queue, 0))),
            Err(e) => SlashCommandResult::Simple(Some(e.to_string())),
        }
    }
    fn register(command: CreateCommand) -> CreateCommand {
        command.description("List the current queue.")
    }
}
