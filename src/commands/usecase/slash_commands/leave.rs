use serenity::builder::CreateApplicationCommand;
use serenity::client::Context;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;

use super::super::services;

pub async fn run(ctx: &Context, command: &ApplicationCommandInteraction) -> String {
    services::leave(ctx, command.guild_id.unwrap())
        .await
        .unwrap()
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("leave").description("leave voice channel")
}
