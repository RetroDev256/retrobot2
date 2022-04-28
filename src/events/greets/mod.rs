use serenity::{client::Context, model::guild::Member};
use std::error::Error;

pub async fn greet(ctx: Context, new_member: &Member) -> Result<(), Box<dyn Error>> {
    let name = new_member.display_name();
    let greet = format!("Welcome to the server {}!", name);
    new_member
        .user
        .direct_message(ctx.http, |msg| msg.content(greet))
        .await?;
    Ok(())
}
