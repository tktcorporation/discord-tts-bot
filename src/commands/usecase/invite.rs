use serenity::{client::Context, model::permissions::Permissions};

pub async fn invite(ctx: &Context) -> Result<std::string::String, serenity::Error> {
    let user = ctx.cache.current_user();
    user.invite_url(&ctx.http, Permissions::from_bits(2184261184).unwrap())
        .await
}
