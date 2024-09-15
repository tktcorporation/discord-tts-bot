use serenity::{client::Context, model::permissions::Permissions};

use super::error::Error;

pub async fn invite(ctx: &Context) -> Result<String, Error> {
    // Fetch the current application information to get the application (client) ID
    let app_info = ctx
        .http
        .get_current_application_info()
        .await
        .map_err(Error::SerenityError)?;

    // Convert your permissions to a bit set
    let permissions = Permissions::from_bits(2184261184).unwrap().bits();

    // Construct the invite URL manually
    let invite_url = format!(
        "https://discord.com/oauth2/authorize?client_id={}&scope=bot&permissions={}",
        app_info.id, permissions
    );

    Ok(invite_url)
}
