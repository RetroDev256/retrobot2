use std::error::Error;

use serenity::{
    client::Context,
    model::interactions::application_command::{
        ApplicationCommandInteraction, ApplicationCommandInteractionDataOptionValue,
    },
};

use crate::custom_cmds::remove_command;

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
            Some(guild_id) => remove_command(guild_id.as_u64(), value),
            _ => "This is not a server.".to_owned(),
        },
        _ => "You must supply the index of the command.".to_owned(),
    };
    int.create_interaction_response(ctx.http, |resp| {
        resp.interaction_response_data(|data| data.ephemeral(true).content(description))
    })
    .await?;
    Ok(())
}
