use serenity::client::Context;
use serenity::gateway::ActivityData;
use serenity::model::user::OnlineStatus;

pub async fn set_help_message_to_activity(ctx: &Context, message: &str) {
    let activity = ActivityData::playing(message);
    ctx.shard.set_presence(Some(activity), OnlineStatus::Online);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_set_activity() {
        // TODO: Implement proper test when we have a way to mock Context
        // For now, we just verify that ActivityData::playing creates the correct activity
        let message = "test message";
        let activity = ActivityData::playing(message);
        assert_eq!(activity.name, message);
    }
}
