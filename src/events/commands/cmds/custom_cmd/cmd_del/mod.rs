use std::error::Error;

use serenity::{
    client::Context, model::interactions::application_command::ApplicationCommandInteraction,
};

pub mod setup;

pub async fn cmd_del(
    _int: ApplicationCommandInteraction,
    _ctx: Context,
) -> Result<(), Box<dyn Error>> {
    unimplemented!();
}
