use std::error::Error;

use regex::RegexBuilder;
use serenity::{client::Context, model::channel::Message};

use crate::custom_cmds::CUST_CMDS;

pub async fn sift_cust_cmd(ctx: Context, msg: Message) -> Result<(), Box<dyn Error>> {
    let server_cmds_opt = match msg.guild_id {
        Some(server_id) => match CUST_CMDS.read() {
            Ok(lock) => lock.get(server_id.as_u64()).cloned(),
            _ => None,
        },
        _ => None,
    };
    if let Some(server_cmds) = server_cmds_opt {
        for (regex_str, resp) in server_cmds {
            if let Ok(regex) = RegexBuilder::new(&regex_str).build() {
                if regex.is_match(&msg.content) {
                    msg.reply(&ctx.http, resp).await?;
                }
            }
        }
    }
    Ok(())
}
