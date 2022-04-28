pub mod setup;

use std::error::Error;

use serenity::{
    client::Context,
    model::interactions::application_command::{
        ApplicationCommandInteraction, ApplicationCommandInteractionDataOptionValue,
    },
    utils::MessageBuilder,
};

pub async fn direct_message(
    int: ApplicationCommandInteraction,
    ctx: Context,
) -> Result<(), Box<dyn Error>> {
    let user_option = match int.data.options.get(0) {
        Some(user_arg) => match user_arg.resolved.as_ref() {
            Some(ApplicationCommandInteractionDataOptionValue::User(user, member)) => {
                Some((user, member))
            }
            _ => None,
        },
        _ => None,
    };
    let text_option = match int.data.options.get(1) {
        Some(text_arg) => match text_arg.resolved.as_ref() {
            Some(ApplicationCommandInteractionDataOptionValue::String(text)) => Some(text),
            _ => None,
        },
        _ => None,
    };
    let success = match user_option {
        Some((user, _member)) => match text_option {
            Some(text) => user
                .direct_message(&ctx.http, |create| {
                    create.content(
                        MessageBuilder::new()
                            .push(&int.user.name)
                            .push_line(" says:")
                            .push(text),
                    )
                })
                .await
                .is_ok(),
            _ => false,
        },
        _ => false,
    };
    let reply = match success {
        true => "Success sending message",
        false => "Can't send message to that user",
    };
    int.create_interaction_response(ctx.http, |reponse| {
        reponse.interaction_response_data(|data| data.ephemeral(true).content(reply))
    })
    .await?;
    Ok(())
}
