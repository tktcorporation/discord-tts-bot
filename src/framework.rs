use std::env;

use crate::commands::message_commands::COMMANDS_GROUP;
use serenity::framework::standard::StandardFramework;

pub fn build_framework() -> StandardFramework {
    StandardFramework::new()
        .configure(|c| {
            c.prefix(&env::var("DISCORD_CMD_PREFIX").unwrap_or_else(|_| "-".to_string()))
        })
        .group(&COMMANDS_GROUP)
}
