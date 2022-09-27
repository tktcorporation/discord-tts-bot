use serenity::async_trait;
use serenity::builder::CreateApplicationCommand;
use serenity::{
    client::Context,
    model::application::{
        command::CommandOptionType,
        interaction::application_command::{ApplicationCommandInteraction, CommandDataOptionValue},
    },
};

use super::super::services;
use super::SlashCommand;

pub struct Play {}
#[async_trait]
impl SlashCommand for Play {
    async fn run(ctx: &Context, command: &ApplicationCommandInteraction) -> Option<String> {
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
                return Some("Must provide a URL to a video or audio".to_string());
            }
        };
        match services::play(ctx, command.guild_id.unwrap(), command.channel_id, &url).await {
            Ok(_) => Some(format!("Queue {}", url)),
            Err(e) => Some(format!("{:?}", e)),
        }
    }

    fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
        command.description("play music").create_option(|option| {
            option
                .name("url or search query")
                .description("url or search query")
                .kind(CommandOptionType::String)
                .required(true)
        })
    }
}
