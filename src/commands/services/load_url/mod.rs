
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_url() {
        let source = match input::ytdl(&url).await {
            Ok(source) => source,
            Err(why) => {
                println!("Err starting source: {:?}", why);
        
                check_msg(msg.channel_id.say(&ctx.http, "Error sourcing ffmpeg").await);
        
                return Ok(());
            }
        };
    }
}