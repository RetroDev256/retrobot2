use serenity::{
    client::Context,
    model::interactions::application_command::{
        ApplicationCommandInteraction, ApplicationCommandInteractionDataOptionValue,
    },
};

use crate::{custom_cmds::add_command, events::commands::cmds::get_element, tools::filter_pings};

pub mod setup;

pub async fn cmd_add(int: ApplicationCommandInteraction, ctx: Context) {
    let regex = match get_element(&int, 0) {
        ApplicationCommandInteractionDataOptionValue::String(regex) => Some(regex),
        _ => None,
    }
    .unwrap();
    let reply = match get_element(&int, 1) {
        ApplicationCommandInteractionDataOptionValue::String(reply) => Some(reply),
        _ => None,
    }
    .unwrap();
    let response = match int.guild_id {
        Some(server_id) => add_command(server_id.as_u64(), regex.to_owned(), reply.to_owned()),
        _ => "This is not a server".to_owned(),
    };
    int.create_interaction_response(ctx.http, |resp| {
        resp.interaction_response_data(|data| data.content(filter_pings(&response)))
    })
    .await
    .unwrap();
}
