use serenity::async_trait;
use serenity::builder::CreateApplicationCommand;
use serenity::client::Context;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;

use super::super::services;
use super::SlashCommand;

pub struct Mute {}
#[async_trait]
impl SlashCommand for Mute {
    async fn run(ctx: &Context, command: &ApplicationCommandInteraction) -> Option<String> {
        match services::mute(ctx, command.guild_id.unwrap()).await {
            Ok(s) => Some(s),
            Err(e) => Some(e.to_string()),
        }
    }
    fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
        command.description("Mute the bot.")
    }
}
