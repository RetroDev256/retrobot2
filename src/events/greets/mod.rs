use serenity::{client::Context, model::guild::Member};
use std::error::Error;

use crate::tools::filter_pings;

pub async fn greet(ctx: Context, new_member: &Member) -> Result<(), Box<dyn Error>> {
    let name = new_member.display_name();
    let greet = format!("Welcome to the server {}! o/", name);
    new_member
        .user
        .direct_message(ctx.http, |msg| msg.content(filter_pings(&greet)))
        .await?;
    Ok(())
}
