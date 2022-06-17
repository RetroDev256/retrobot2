pub mod setup;

use serenity::{
    client::Context,
    model::{
        guild::Emoji, id::GuildId,
        prelude::interaction::application_command::ApplicationCommandInteraction,
    },
};

pub async fn emoji_dump(int: ApplicationCommandInteraction, ctx: Context) {
    let guild_id: GuildId = int.guild_id.unwrap();
    let emojis: Vec<Emoji> = guild_id.emojis(&ctx.http).await.unwrap();
    let mut emoji_str: Vec<String> = emojis.iter().map(|emoji| emoji.to_string()).collect();
    let mut message = String::new();
    int.create_interaction_response(&ctx.http, |resp| {
        resp.interaction_response_data(|data| data.ephemeral(true).content("Enumerating emojis..."))
    })
    .await
    .unwrap();
    while let Some(emoji) = emoji_str.pop() {
        if message.len() + emoji.len() > 2000 {
            int.channel_id
                .send_message(&ctx.http, |data| data.content(&message))
                .await
                .unwrap();
            message = String::new();
        }
        message = [message, emoji].concat();
    }
    int.channel_id
        .send_message(&ctx.http, |data| data.content(&message))
        .await
        .unwrap();
}
