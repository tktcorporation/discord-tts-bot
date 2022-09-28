use serenity::{client::Context, model::permissions::Permissions};

use super::error::Error;

pub async fn invite(ctx: &Context) -> Result<String, Error> {
    let user = ctx.cache.current_user();
    match user
        .invite_url(&ctx.http, Permissions::from_bits(2184261184).unwrap())
        .await
    {
        Ok(url) => Ok(url),
        Err(e) => Err(Error::SerenityError(e)),
    }
}
