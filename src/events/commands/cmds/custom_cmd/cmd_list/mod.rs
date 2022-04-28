use std::error::Error;

use serenity::{
    client::Context, model::interactions::application_command::ApplicationCommandInteraction,
    utils::MessageBuilder,
};

use crate::custom_cmds::CUST_CMDS;

pub mod setup;

pub async fn cmd_list(
    int: ApplicationCommandInteraction,
    ctx: Context,
) -> Result<(), Box<dyn Error>> {
    int.create_interaction_response(&ctx.http, |resp| {
        resp.interaction_response_data(|data| data.content("Listing commands for the server:"))
    })
    .await?;
    let mut cmds_to_list = vec![];
    let response_opt = match CUST_CMDS.read() {
        Ok(cmds) => match int.guild_id {
            Some(guild_id) => match cmds.get(guild_id.as_u64()) {
                Some(cmd_list) => match cmd_list.is_empty() {
                    true => Some("There are no commands for this server."),
                    _ => {
                        for (i, cmd) in cmd_list.iter().enumerate() {
                            let mut message = MessageBuilder::new();
                            let fmt_cmd = message
                                .push_bold("Command ")
                                .push(i)
                                .push_bold_line(':')
                                .push_bold_line("Regex key:")
                                .push_line(&cmd.regex)
                                .push_bold_line("Response:")
                                .push(&cmd.response)
                                .build();
                            let msg_content = match fmt_cmd.len() >= 2000 {
                                true => format!("{}...", &fmt_cmd[0..1997]),
                                _ => fmt_cmd,
                            };
                            cmds_to_list.push(msg_content);
                        }
                        None
                    }
                },
                _ => Some("There is not a list of commands for this server."),
            },
            _ => Some("This is not a server."),
        },
        _ => Some("Unable to get a read lock on the command list."),
    };
    for cmd_txt in cmds_to_list {
        int.create_followup_message(&ctx.http, |followup| followup.content(cmd_txt))
            .await?;
    }
    if let Some(response) = response_opt {
        int.create_followup_message(&ctx.http, |followup| followup.content(response))
            .await?;
    }
    Ok(())
}
