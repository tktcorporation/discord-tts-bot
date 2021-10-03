use std::{collections::HashSet, env};

use super::commands::GENERAL_GROUP;
use serenity::{
    framework::standard::{
        help_commands, macros::help, Args, CommandGroup, CommandResult, HelpOptions,
        StandardFramework,
    },
    model::prelude::*,
    prelude::*,
};

#[help]
async fn my_help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let _ = help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
    Ok(())
}

pub fn build_framework() -> StandardFramework {
    StandardFramework::new()
        .configure(|c| {
            c.prefix(
                &env::var("DISCORD_CMD_PREFIX")
                    .expect("Expected a command prefix in the environment"),
            )
        })
        .help(&MY_HELP)
        .group(&GENERAL_GROUP)
}