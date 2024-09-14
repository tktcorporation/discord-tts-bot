use std::env;

use serenity::gateway::ActivityData;
use serenity::{async_trait, model::gateway::Activity};

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait ActivityController {
    async fn set_activity(&self, activity: ActivityData);
}

pub async fn set_help_message_to_activity(ctx: Box<dyn ActivityController + Send + Sync>) {
    ctx.set_activity(ActivityData::playing(
        env::var("DISCORD_CMD_PREFIX").expect("Expected a command prefix in the environment")
            + "join で呼んでね",
    ))
    .await;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_set_activity() {
        let mut controller = MockActivityController::new();
        controller.expect_set_activity().times(1).return_const(());

        set_help_message_to_activity(Box::new(controller)).await
    }
}
