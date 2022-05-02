use serenity::{
    client::Context,
    model::interactions::application_command::{
        ApplicationCommandInteraction, ApplicationCommandInteractionDataOptionValue,
    },
};

use crate::{
    custom_cmds::remove_command, events::commands::cmds::get_element, tools::filter_pings,
};

pub mod setup;

pub async fn cmd_del(int: ApplicationCommandInteraction, ctx: Context) {
    let number = match get_element(&int, 0) {
        ApplicationCommandInteractionDataOptionValue::Integer(number) => Some(number),
        _ => None,
    }
    .unwrap();
    let description = match int.guild_id {
        Some(guild_id) => remove_command(guild_id.as_u64(), *number as usize),
        _ => "This is not a server.".to_owned(),
    };
    int.create_interaction_response(ctx.http, |resp| {
        resp.interaction_response_data(|data| data.content(filter_pings(&description)))
    })
    .await
    .unwrap();
}
