pub mod cmd_setup;
pub mod cmds;

use std::error::Error;

use serenity::{client::Context, model::interactions::Interaction};

use self::cmds::{
    acr_generator::acr_generator,
    calc_ti_call::calc_ti_call,
    custom_cmd::{cmd_add::cmd_add, cmd_del::cmd_del, cmd_list::cmd_list},
    direct_message::direct_message,
    emoji_dump::emoji_dump,
    say::say,
};

pub async fn command_manage(ctx: Context, interaction: Interaction) -> Result<(), Box<dyn Error>> {
    if let Interaction::ApplicationCommand(cmd_int) = interaction {
        match cmd_int.data.name.as_str() {
            "dm" => direct_message(cmd_int, ctx).await,
            "emojis" => emoji_dump(cmd_int, ctx).await,
            "acr" => acr_generator(cmd_int, ctx).await,
            "calc" => calc_ti_call(cmd_int, ctx).await,
            "say" => say(cmd_int, ctx).await,
            "cmd_add" => cmd_add(cmd_int, ctx).await,
            "cmd_del" => cmd_del(cmd_int, ctx).await,
            "cmd_list" => cmd_list(cmd_int, ctx).await,
            _ => Ok(()),
        }?;
    }
    Ok(())
}
