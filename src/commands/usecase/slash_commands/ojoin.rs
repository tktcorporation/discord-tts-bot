use serenity::async_trait;
use serenity::builder::CreateApplicationCommand;
use serenity::client::Context;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;

use super::super::services;
use super::{SlashCommand, SlashCommandResult};

pub struct Ojoin {}
#[async_trait]
impl SlashCommand for Ojoin {
    async fn run(ctx: &Context, command: &ApplicationCommandInteraction) -> SlashCommandResult {
        let guild = ctx.cache.guild(command.guild_id.unwrap()).unwrap();
        use crate::handler::usecase::text_to_speech::speech_options;
        match services::join(
            ctx,
            guild,
            &command.user.id,
            command.channel_id,
            speech_options::SpeechOptions { is_ojosama: true },
        )
        .await
        {
            Ok(s) => SlashCommandResult::Simple(Some(s)),
            Err(e) => SlashCommandResult::Simple(Some(e.to_string())),
        }
    }

    fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
        command.description("Join your voice channel to use tts in Ojosama.")
    }
}
