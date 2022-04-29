use std::error::Error;

use serenity::{client::Context, model::channel::Message};

use crate::{custom_cmds::get_commands, tools::filter_pings};

pub async fn sift_cust_cmd(ctx: Context, msg: Message) -> Result<(), Box<dyn Error>> {
    let server_cmds_opt = msg
        .guild_id
        .map(|server_id| get_commands(server_id.as_u64()));
    if let Some(server_cmds) = server_cmds_opt {
        for (regex, _, resp) in server_cmds {
            if regex.is_match(&msg.content) {
                msg.reply(&ctx.http, filter_pings(&resp)).await?;
            }
        }
    }
    Ok(())
}
