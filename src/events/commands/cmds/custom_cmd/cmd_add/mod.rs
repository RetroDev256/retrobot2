use std::error::Error;

use serenity::{
    client::Context,
    model::{
        id::GuildId,
        interactions::application_command::{
            ApplicationCommandInteraction, ApplicationCommandInteractionDataOptionValue,
        },
    },
};

use crate::custom_cmds::add_command;

pub mod setup;

pub async fn cmd_add(
    int: ApplicationCommandInteraction,
    ctx: Context,
) -> Result<(), Box<dyn Error>> {
    let regex_option = match int.data.options.get(0) {
        Some(regex_arg) => match regex_arg.resolved.as_ref() {
            Some(ApplicationCommandInteractionDataOptionValue::String(regex_str)) => {
                Some(regex_str.clone())
            }
            _ => None,
        },
        _ => None,
    };
    let reply_option = match int.data.options.get(1) {
        Some(reply_arg) => match reply_arg.resolved.as_ref() {
            Some(ApplicationCommandInteractionDataOptionValue::String(reply_str)) => {
                Some(reply_str.clone())
            }
            _ => None,
        },
        _ => None,
    };
    let response = try_add_cmd(regex_option, reply_option, int.guild_id);
    int.create_interaction_response(ctx.http, |resp| {
        resp.interaction_response_data(|data| data.ephemeral(true).content(response))
    })
    .await?;
    Ok(())
}

fn try_add_cmd<'a>(
    regex_option: Option<String>,
    reply_option: Option<String>,
    guild_id: Option<GuildId>,
) -> String {
    match (regex_option, reply_option) {
        (Some(regex_str), Some(reply)) => match guild_id {
            Some(server_id) => add_command(*server_id.as_u64(), regex_str, reply),
            _ => "This is not a server".to_owned(),
        },
        (Some(_regex_str), None) => "Response was not provided.".to_owned(),
        (None, Some(_reply)) => "Regex was not provided.".to_owned(),
        _ => "Neither regex nor response was provided.".to_owned(),
    }
}
