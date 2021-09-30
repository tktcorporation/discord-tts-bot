use super::check_msg;
use serenity::{
    client::Context,
    model::{
        channel::Message as SerenityMessage, id, id::ChannelId as SerenityChannelId,
        misc::Mentionable,
    },
};
use songbird::{self, ffmpeg};
use std::path::Path;

pub use crate::model::{Message, Voice};

impl Voice {
    async fn join(
        &self,
        connect_to: id::ChannelId,
    ) -> (
        std::sync::Arc<tokio::sync::Mutex<songbird::Call>>,
        songbird::error::JoinResult<()>,
    ) {
        self.manager.join(self.guild_id, connect_to).await
    }
}

pub async fn join(ctx: &Context, msg: &SerenityMessage, joiner: Voice) -> Result<(), String> {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    let channel_id = guild
        .voice_states
        .get(&msg.author.id)
        .and_then(|voice_state| voice_state.channel_id);

    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            check_msg(msg.reply(ctx, "Not in a voice channel").await);

            return Ok(());
        }
    };

    let comment = match _join(&joiner, connect_to).await {
        Ok(()) => format!("Joined {}", connect_to.mention()),
        Err(e) => e,
    };

    check_msg(msg.channel_id.say(&ctx.http, &comment).await);
    Ok(())
}

async fn _join(joiner: &Voice, connect_to: SerenityChannelId) -> Result<(), String> {
    let (handle_lock, success) = joiner.join(connect_to).await;

    if let Ok(_channel) = success {
        let mut handle = handle_lock.lock().await;

        let root = env!("CARGO_MANIFEST_DIR");
        let path = Path::new(root);
        let file_path = path.join("sounds").join("shabeko_dayo.wav");
        let input = ffmpeg(file_path)
            .await
            .expect("This might fail: handle this error!");
        handle.enqueue_source(input);
        Ok(())
    } else {
        Err("Error joining the channel".to_string())
    }
}

// #[async_trait]
// pub trait Joiner {
//     async fn join(
//         &self,
//         connect_to: id::ChannelId,
//     ) -> (
//         std::sync::Arc<tokio::sync::Mutex<songbird::Call>>,
//         songbird::error::JoinResult<()>,
//     );
//     async fn piin(
//         &self,
//     ) -> (String,
//         String
//     );
// }
