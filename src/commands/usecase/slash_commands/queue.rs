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
        let guild_id = match command.guild_id {
            Some(id) => id,
            None => return SlashCommandResult::Simple(Some("This command can only be used in a server.".to_string())),
        };
        match services::queue::queue(ctx, guild_id).await {
            Ok(queue) => {
                SlashCommandResult::Embed(Box::new(services::queue::create_queue_embed(&queue, 0)))
            }
            Err(e) => SlashCommandResult::Simple(Some(e.to_string())),
        }
    }
    fn register(command: CreateCommand) -> CreateCommand {
        command.description("List the current queue.")
    }
}
