use serenity::{
    prelude::*,
    model::prelude::*,
    async_trait
};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        tracing::info!("Connected as {}", ready.user.name)
    }

    async fn resume(&self, _ctx: Context, _: ResumedEvent) {
        tracing::info!("Resumed!");
    }
}