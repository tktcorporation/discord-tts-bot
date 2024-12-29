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

mod error;
mod model;
use error::{format_err, report_error};
use model::context::Context as Ctx;

use crate::commands::slash_commands::{SlashCommandResult, SlashCommands};
use model::{
    speaker::{ChangeOfStates, CurrentVoiceState, Role},
    voice::Voice,
};
pub mod usecase;
use usecase::set_help_message_to_activity::set_help_message_to_activity;
#[cfg(test)]
use usecase::text_to_speech::SpeechMessage;

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

            match command_result {
                SlashCommandResult::Simple(None) => {
                    command.delete_response(&ctx.http).await.unwrap();
                }
                SlashCommandResult::Simple(Some(message)) => {
                    command
                        .edit_response(
                            &ctx.http,
                            EditInteractionResponse::default().content(message),
                        )
                        .await
                        .unwrap();
                }
                SlashCommandResult::Embed(embed) => {
                    command
                        .edit_response(&ctx.http, EditInteractionResponse::default().embed(*embed))
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
        let state = CurrentVoiceState::new(new_voice_state.clone());
        let change = state.change_of_states(old_voice_state.as_ref(), &ctx);
        let member = match state.voice_member().await {
            Ok(m) => m,
            Err(e) => {
                report_error(&format_err("Failed to get voice member", e));
                return;
            }
        };
        let guild_id = member.guild_id;
        let voice = Voice::from(&ctx, guild_id).await;
        let role = match member.role(&ctx).await {
            Ok(r) => r,
            Err(e) => {
                report_error(&format_err("Failed to get member role", e));
                return;
            }
        };

        if let Role::Me = role {
            if let ChangeOfStates::Leave = change {
                if let Err(e) = voice.remove().await {
                    report_error(&format_err("Failed to remove voice", e));
                }
                println!("removed");
            };
            return println!("This is me(bot). My entering is ignored.");
        }

        println!("old_voice_state: {:?}", old_voice_state);
        println!("new_voice_state: {:?}", new_voice_state);

        #[cfg(feature = "tts")]
        if let Err(e) = speech_greeting(&ctx, &voice, &change, &member.user).await {
            report_error(&format_err("Failed to speech greeting", e));
        }
        if let Err(e) = leave_if_alone(&ctx, &voice).await {
            report_error(&format_err("Failed to check/handle leave if alone", e));
        }
    }
}

async fn leave_if_alone(ctx: &Context, voice: &Voice) -> Result<(), String> {
    use crate::model::voice::Error;
    match voice.is_alone(ctx).await {
        Ok(true) => {
            voice
                .remove()
                .await
                .map_err(|e| format!("Failed to remove voice: {:?}", e))?;
            Ok(())
        }
        Ok(false) => Ok(()),
        Err(e) => match e {
            Error::ConnectionNotFound => Ok(()),
            Error::NotInVoiceChannel => Ok(()),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::mock;
    use mockall::predicate::*;

    mock! {
        pub Voice {
            async fn speech(&self, msg: SpeechMessage) -> Result<(), String>;
        }
    }

    #[test]
    fn test_afk_detection() {
        // 通常状態からAFKに変更されるケース
        let old_deaf = false;
        let new_deaf = true;
        assert!(
            !old_deaf && new_deaf,
            "通常状態からAFKへの変更を検知できません"
        );

        // AFKから通常状態に戻るケース
        let old_deaf = true;
        let new_deaf = false;
        assert!(
            old_deaf && !new_deaf,
            "AFKから通常状態への変更を検知できません"
        );

        // 最初からAFKの状態のケース
        let old_deaf = true;
        let new_deaf = true;
        assert!(old_deaf || !new_deaf, "既にAFKの状態で誤検知しています");
    }

    #[tokio::test]
    async fn test_speech_messages() {
        let mut mock_voice = MockVoice::new();

        // おやすみなさいのテスト
        mock_voice
            .expect_speech()
            .with(eq(SpeechMessage {
                value: "おやすみなさい".to_string(),
            }))
            .times(1)
            .return_once(|_| Ok(()));

        // おはようございますのテスト
        mock_voice
            .expect_speech()
            .with(eq(SpeechMessage {
                value: "おはようございます".to_string(),
            }))
            .times(1)
            .return_once(|_| Ok(()));

        // メッセージの内容を確認
        let goodnight_msg = SpeechMessage {
            value: "おやすみなさい".to_string(),
        };
        mock_voice.speech(goodnight_msg).await.unwrap();

        let morning_msg = SpeechMessage {
            value: "おはようございます".to_string(),
        };
        mock_voice.speech(morning_msg).await.unwrap();
    }
}
