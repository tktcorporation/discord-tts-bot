use serenity::{self, client::Context};

pub async fn leave(
    ctx: &Context,
    guild_id: serenity::model::id::GuildId,
) -> Result<String, String> {
    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();
    let has_handler = manager.get(guild_id).is_some();

    if has_handler {
        if let Err(e) = manager.remove(guild_id).await {
            return Err(format!("Failed: {:?}", e));
        }
        return Ok("ばいばい".to_string());
    }
    Ok("Not in a voice channel".to_string())
}
