pub mod setup;

use super::get_element;
use arb_hash::digest::arb_digest;
use serenity::{
    client::Context,
    model::interactions::application_command::{
        ApplicationCommandInteraction, ApplicationCommandInteractionDataOptionValue,
    },
};

pub async fn msg_digest(int: ApplicationCommandInteraction, ctx: Context) {
    let text = match get_element(&int, 0) {
        ApplicationCommandInteractionDataOptionValue::String(text) => Some(text),
        _ => None,
    }
    .unwrap();
    int.create_interaction_response(&ctx.http, |resp| {
        resp.interaction_response_data(|data| data.content("Hashing blocks in file, combining into digest..."))
    })
    .await
    .unwrap();
    let digest = arb_digest::<2, 64>(text.as_bytes());
    int.create_followup_message(&ctx.http, |data| {
        data.content("Converting to hexadecimal...")
    })
    .await
    .unwrap();
    let hex_bytes: String = digest
        .into_iter()
        .map(|byte| format!("{:02X}", byte))
        .collect();
    let message = format!("```{}```", hex_bytes);
    int.create_followup_message(&ctx.http, |data| data.content(message))
        .await
        .unwrap();
}
