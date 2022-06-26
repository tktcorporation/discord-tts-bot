#[cfg(feature = "tts")]
use serenity::model::{channel::Message as SerenityMessage, voice};
use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::gateway::Ready,
};

mod model;
use model::context::Context as Ctx;

#[cfg(feature = "tts")]
use model::{speaker::CurrentVoiceState, voice::Voice};
pub mod usecase;
use usecase::set_help_message_to_activity::set_help_message_to_activity;

#[cfg(feature = "tts")]
use usecase::{speech_welcome_see_you::change_check, text_to_speech::text_to_speech};

pub struct Handler;

#[async_trait]
#[cfg_attr(feature = "mock", mockall::automock)]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        let cont = Ctx::new(ctx);
        set_help_message_to_activity(Box::new(cont)).await
    }

    #[cfg(feature = "tts")]
    async fn message(&self, ctx: Context, msg: SerenityMessage) {
        let guild_id = msg.guild(&ctx.cache).await.unwrap().id;
        let voice = Voice::from(&ctx, guild_id).await;
        let tts_msg = crate::model::Message::new(msg);
        text_to_speech(Box::new(voice), tts_msg).await
    }

    #[cfg(feature = "tts")]
    async fn voice_state_update(
        &self,
        ctx: Context,
        _: Option<serenity::model::id::GuildId>,
        old_voice_state: Option<voice::VoiceState>,
        new_voice_state: voice::VoiceState,
    ) {
        let state = CurrentVoiceState::new(new_voice_state);
        change_check(&ctx, state, old_voice_state).await;
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
