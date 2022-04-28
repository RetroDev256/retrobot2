mod events;

use events::Handler;
use serenity::{prelude::GatewayIntents, Client};

use std::{error::Error, fs::read_to_string};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let token_str = read_to_string("token/token.txt")?;
    let intents = GatewayIntents::all();
    let mut client = Client::builder(&token_str.trim(), intents)
        .event_handler(Handler)
        .await?;
    client.start_shards(num_cpus::get() as u64).await?;
    Ok(())
}
