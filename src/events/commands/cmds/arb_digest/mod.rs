pub mod setup;

use arb_hash::digest::arb_digest;
use serenity::{
    client::Context,
    model::interactions::application_command::{
        ApplicationCommandInteraction, ApplicationCommandInteractionDataOptionValue,
    },
};
use std::io::Write;
use tempfile::Builder;

use super::get_element;

pub async fn digest(int: ApplicationCommandInteraction, ctx: Context) {
    let attachment = match get_element(&int, 0) {
        ApplicationCommandInteractionDataOptionValue::Attachment(file) => Some(file),
        _ => None,
    }
    .unwrap();
    let bytes = *match get_element(&int, 1) {
        ApplicationCommandInteractionDataOptionValue::Integer(bytes) => Some(bytes),
        _ => None,
    }
    .unwrap();
    let rounds = *match get_element(&int, 2) {
        ApplicationCommandInteractionDataOptionValue::Integer(rounds) => Some(rounds),
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
    let digest = arb_digest(&file, bytes as usize, rounds as u64);
    int.create_followup_message(&ctx.http, |data| {
        data.content("Converting to hexadecimal...")
    })
    .await
    .unwrap();
    let hex_bytes: String = digest
        .into_iter()
        .map(|byte| format!("{:02X}", byte))
        .collect();
    if bytes <= 997 {
        let message = format!("```{}```", hex_bytes);
        int.create_followup_message(&ctx.http, |data| data.content(message))
            .await
            .unwrap();
    } else {
        let mut tmp_content = Builder::new().suffix(".txt").tempfile().unwrap();
        int.create_followup_message(&ctx.http, |data| data.content("Writing to file..."))
            .await
            .unwrap();
        write!(tmp_content, "{}", hex_bytes).unwrap();
        int.create_followup_message(&ctx.http, |data| data.add_file(tmp_content.path()))
            .await
            .unwrap();
    }
}
