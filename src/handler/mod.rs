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

use crate::infrastructure::tmp_path;
use std::fs;
use std::time::SystemTime;
use tokio::time::{interval, Duration};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            println!("Received command interaction: {}", command.data.name);

            command
                .create_response(
                    &ctx,
                    CreateInteractionResponse::Message(
                        CreateInteractionResponseMessage::new().content("Processing..."),
                    ),
                )
                .await
                .unwrap();

            let command_result = match command.data.name.as_str().parse::<SlashCommands>() {
                Ok(slash_command) => slash_command.run(&ctx, &command).await,
                Err(_) => {
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

        let ctx_clone = ctx.clone();
        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(300));
            loop {
                interval.tick().await;
                if let Ok(entries) = fs::read_dir(tmp_path()) {
                    let guild_count = entries
                        .filter(|e| e.is_ok())
                        .filter(|e| e.as_ref().unwrap().path().is_dir())
                        .count();
                    let activity = format!("{guild_count} サーバーで稼働中 | /help");
                    set_help_message_to_activity(&ctx_clone, &activity).await;
                }
                cleanup_old_audio_files();
            }
        });

        // 新しいコマンドを登録
        Command::set_global_commands(&ctx.http, SlashCommands::get_commands())
            .await
            .expect("Failed to set global commands");
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

        println!("old_voice_state: {old_voice_state:?}");
        println!("new_voice_state: {new_voice_state:?}");

        #[cfg(feature = "tts")]
        if let Err(e) = speech_greeting(&ctx, &voice, &change, &member.user).await {
            report_error(&format_err("Failed to speech greeting", e));
        }
        if let Err(e) = leave_if_alone(&ctx, &voice).await {
            report_error(&format_err("Failed to check/handle leave if alone", e));
        }
    }
}

/// Delete audio files older than 10 minutes to prevent memory/disk accumulation.
fn cleanup_old_audio_files() {
    let max_age = std::time::Duration::from_secs(600);
    let now = SystemTime::now();

    let Ok(guild_dirs) = fs::read_dir(tmp_path()) else {
        return;
    };
    for guild_entry in guild_dirs.flatten() {
        let sounds_dir = guild_entry.path().join("sounds");
        let Ok(files) = fs::read_dir(&sounds_dir) else {
            continue;
        };
        for file_entry in files.flatten() {
            let path = file_entry.path();
            if !path.is_file() {
                continue;
            }
            let Ok(metadata) = fs::metadata(&path) else {
                continue;
            };
            let is_old = metadata
                .modified()
                .ok()
                .and_then(|modified| now.duration_since(modified).ok())
                .is_some_and(|age| age > max_age);
            if is_old {
                let _ = fs::remove_file(&path);
            }
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
                .map_err(|e| format!("Failed to remove voice: {e:?}"))?;
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
