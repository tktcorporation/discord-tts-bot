use serenity::async_trait;
use serenity::builder::CreateApplicationCommand;
use serenity::client::Context;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;

use super::super::services;
use super::SlashCommand;

pub struct Invite {}
#[async_trait]
impl SlashCommand for Invite {
    async fn run(ctx: &Context, _command: &ApplicationCommandInteraction) -> Option<String> {
        match services::invite(ctx).await {
            Ok(s) => Some(s),
            Err(e) => Some(e.to_string()),
        }
    }
    fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
        command.description("Invite the bot to your server.")
    }
}
