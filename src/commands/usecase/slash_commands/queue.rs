use serenity::async_trait;
use serenity::builder::{CreateApplicationCommand, CreateEmbed};
use serenity::client::Context;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;

use super::super::services;
use super::{SlashCommand, SlashCommandResult};

pub struct Queue {}
#[async_trait]
impl SlashCommand for Queue {
    async fn run(ctx: &Context, command: &ApplicationCommandInteraction) -> SlashCommandResult {
        match services::queue(ctx, command.guild_id.unwrap()).await {
            Ok(queue) => {
                let mut embed = CreateEmbed::default();
                embed.title("List Queue");
                for (i, val) in queue.iter().enumerate() {
                    embed.field(
                        ".",
                        format!(
                            "`{}` {}",
                            i + 1,
                            val.metadata().title.as_ref().unwrap_or(&String::from(""))
                        ),
                        false,
                    );
                }
                SlashCommandResult::Embed(embed)
            }
            Err(e) => SlashCommandResult::Simple(Some(e.to_string())),
        }
    }
    fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
        command.description("List the current queue.")
    }
}
