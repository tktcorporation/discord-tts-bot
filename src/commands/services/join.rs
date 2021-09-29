use super::check_msg;
use serenity::{
    client::Context,
    model::{channel::Message, id, id::ChannelId as SerenityChannelId, misc::Mentionable},
};
use songbird::{self, ffmpeg};
use std::path::Path;

pub use crate::model::Voice;

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

pub async fn join(ctx: &Context, msg: &Message, joiner: Voice) -> Result<(), String> {
    let (_, channel_id) = match joiner.guild_id_and_channel_id().await {
        Ok(ids) => ids,
        Err(e) => {
            check_msg(msg.reply(ctx, e).await);

            return Ok(());
        }
    };

    let connect_to = match channel_id {
        Some(channel) => SerenityChannelId(channel.0),
        None => {
            check_msg(msg.reply(ctx, "Not in a voice channel").await);

            return Ok(());
        }
    };

    let (handle_lock, success) = joiner.join(connect_to).await;

    if let Ok(_channel) = success {
        check_msg(
            msg.channel_id
                .say(&ctx.http, &format!("Joined {}", connect_to.mention()))
                .await,
        );

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
        check_msg(
            msg.channel_id
                .say(&ctx.http, "Error joining the channel")
                .await,
        );
        Ok(())
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
