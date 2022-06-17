mod dictionary;
pub mod setup;

use rand::{prelude::ThreadRng, Rng};
use serenity::{
    client::Context,
    model::prelude::interaction::application_command::{
        ApplicationCommandInteraction, CommandDataOptionValue,
    },
    utils::MessageBuilder,
};

use crate::tools::filter_pings;

use self::dictionary::get_elem_list;

use super::get_element;

pub async fn acr_generator(int: ApplicationCommandInteraction, ctx: Context) {
    let input = match get_element(&int, 0) {
        CommandDataOptionValue::String(input) => Some(input),
        _ => None,
    }
    .unwrap();
    let mut builder = MessageBuilder::new();
    {
        let mut thread_rng = ThreadRng::default();
        for c in input.chars() {
            if let Some(list) = get_elem_list(c) {
                let index = thread_rng.gen_range(0..list.len());
                builder.push(list[index]).push(' ');
            }
        }
    }
    let message = filter_pings(&builder.build());
    int.create_interaction_response(ctx.http, |resp| {
        resp.interaction_response_data(|data| data.content(message.trim()))
    })
    .await
    .unwrap();
}
