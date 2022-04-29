use std::error::Error;

use serenity::{
    client::Context,
    model::interactions::application_command::{
        ApplicationCommandInteraction, ApplicationCommandInteractionDataOptionValue,
    },
};

pub mod setup;

pub async fn say(int: ApplicationCommandInteraction, ctx: Context) -> Result<(), Box<dyn Error>> {
    if let Some(input_arg) = int.data.options.get(0) {
        if let Some(ApplicationCommandInteractionDataOptionValue::String(input)) =
            input_arg.resolved.as_ref()
        {
            int.channel_id.say(ctx.http, input).await?;
        }
    };
    Ok(())
}
