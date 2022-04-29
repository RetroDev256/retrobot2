use std::error::Error;

use serenity::{
    client::Context,
    model::interactions::application_command::{
        ApplicationCommandInteraction, ApplicationCommandInteractionDataOptionValue,
    },
};

use crate::tools::filter_pings;

pub mod setup;

pub async fn say(int: ApplicationCommandInteraction, ctx: Context) -> Result<(), Box<dyn Error>> {
    if let Some(input_arg) = int.data.options.get(0) {
        if let Some(ApplicationCommandInteractionDataOptionValue::String(input)) =
            input_arg.resolved.as_ref()
        {
            int.create_interaction_response(&ctx.http, |resp| {
                resp.interaction_response_data(|data| data.ephemeral(true).content("ok"))
            })
            .await?;
            int.channel_id.say(&ctx.http, filter_pings(input)).await?;
        }
    };
    Ok(())
}
