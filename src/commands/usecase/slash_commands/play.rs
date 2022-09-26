use serenity::builder::CreateApplicationCommand;
use serenity::{
    client::Context,
    model::application::{
        command::CommandOptionType,
        interaction::application_command::{ApplicationCommandInteraction, CommandDataOptionValue},
    },
};

use super::super::services;

pub async fn run(ctx: &Context, command: &ApplicationCommandInteraction) -> String {
    let url_option = command
        .data
        .options
        .get(0)
        .expect("url option is required")
        .resolved
        .clone()
        .unwrap();
    let url = match url_option {
        CommandDataOptionValue::String(url) => url.clone(),
        _ => {
            return "Must provide a URL to a video or audio".to_string();
        }
    };
    match services::play(ctx, command.guild_id.unwrap(), command.channel_id, &url).await {
        Ok(_) => "ok".to_string(),
        Err(e) => format!("{:?}", e),
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("play")
        .description("play music")
        .create_option(|option| {
            option
                .name("url")
                .description("url or search query")
                .kind(CommandOptionType::String)
                .required(true)
        })
}
