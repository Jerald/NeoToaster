use std::env;

use serenity::{
    prelude::*,
    async_trait,
    framework::StandardFramework,
};

use tracing_subscriber::{
    FmtSubscriber,
    EnvFilter,
    filter::LevelFilter
};

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Unable to start tracing logger!");

    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a discord token in the environment at `DISCORD_TOKEN`");

    let framework = {
        let framework = StandardFramework::new()
            .configure(|c| c
                .prefix("nt>")
            );

        neotoaster_commands::GROUP_LIST.iter().copied().fold(
            framework,
            |frame, group| frame.group(group)
        )
    };

    let mut client = Client::builder(&token)
        .framework(framework)
        .event_handler(neotoaster::handler::Handler)
        .await
        .expect("Error creating client!");

    if let Err(err) = client.start().await {
        tracing::error!("Client error: {:?}", err);
    }
}