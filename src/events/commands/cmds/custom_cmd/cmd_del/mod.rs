use std::error::Error;

use serenity::{
    client::Context,
    model::interactions::application_command::{
        ApplicationCommandInteraction, ApplicationCommandInteractionDataOptionValue,
    },
};

use crate::custom_cmds::CUST_CMDS;

pub mod setup;

pub async fn cmd_del(
    int: ApplicationCommandInteraction,
    ctx: Context,
) -> Result<(), Box<dyn Error>> {
    let number_option = match int.data.options.get(0) {
        Some(number_arg) => match number_arg.resolved.as_ref() {
            Some(ApplicationCommandInteractionDataOptionValue::Integer(number)) => {
                Some(*number as usize)
            }
            _ => None,
        },
        _ => None,
    };
    let description = match number_option {
        Some(value) => match int.guild_id {
            Some(guild_id) => match CUST_CMDS.write() {
                Ok(mut lock) => match lock.get_mut(guild_id.as_u64()) {
                    Some(server_cmds) => {
                        let key_opt = match server_cmds.iter_mut().nth(value) {
                            Some(entry) => Some(entry.0.clone()),
                            _ => None,
                        };
                        match key_opt {
                            Some(key) => {
                                let _ = server_cmds.remove(&key);
                                "Removed command from list of server commands."
                            }
                            _ => "Command was not present in list of commands.",
                        }
                    }
                    _ => "Server has no commands.",
                },
                _ => "Couldn't acquire lock on command list.",
            },
            _ => "This is not a server.",
        },
        _ => "You must supply the index of the command.",
    };
    int.create_interaction_response(ctx.http, |resp| {
        resp.interaction_response_data(|data| data.ephemeral(true).content(description))
    })
    .await?;
    Ok(())
}
