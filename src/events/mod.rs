mod commands;
mod greets;
mod message;
mod ready;

use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::{channel::Message, gateway::Ready, guild::Member, interactions::Interaction},
};

use self::{commands::command_manage, greets::greet, message::message_event, ready::shard_start};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        if shard_start(_ctx, ready).await.is_err() {
            println!("Failed to ready shard.");
        }
    }
    async fn guild_member_addition(&self, ctx: Context, new_member: Member) {
        if greet(ctx, &new_member).await.is_err() {
            println!("Failed to greet {}.", new_member.display_name());
        }
    }
    async fn message(&self, ctx: Context, msg: Message) {
        if message_event(ctx, msg).await.is_err() {
            println!("Failed to fully process message.");
        }
    }
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if command_manage(ctx, interaction).await.is_err() {
            println!("Failed to manage slash command.");
        }
    }
}
