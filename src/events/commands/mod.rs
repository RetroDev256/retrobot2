pub mod cmd_setup;
pub mod cmds;

use std::error::Error;

use serenity::{client::Context, model::interactions::Interaction};

use self::cmds::{
    acr_generator::acr_generator, calc_ti_call::calc_ti_call, direct_message::direct_message,
    emoji_dump::emoji_dump,
};

pub async fn command_manage(ctx: Context, interaction: Interaction) -> Result<(), Box<dyn Error>> {
    if let Interaction::ApplicationCommand(cmd_int) = interaction {
        match cmd_int.data.name.as_str() {
            "dm" => direct_message(cmd_int, ctx).await,
            "emojis" => emoji_dump(cmd_int, ctx).await,
            "acr" => acr_generator(cmd_int, ctx).await,
            "calc" => calc_ti_call(cmd_int, ctx).await,
            _ => Ok(()),
        }?;
    }
    Ok(())
}
