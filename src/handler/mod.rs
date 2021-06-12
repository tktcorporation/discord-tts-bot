use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::{channel::Message, gateway::Ready},
};
use songbird::input::Input;
use std::sync::Arc;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }

    async fn message(&self, ctx: Context, msg: Message) {
        // botに反応しないようにする
        if msg.author.bot {
            return;
        }

        // voice channel にいない場合は動かさない
        if get_handler_when_in_voice_channel(&ctx, &msg)
            .await
            .is_none()
        {
            return;
        }

        // サーバーのID
        eprintln!("guild_id = {:?}", msg.guild_id);
        // チャンネル名
        let channel_name = msg.channel_id.name(&ctx.cache).await;
        eprintln!("channel_name = {:?}", channel_name);
        // メッセージの送信
        let content = msg.content;
        if let Err(why) = msg.channel_id.say(&ctx.http, content).await {
            println!("Error sending message: {:?}", why);
        }
    }
}

async fn get_handler_when_in_voice_channel(
    ctx: &Context,
    msg: &Message,
) -> Option<Arc<serenity::prelude::Mutex<songbird::Call>>> {
    let manager = songbird::get(&ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();
    return manager.get(msg.guild(&ctx.cache).await.unwrap().id);
}

async fn speak(handler_lock: &Arc<serenity::prelude::Mutex<songbird::Call>>, input: Input) {
    let mut handler = handler_lock.lock().await;
    // if let Some(handler_lock) = manager.get(guild_id) {
    //     let mut handler = handler_lock.lock().await;

    handler.enqueue_source(input);
}
