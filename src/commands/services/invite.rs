use serenity::{client::Context, model::permissions::Permissions};

pub async fn invite(ctx: &Context) -> Result<std::string::String, serenity::Error> {
    let user = ctx.cache.current_user().await;
    user.invite_url(&ctx.http, Permissions { bits: 2184261184 })
        .await
}
