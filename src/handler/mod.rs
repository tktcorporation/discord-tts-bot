use serenity::all::EditInteractionResponse;
use serenity::builder::{CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::model::application::{Command, Interaction};
#[cfg(feature = "tts")]
use serenity::model::channel::Message as SerenityMessage;
use serenity::model::voice;
use serenity::{
    async_trait,
    client::{Context, EventHandler},
};

use serenity::model::gateway::Ready;

mod model;
use model::context::Context as Ctx;

use crate::commands::slash_commands::{SlashCommandResult, SlashCommands};
use model::{
    speaker::{ChangeOfStates, CurrentVoiceState, Role},
    voice::Voice,
};
pub mod usecase;
use usecase::set_help_message_to_activity::set_help_message_to_activity;

#[cfg(feature = "tts")]
use usecase::{speech_welcome_see_you::speech_greeting, text_to_speech::text_to_speech};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            println!("Received command interaction: {command:#?}");

            command
                .create_response(
                    &ctx,
                    CreateInteractionResponse::Message(
                        CreateInteractionResponseMessage::new().content("Processing..."),
                    ),
                )
                .await
                .unwrap();

            let command_result = match SlashCommands::from_str(command.data.name.as_str()) {
                Some(slash_command) => slash_command.run(&ctx, &command).await,
                None => {
                    command
                        .edit_response(
                            &ctx.http,
                            EditInteractionResponse::default().content("Unknown command"),
                        )
                        .await
                        .unwrap();
                    return;
                }
            };

            let result = match command_result {
                SlashCommandResult::Simple(None) => {
                    command.delete_response(&ctx.http).await.unwrap();
                    return;
                }
                SlashCommandResult::Simple(Some(message)) => {
                    command
                        .edit_response(
                            &ctx.http,
                            EditInteractionResponse::default().content(message),
                        )
                        .await
                }
                SlashCommandResult::Embed(embed) => {
                    command
                        .edit_response(&ctx.http, EditInteractionResponse::default().embed(*embed))
                        .await
                }
            };
            match result {
                Ok(_) => (),
                Err(e) => {
                    command
                        .edit_response(
                            &ctx,
                            EditInteractionResponse::new().content(format!("{:?}", e)),
                        )
                        .await
                        .unwrap();
                }
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        Command::set_global_commands(
            &ctx.http,
            vec![
                SlashCommands::Join.register(),
                SlashCommands::Leave.register(),
                SlashCommands::Ping.register(),
                SlashCommands::Clear.register(),
                SlashCommands::Invite.register(),
                SlashCommands::SelectChannel.register(),
                SlashCommands::Skip.register(),
                #[cfg(feature = "music")]
                SlashCommands::Play.register(),
                SlashCommands::Repeat.register(),
                SlashCommands::Queue.register(),
            ],
        )
        .await
        .unwrap();

        let cont = Ctx::new(ctx);
        set_help_message_to_activity(Box::new(cont)).await;
    }

    #[cfg(feature = "tts")]
    async fn message(&self, ctx: Context, msg: SerenityMessage) {
        let guild_id = msg.guild(&ctx.cache).unwrap().id;
        let voice = Voice::from(&ctx, guild_id).await;
        let tts_msg = crate::model::Message::new(msg);
        text_to_speech(Box::new(voice), tts_msg).await
    }

    async fn voice_state_update(
        &self,
        ctx: Context,
        old_voice_state: Option<voice::VoiceState>,
        new_voice_state: voice::VoiceState,
    ) {
        let state = CurrentVoiceState::new(new_voice_state);
        let change = state.change_of_states(old_voice_state.as_ref());
        let member = state.voice_member().await.expect("member is not received");
        let voice = Voice::from(&ctx, member.guild_id).await;
        let role = member.role(&ctx).await;
        if let Role::Me = role {
            if let ChangeOfStates::Leave = change {
                voice.remove().await.unwrap();
                println!("removed");
            };
            return println!("This is me(bot). My entering is ignored.");
        }
        #[cfg(feature = "tts")]
        speech_greeting(&ctx, &voice, &change, &member.user).await;
        leave_if_alone(&ctx, &voice).await;
    }
}

async fn leave_if_alone(ctx: &Context, voice: &Voice) {
    use crate::model::voice::Error;
    match voice.is_alone(ctx).await {
        Ok(true) => voice.remove().await.unwrap(),
        Ok(false) => (),
        Err(e) => match e {
            Error::ConnectionNotFound => (),
            Error::NotInVoiceChannel => (),
        },
    }
}
