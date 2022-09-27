use serenity::async_trait;
use serenity::builder::CreateApplicationCommand;
use serenity::client::Context;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;

use super::super::services;
use super::SlashCommand;

pub struct Leave {}
#[async_trait]
impl SlashCommand for Leave {
    async fn run(ctx: &Context, command: &ApplicationCommandInteraction) -> Option<String> {
        services::leave(ctx, command.guild_id.unwrap())
            .await
            .unwrap()
            .into()
    }
    fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
        command.description("Leave the voice channel.")
    }
}
