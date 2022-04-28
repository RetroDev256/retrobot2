mod cust_cmd_resp;

use std::error::Error;

use serenity::{client::Context, model::channel::Message};

use self::cust_cmd_resp::sift_cust_cmd;

pub async fn message_event(ctx: Context, msg: Message) -> Result<(), Box<dyn Error>> {
    let is_bot = msg.author.bot;
    if !is_bot {
        sift_cust_cmd(ctx, msg).await?;
    }
    Ok(())
}
