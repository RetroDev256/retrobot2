use std::error::Error;

use serenity::{client::Context, model::channel::Message};

use crate::custom_cmds::get_commands;

pub async fn sift_cust_cmd(ctx: Context, msg: Message) -> Result<(), Box<dyn Error>> {
    let server_cmds_opt = match msg.guild_id {
        Some(server_id) => Some(get_commands(*server_id.as_u64())),
        _ => None,
    };
    if let Some(server_cmds) = server_cmds_opt {
        for (regex, _, resp) in server_cmds {
            if regex.is_match(&msg.content) {
                msg.reply(&ctx.http, resp).await?;
            }
        }
    }
    Ok(())
}
