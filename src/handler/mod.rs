use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::{channel::Message, gateway::Ready},
};
use songbird::input::{
    restartable::Restartable,
    error::{Error, Result},
    Codec, Container, Input, Metadata,
};
use songbird::{driver::Driver, ffmpeg, tracks::create_player, tracks::Track};
use std::sync::Arc;
use std::{
    io::{BufRead, BufReader, Read},
    process::{Command, Stdio},
};
use tokio::{process::Command as TokioCommand, task};

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
        let content = msg.content.clone();
        if let Err(why) = msg.channel_id.say(&ctx.http, content).await {
            println!("Error sending message: {:?}", why);
        }

        let handler_lock = get_handler_when_in_voice_channel(&ctx, &msg).await.unwrap();

        let ma = msg.content.clone();
        match ma.as_str() {
            "BGM" => {
                let input = get_bgm_input().await.unwrap();
                play_input(&handler_lock, input).await;
            },
            _ => {},
        };

        let mut handler = handler_lock.lock().await;
        let track = get_track_from_local(0.7).await;
        // handler.play_only(track);
        handler.play(track);
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

async fn play_input(handler_lock: &std::sync::Arc<serenity::prelude::Mutex<songbird::Call>>, input: Input) {
    let mut handler = handler_lock.lock().await;
    // if let Some(handler_lock) = manager.get(guild_id) {
    //     let mut handler = handler_lock.lock().await;

    handler.enqueue_source(input);
}

async fn get_bgm_input() -> Result<Input> {
    let url = "https://youtu.be/16Bj6aPi1A8";
    let source = match Restartable::ytdl(url, true).await {
        Ok(source) => return Ok(source.into()),
        Err(why) => {
            println!("Err get input source: {:?}", why);

            return Err(why);
        }
    };
}

async fn play_track(handler_lock: &Arc<serenity::prelude::Mutex<songbird::Call>>, track: Track) {
    let mut handler = handler_lock.lock().await;
    // if let Some(handler_lock) = manager.get(guild_id) {
    //     let mut handler = handler_lock.lock().await;

    handler.play_only(track)
}

async fn get_track_from_local(volume: f32) -> Track {
    let source = ffmpeg("./2_23_AM_2.mp3")
        .await
        .expect("This might fail: handle this error!");

    let (mut audio, _) = create_player(source);
    audio.set_volume(volume);

    audio
}
