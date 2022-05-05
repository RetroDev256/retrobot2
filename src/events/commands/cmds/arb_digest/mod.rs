pub mod setup;

use super::get_element;
use arb_hash::block::AHBlock;
use serenity::{
    client::Context,
    model::interactions::application_command::{
        ApplicationCommandInteraction, ApplicationCommandInteractionDataOptionValue,
    },
};

pub async fn digest(int: ApplicationCommandInteraction, ctx: Context) {
    let attachment = match get_element(&int, 0) {
        ApplicationCommandInteractionDataOptionValue::Attachment(file) => Some(file),
        _ => None,
    }
    .unwrap();
    int.create_interaction_response(&ctx.http, |resp| {
        resp.interaction_response_data(|data| data.content("Downloading file..."))
    })
    .await
    .unwrap();
    let file = attachment.download().await.unwrap();
    int.create_followup_message(&ctx.http, |data| {
        data.content("Hashing blocks in file, combining into digest...")
    })
    .await
    .unwrap();
    let digest = AHBlock::<64>::arb_digest_parallel::<2>(file.as_slice(), num_cpus::get());
    int.create_followup_message(&ctx.http, |data| {
        data.content("Converting to hexadecimal...")
    })
    .await
    .unwrap();
    let hex_bytes: String = digest
        .data
        .into_iter()
        .map(|byte| format!("{:02X}", byte))
        .collect();
    let message = format!("```{}```", hex_bytes);
    int.create_followup_message(&ctx.http, |data| data.content(message))
        .await
        .unwrap();
}
