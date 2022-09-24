use serenity::builder::CreateApplicationCommand;
use serenity::client::Context;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;

pub use crate::commands::usecase;

pub async fn run(ctx: &Context, command: &ApplicationCommandInteraction) -> String {
    let guild = ctx.cache.guild(command.guild_id.unwrap()).unwrap();
    use crate::handler::usecase::text_to_speech::speech_options;
    usecase::join::join(
        ctx,
        guild,
        &command.user.id,
        command.channel_id,
        speech_options::SpeechOptions::default(),
    )
    .await
    .unwrap()
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("join").description("join voice channel")
}
