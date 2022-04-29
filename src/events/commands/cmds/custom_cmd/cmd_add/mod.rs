use std::{collections::HashMap, error::Error};

use regex::RegexBuilder;
use serenity::{
    client::Context,
    model::{
        id::GuildId,
        interactions::application_command::{
            ApplicationCommandInteraction, ApplicationCommandInteractionDataOptionValue,
        },
    },
};

use crate::custom_cmds::CUST_CMDS;

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
        (Some(regex_str), Some(reply)) => {
            match RegexBuilder::new(&regex_str).size_limit(1048576).build() {
                Ok(_regex) => match CUST_CMDS.write() {
                    Ok(mut lock) => match guild_id {
                        Some(server_id) => match lock.get_mut(&server_id.0) {
                            Some(cmds) => match cmds.insert(regex_str, reply) {
                                None => "Successfully added new command.".to_owned(),
                                Some(_) => "Overwrote old command with the same key.".to_owned(),
                            },
                            _ => {
                                let _ =
                                    lock.insert(server_id.0, HashMap::from([(regex_str, reply)]));
                                "Successfully added first command.".to_owned()
                            }
                        },
                        _ => "This is not a server.".to_owned(),
                    },
                    _ => "Unable to obtain a write lock on server commands list.".to_owned(),
                },
                Err(regex_err) => match regex_err {
                    regex::Error::Syntax(syntax) => match syntax.len() > 1972 {
                        true => format!("Invalid regex syntax: ```{}...```", &syntax[..1969]),
                        false => format!("Invalid regex syntax: ```{}```", syntax),
                    },
                    regex::Error::CompiledTooBig(too_big) => format!(
                        "Compiled regex exceeded allowed 1 MiB: {} bytes used.",
                        too_big
                    ),
                    _ => "Failed to compile regex.".to_owned(),
                },
            }
        }
        (Some(_regex_str), None) => "Response was not provided.".to_owned(),
        (None, Some(_reply)) => "Regex was not provided.".to_owned(),
        _ => "Neither regex nor response was provided.".to_owned(),
    }
}
