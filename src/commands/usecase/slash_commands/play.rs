use serenity::async_trait;
use serenity::builder::CreateApplicationCommand;
use serenity::{
    self,
    client::Context,
    model::application::{
        command::CommandOptionType,
        interaction::application_command::{ApplicationCommandInteraction, CommandDataOptionValue},
    },
};

use super::super::services;
use super::{SlashCommand, SlashCommandResult};

pub struct Play {}
#[async_trait]
impl SlashCommand for Play {
    async fn run(ctx: &Context, command: &ApplicationCommandInteraction) -> SlashCommandResult {
        let url_option = command
            .data
            .options
            .first()
            .expect("url option is required")
            .resolved
            .clone()
            .unwrap();
        let url = match url_option {
            CommandDataOptionValue::String(url) => url.clone(),
            _ => {
                return SlashCommandResult::Simple(Some(
                    "Must provide a URL to a video or audio".to_string(),
                ));
            }
        };
        let guild_id = command.guild_id.unwrap();
        match services::play(ctx, guild_id, command.channel_id, &url).await {
            Ok(_) => SlashCommandResult::Simple(Some(format!("Queue {url}"))),
            Err(e) => match e {
                services::error::Error::NotInVoiceChannel => {
                    use crate::handler::usecase::text_to_speech::speech_options;
                    let joined_message = match services::join(
                        ctx,
                        ctx.cache.guild(guild_id).unwrap(),
                        &command.user.id,
                        command.channel_id,
                        speech_options::SpeechOptions::default(),
                    )
                    .await
                    {
                        Ok(s) => s,
                        Err(e) => return SlashCommandResult::Simple(Some(e.to_string())),
                    };
                    if let Err(e) = services::play(ctx, guild_id, command.channel_id, &url).await {
                        return SlashCommandResult::Simple(Some(e.to_string()));
                    };
                    SlashCommandResult::Simple(Some(
                        joined_message + format!(" and Queue {url}").as_str(),
                    ))
                }
                _ => SlashCommandResult::Simple(Some(e.to_string())),
            },
        }
    }

    fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
        command.description("play music").create_option(|option| {
            option
                .name("url")
                .description("url or search query")
                .kind(CommandOptionType::String)
                .required(true)
        })
    }
}
