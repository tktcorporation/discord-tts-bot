use serenity::async_trait;
use serenity::builder::CreateApplicationCommand;
use serenity::client::Context;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;

use super::{SlashCommand, SlashCommandResult};

pub struct Ping {}
#[async_trait]
impl SlashCommand for Ping {
    async fn run(_ctx: &Context, _command: &ApplicationCommandInteraction) -> SlashCommandResult {
        SlashCommandResult::Simple("Hey, I'm alive!".to_string().into())
    }
    fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
        command.description("Check if the bot is alive.")
    }
}
