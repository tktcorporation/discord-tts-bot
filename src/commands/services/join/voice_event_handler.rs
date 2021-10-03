use serenity::{
    async_trait,
};
use songbird::{
    Event, EventContext, EventHandler as VoiceEventHandler,
};

pub struct Receiver;

impl Receiver {
    pub fn new() -> Self {
        // You can manage state here, such as a buffer of audio packet bytes so
        // you can later store them in intervals.
        Self {}
    }
}

#[async_trait]
impl VoiceEventHandler for Receiver {
    #[allow(unused_variables)]
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        println!("VoiceEventHandler");
        use EventContext as Ctx;
        match ctx {
            Ctx::SpeakingUpdate(data) => {
                // You can implement logic here which reacts to a user starting
                // or stopping speaking.
                println!(
                    "Source {} has {} speaking.",
                    data.ssrc,
                    if data.speaking { "started" } else { "stopped" },
                );
            }
            _ => {
                // We won't be registering this struct for any more event classes.
                unimplemented!()
            }
        }

        None
    }
}
