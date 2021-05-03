use serenity::{framework::standard::Command, prelude::*};
use serenity::model::channel::Message;

use serenity::framework::standard::{
    CommandResult,
    macros::{
        command,
        group
    }
};

#[group]
#[commands(test, ping, hello, frack_you, youmustconstructadditionalpylons)]
struct General;

#[command]
async fn test(ctx: &Context, msg: &Message) -> CommandResult
{
    msg.channel_id.say(ctx, "I work right! And this was changed at runtime!").await?;
    Ok(())
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult
{
    msg.channel_id.say(ctx, "Pong! Was I dead?").await?;
    Ok(())
}

#[command]
async fn hello(ctx: &Context, msg: &Message) -> CommandResult
{
    msg.channel_id.say(ctx, "Hi! This is NeoToaster!").await?;
    Ok(())
}

#[command]
async fn youmustconstructadditionalpylons(ctx: &Context, msg: &Message) -> CommandResult
{
    msg.channel_id.say(ctx, "StolenLight asked for this...").await?;
    Ok(())
}

#[command]
async fn frack_you(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(ctx, "No, frack _you_").await?;
    Ok(())
}