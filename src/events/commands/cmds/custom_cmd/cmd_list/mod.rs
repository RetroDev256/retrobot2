use serenity::{
    client::Context, model::interactions::application_command::ApplicationCommandInteraction,
};

use crate::custom_cmds::get_commands;

pub mod setup;

pub async fn cmd_list(int: ApplicationCommandInteraction, ctx: Context) {
    int.create_interaction_response(&ctx.http, |resp| {
        resp.interaction_response_data(|data| {
            data.ephemeral(true)
                .content("Listing commands for the server:")
        })
    })
    .await
    .unwrap();
    let resp_opt = match int.guild_id {
        Some(guild_id) => {
            let server_cmds = get_commands(guild_id.as_u64());
            match server_cmds.is_empty() {
                true => Some("Server has no commands"),
                _ => {
                    for (i, (_, reg_str, reply)) in server_cmds.into_iter().enumerate() {
                        let fmt_cmd = format!(
                            "**Command {} Key:**\n```{}```**Response:**\n```{}```",
                            i, reg_str, reply
                        );
                        let msg_content = match fmt_cmd.len() > 2000 {
                            true => format!("{}...", &fmt_cmd[0..1997]),
                            _ => fmt_cmd,
                        };
                        int.create_followup_message(&ctx.http, |followup| {
                            followup.ephemeral(true).content(msg_content)
                        })
                        .await
                        .unwrap();
                    }
                    None
                }
            }
        }
        _ => Some("This is not a server."),
    };
    if let Some(resp) = resp_opt {
        int.create_followup_message(&ctx.http, |followup| followup.ephemeral(true).content(resp))
            .await
            .unwrap();
    }
}
