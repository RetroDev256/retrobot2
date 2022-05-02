use serenity::model::interactions::application_command::{
    ApplicationCommandInteraction, ApplicationCommandInteractionDataOptionValue,
};

pub mod acr_generator;
pub mod calc_ti_call;
pub mod custom_cmd;
pub mod direct_message;
pub mod emoji_dump;
pub mod msg_digest;
pub mod say;

pub fn get_element(
    int: &ApplicationCommandInteraction,
    index: usize,
) -> &ApplicationCommandInteractionDataOptionValue {
    return int
        .data
        .options
        .get(index)
        .unwrap()
        .resolved
        .as_ref()
        .unwrap();
}
