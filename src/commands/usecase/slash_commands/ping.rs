use serenity::async_trait;
use serenity::client::Context;
use serenity::builder::CreateCommand;
use serenity::model::application::CommandInteraction;

use super::{SlashCommand, SlashCommandResult};

pub struct Ping {}
#[async_trait]
impl SlashCommand for Ping {
    async fn run(_ctx: &Context, _command: &CommandInteraction) -> SlashCommandResult {
        SlashCommandResult::Simple("Hey, I'm alive!".to_string().into())
    }
    fn register(command: CreateCommand) -> CreateCommand {
        command.description("Check if the bot is alive.")
    }
}
