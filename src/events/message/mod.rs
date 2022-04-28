use std::error::Error;

use serenity::{client::Context, model::channel::Message};

pub async fn message_event(_ctx: Context, _msg: Message) -> Result<(), Box<dyn Error>> {
    // Currently do nothing
    Ok(())
}
