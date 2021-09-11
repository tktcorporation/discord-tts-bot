use serenity::{async_trait, client::Context as SerenityContext, model::gateway::Activity};

use super::super::usecase::ActivityController;

pub struct Context {
    ctx: SerenityContext,
}

#[cfg_attr(feature = "mock", mockall::automock)]
#[async_trait]
impl ActivityController for Context {
    async fn set_activity(&self, activity: Activity) {
        self.ctx.set_activity(activity).await
    }
}

impl Context {
    pub fn new(ctx: SerenityContext) -> Context {
        Context { ctx }
    }
}
