use serenity::model::application::command::Command;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
#[cfg(feature = "tts")]
use serenity::model::channel::Message as SerenityMessage;
use serenity::model::voice;
use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::gateway::Ready,
};

mod model;
use model::context::Context as Ctx;

use crate::commands::slash_commands;
use model::{
    speaker::{CurrentVoiceState, Role},
    voice::Voice,
};
pub mod usecase;
use usecase::set_help_message_to_activity::set_help_message_to_activity;

#[cfg(feature = "tts")]
use usecase::{speech_welcome_see_you::speech_greeting, text_to_speech::text_to_speech};

pub struct Handler;

#[async_trait]
#[cfg_attr(feature = "mock", mockall::automock)]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            println!("Received command interaction: {:#?}", command);

            let content = match command.data.name.as_str() {
                "join" => slash_commands::join::run(&ctx, &command).await,
                _ => "not implemented :(".to_string(),
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        Command::set_global_application_commands(&ctx.http, |commands| {
            commands.create_application_command(|command| slash_commands::join::register(command))
        })
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
        #[cfg(feature = "tts")]
        let change = state.change_of_states(old_voice_state.as_ref());
        let member = state.voice_member().await.expect("member is not received");
        let voice = Voice::from(&ctx, member.guild_id).await;
        let role = member.role(&ctx).await;
        if let Role::Me = role {
            return println!("This is me(bot). My entering is ignored.");
        }
        #[cfg(feature = "tts")]
        speech_greeting(&ctx, &voice, &change, &member.user).await;
        leave_if_alone(&ctx, &voice).await;
    }
}

async fn leave_if_alone(ctx: &Context, voice: &Voice) {
    if voice.is_alone(ctx).await.unwrap() {
        voice.leave().await.unwrap()
    }
}

// async fn debug_print(msg: &SerenityMessage, ctx: &Context) {
//     // サーバーのID
//     eprintln!("guild_id = {:?}", msg.guild_id);
//     // チャンネル名
//     let channel_name = msg.channel_id.name(&ctx.cache).await;
//     eprintln!("channel_name = {:?}", channel_name);
//     // メッセージの送信
//     let content = msg.content.clone();
//     eprintln!("message received: {:?}", content);
// }
