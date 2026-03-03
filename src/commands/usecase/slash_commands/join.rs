use serenity::async_trait;
use serenity::builder::CreateCommand;
use serenity::client::Context;
use serenity::model::application::CommandInteraction;

use super::super::services;
use super::{SlashCommand, SlashCommandResult};

pub struct Join {}
#[async_trait]
impl SlashCommand for Join {
    async fn run(ctx: &Context, command: &CommandInteraction) -> SlashCommandResult {
        let guild_id = match command.guild_id {
            Some(id) => id,
            None => return SlashCommandResult::Simple(Some("This command can only be used in a server.".to_string())),
        };
        let guild = match guild_id.to_guild_cached(ctx) {
            Some(g) => g.clone(),
            None => return SlashCommandResult::Simple(Some("Failed to find server info.".to_string())),
        };
        use crate::handler::usecase::text_to_speech::speech_options;
        match services::join(
            ctx,
            guild,
            &command.user.id,
            command.channel_id,
            speech_options::SpeechOptions::default(),
        )
        .await
        {
            Ok(s) => SlashCommandResult::Simple(Some(s)),
            Err(e) => SlashCommandResult::Simple(Some(e.to_string())),
        }
    }

    fn register(command: CreateCommand) -> CreateCommand {
        command.description("Join your voice channel to use tts.")
    }
}
