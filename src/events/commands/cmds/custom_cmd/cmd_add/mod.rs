use std::error::Error;

use regex::RegexBuilder;
use serenity::{
    client::Context,
    model::interactions::application_command::{
        ApplicationCommandInteraction, ApplicationCommandInteractionDataOptionValue,
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
    let response = match (regex_option, reply_option) {
        (Some(regex_str), Some(reply)) => match RegexBuilder::new(&regex_str).build() {
            Ok(_regex) => match CUST_CMDS.write() {
                Ok(mut lock) => match int.guild_id {
                    Some(server_id) => match lock.get_mut(&server_id.0) {
                        Some(cmds) => {
                            cmds.push(Command::new(regex_str, reply));
                            "Sucessfully added command."
                        }
                        _ => match lock.insert(server_id.0, vec![Command::new(regex_str, reply)]) {
                            Some(_) => "Successfully added first command.",
                            _ => "Failed to add first command.",
                        },
                    },
                    _ => "This is not a server.",
                },
                _ => "Unable to obtain a write lock on server commands list.",
            },
            _ => "Invalid regex.",
        },
        (Some(_regex_str), None) => "Response was not provided.",
        (None, Some(_reply)) => "Regex was not provided.",
        _ => "Neither regex nor response was provided.",
    };
    int.create_interaction_response(ctx.http, |resp| {
        resp.interaction_response_data(|data| data.content(response))
    })
    .await?;
    Ok(())
}
