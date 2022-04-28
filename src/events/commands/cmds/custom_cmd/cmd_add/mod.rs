use std::{collections::HashSet, error::Error};

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

use crate::custom_cmds::{Command, CUST_CMDS};

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
    let reply_option = match int.data.options.get(0) {
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
        resp.interaction_response_data(|data| data.content(response))
    })
    .await?;
    Ok(())
}

fn try_add_cmd<'a>(
    regex_option: Option<String>,
    reply_option: Option<String>,
    guild_id: Option<GuildId>,
) -> &'a str {
    match (regex_option, reply_option) {
        (Some(regex_str), Some(reply)) => match RegexBuilder::new(&regex_str).build() {
            Ok(_regex) => match CUST_CMDS.write() {
                Ok(mut lock) => match guild_id {
                    Some(server_id) => {
                        let command = Command::new(regex_str, reply);
                        match lock.get_mut(&server_id.0) {
                            Some(cmds) => match cmds.insert(command) {
                                true => "Successfully added new command.",
                                false => "This command has already been added.",
                            },
                            _ => {
                                let _ = lock.insert(server_id.0, HashSet::from([command]));
                                "Successfully added first command."
                            }
                        }
                    }
                    _ => "This is not a server.",
                },
                _ => "Unable to obtain a write lock on server commands list.",
            },
            _ => "Invalid regex.",
        },
        (Some(_regex_str), None) => "Response was not provided.",
        (None, Some(_reply)) => "Regex was not provided.",
        _ => "Neither regex nor response was provided.",
    }
}
