use serenity::async_trait;
use serenity::builder::CreateApplicationCommand;
use serenity::client::Context;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;

use super::super::services;
use super::SlashCommand;

pub struct Join {}
#[async_trait]
impl SlashCommand for Join {
    async fn run(ctx: &Context, command: &ApplicationCommandInteraction) -> Option<String> {
        let guild = ctx.cache.guild(command.guild_id.unwrap()).unwrap();
        use crate::handler::usecase::text_to_speech::speech_options;
        services::join::join(
            ctx,
            guild,
            &command.user.id,
            command.channel_id,
            speech_options::SpeechOptions::default(),
        )
        .await
        .unwrap()
        .into()
    }

    fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
        command.description("Join your voice channel to use tts.")
    }
}
