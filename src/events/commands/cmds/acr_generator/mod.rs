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
use std::error::Error;

use self::dictionary::DICT;

pub async fn acr_generator(
    int: ApplicationCommandInteraction,
    ctx: Context,
) -> Result<(), Box<dyn Error>> {
    let input_option = match int.data.options.get(0) {
        Some(input_arg) => match input_arg.resolved.as_ref() {
            Some(ApplicationCommandInteractionDataOptionValue::String(input)) => Some(input),
            _ => None,
        },
        _ => None,
    };
    let mut builder = MessageBuilder::new();
    if let Some(input) = input_option {
        let mut thread_rng = ThreadRng::default();
        const ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        for c in input.to_uppercase().chars() {
            if let Some(pos) = ALPHABET.find(c) {
                let index: usize = thread_rng.gen_range(0..DICT[pos].len());
                builder.push(DICT[pos][index]).push(' ');
            }
        }
    }
    let built_message = builder.build();
    let message = match built_message.is_empty() {
        true => "Can't form that into an acronym.",
        false => &built_message,
    };
    int.create_interaction_response(ctx.http, |resp| {
        resp.interaction_response_data(|data| data.content(message))
    })
    .await?;
    Ok(())
}
