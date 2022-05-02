mod dictionary;
pub mod setup;

use rand::{prelude::ThreadRng, Rng};
use serenity::{
    client::Context,
    model::interactions::application_command::{
        ApplicationCommandInteraction, ApplicationCommandInteractionDataOptionValue,
    },
    utils::MessageBuilder,
};

use crate::tools::filter_pings;

use self::dictionary::get_elem_list;

use super::get_element;

pub async fn acr_generator(int: ApplicationCommandInteraction, ctx: Context) {
    let input = match get_element(&int, 0) {
        ApplicationCommandInteractionDataOptionValue::String(input) => Some(input),
        _ => None,
    }
    .unwrap();
    let mut builder = MessageBuilder::new();
    {
        let mut thread_rng = ThreadRng::default();
        for c in input.chars() {
            match get_elem_list(c) {
                Some(list) => {
                    let index = thread_rng.gen_range(0..list.len());
                    builder.push(list[index]).push(' ')
                }
                _ => builder.push(format!("[No {}'s]", c)),
            };
        }
    }
    let message = filter_pings(&builder.build());
    int.create_interaction_response(ctx.http, |resp| {
        resp.interaction_response_data(|data| data.content(message))
    })
    .await
    .unwrap();
}
