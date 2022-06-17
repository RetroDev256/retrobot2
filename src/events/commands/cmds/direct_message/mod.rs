pub mod setup;

use serenity::{
    client::Context,
    model::prelude::interaction::application_command::{
        ApplicationCommandInteraction, CommandDataOptionValue,
    },
    utils::MessageBuilder,
};

use crate::tools::filter_pings;

use super::get_element;

pub async fn direct_message(int: ApplicationCommandInteraction, ctx: Context) {
    let user = match get_element(&int, 0) {
        CommandDataOptionValue::User(user, member) => Some((user, member)),
        _ => None,
    }
    .unwrap();
    let text = match get_element(&int, 1) {
        CommandDataOptionValue::String(text) => Some(text),
        _ => None,
    }
    .unwrap();
    let success = user
        .0
        .direct_message(&ctx.http, |create| {
            create.content(
                MessageBuilder::new()
                    .push(filter_pings(&int.user.name))
                    .push_line(" says:")
                    .push(filter_pings(text)),
            )
        })
        .await
        .is_ok();
    let reply = match success {
        true => "Success sending message",
        false => "Can't send message to that user",
    };
    int.create_interaction_response(ctx.http, |reponse| {
        reponse.interaction_response_data(|data| data.ephemeral(true).content(filter_pings(reply)))
    })
    .await
    .unwrap();
}
