use serenity::async_trait;
use serenity::client::Context;
use serenity::builder::CreateCommand;
use serenity::model::application::CommandInteraction;

use super::super::services;
use super::{SlashCommand, SlashCommandResult};

pub struct Join {}
#[async_trait]
impl SlashCommand for Join {
    async fn run(ctx: &Context, command: &CommandInteraction) -> SlashCommandResult {
        let guild = command.guild_id.unwrap().to_guild_cached(ctx).unwrap().clone();
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
