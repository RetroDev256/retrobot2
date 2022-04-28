use std::error::Error;

use serenity::{client::Context, model::gateway::Ready};

use super::commands::cmd_setup::setup_commands;

pub async fn shard_start(ctx: Context, ready: Ready) -> Result<(), Box<dyn Error>> {
    if let Some(shard) = ready.shard {
        println!(
            "{} is connected on shard {}/{}",
            ready.user.name,
            shard[0] + 1,
            shard[1]
        );
    }
    setup_commands(ctx, ready).await
}
