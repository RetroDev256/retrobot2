use std::error::Error;

use serenity::{client::Context, model::gateway::Ready};

use super::cmds::{
    acr_generator::setup::acr_generator_setup, arb_digest::setup::arb_digest_setup,
    calc_ti_call::setup::calc_ti_call_setup, custom_cmd::setup::custom_cmd_setup,
    direct_message::setup::direct_message_setup, emoji_dump::setup::emoji_dump_setup,
    msg_digest::setup::msg_digest_setup, say::setup::say_setup,
};

pub async fn setup_commands(ctx: Context, ready: Ready) -> Result<(), Box<dyn Error>> {
    for guild in ready.guilds {
        guild
            .id
            .set_application_commands(&ctx.http, |cmds| {
                arb_digest_setup(cmds);
                acr_generator_setup(cmds);
                calc_ti_call_setup(cmds);
                direct_message_setup(cmds);
                emoji_dump_setup(cmds);
                custom_cmd_setup(cmds);
                msg_digest_setup(cmds);
                say_setup(cmds);
                cmds
            })
            .await?;
    }
    Ok(())
}
