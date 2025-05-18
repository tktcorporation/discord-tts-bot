use serenity::all::{CommandOptionType, ResolvedOption, ResolvedValue};
use serenity::async_trait;
use serenity::builder::{CreateCommand, CreateCommandOption};
use serenity::client::Context;
use serenity::model::application::CommandInteraction;

use super::super::services;
use super::{SlashCommand, SlashCommandResult};

pub struct Play {}
#[async_trait]
impl SlashCommand for Play {
    async fn run(ctx: &Context, command: &CommandInteraction) -> SlashCommandResult {
        // Extract and clone necessary data to avoid holding non-Send references across awaits
        let resolved_options = command.data.options();
        let url = match resolved_options.first().unwrap() {
            ResolvedOption {
                value: ResolvedValue::String(url),
                ..
            } => url,
            _ => {
                return SlashCommandResult::Simple(Some(
                    "Must provide a URL to a video or audio".to_string(),
                ))
            }
        };
        let guild_id = command.guild_id.unwrap();

        match services::play(ctx, guild_id, command.channel_id, url).await {
            Ok(_) => SlashCommandResult::Simple(Some(format!("Queue {url}"))),
            Err(e) => match e {
                services::error::Error::NotInVoiceChannel => {
                    use crate::handler::usecase::text_to_speech::speech_options;
                    // Clone the Guild to avoid holding a reference across await
                    let guild = ctx.cache.guild(guild_id).unwrap().clone();

                    let joined_message = match services::join(
                        ctx,
                        guild,
                        &command.user.id,
                        command.channel_id,
                        speech_options::SpeechOptions::default(),
                    )
                    .await
                    {
                        Ok(s) => s,
                        Err(e) => return SlashCommandResult::Simple(Some(e.to_string())),
                    };

                    if let Err(e) = services::play(ctx, guild_id, command.channel_id, url).await {
                        return SlashCommandResult::Simple(Some(e.to_string()));
                    };

                    SlashCommandResult::Simple(Some(format!("{joined_message} and Queue {url}")))
                }
                _ => SlashCommandResult::Simple(Some(e.to_string())),
            },
        }
    }

    fn register(command: CreateCommand) -> CreateCommand {
        command.description("play music").add_option(
            CreateCommandOption::new(CommandOptionType::String, "url", "url or search query")
                .required(true),
        )
    }
}
