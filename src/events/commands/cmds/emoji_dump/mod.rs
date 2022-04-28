pub mod setup;

use std::error::Error;

use serenity::{
    client::Context, model::interactions::application_command::ApplicationCommandInteraction,
    utils::MessageBuilder,
};

pub async fn emoji_dump(
    int: ApplicationCommandInteraction,
    ctx: Context,
) -> Result<(), Box<dyn Error>> {
    let mut message_build = MessageBuilder::new();
    match int.guild_id {
        Some(guild_id) => match guild_id.emojis(&ctx.http).await {
            Ok(emojis) => emojis.into_iter().for_each(|emoji| {
                message_build.emoji(&emoji);
            }),
            _ => {
                message_build.push("For some reason I can't seem to get the list of emojis.");
            }
        },
        _ => {
            message_build.push("What emojis would exist in this barren wasteland?");
        }
    }
    int.create_interaction_response(ctx.http, |response| {
        response.interaction_response_data(|data| data.content(message_build.build()))
    })
    .await?;
    Ok(())
}
