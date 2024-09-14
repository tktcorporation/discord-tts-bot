use serenity::gateway::ActivityData;
use serenity::{async_trait, client::Context as SerenityContext};

use super::super::usecase::set_help_message_to_activity::ActivityController;

pub struct Context {
    ctx: SerenityContext,
}

#[async_trait]
impl ActivityController for Context {
    async fn set_activity(&self, activity: ActivityData) {
        self.ctx.set_activity(Some(activity))
    }
}

impl Context {
    pub fn new(ctx: SerenityContext) -> Context {
        Context { ctx }
    }
}
