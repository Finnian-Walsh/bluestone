use crate::slash_commands;
use anyhow::{Context as AnyhowContext, Result};
use serenity::{
    all::Context,
    async_trait,
    model::{
        application::{Command, Interaction},
        channel::Message,
        gateway::Ready,
        id::GuildId,
    },
    prelude::*,
};

pub struct Handler;

const BOT_PREFIX: &str = "bluestone";

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            if let Err(err) = slash_commands::run(ctx, command).await {
                eprintln!("{err:?}");
            }
        }
    }

    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("{} is connected", ready.user.name);
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        }

        let _split_whitespace = msg.content.split_whitespace();

        if msg.content.len() != BOT_PREFIX.len() {
            return;
        }

        if msg.content.to_lowercase() != BOT_PREFIX {
            return;
        }

        let channel_id = msg.channel_id;
        let http = &ctx.http;

        if let Err(why) = channel_id
            .say(http, "# yo <a:ferris_moving:1408770783559553064>")
            .await
            .context("Failed to say yo")
        {
            eprintln!("{:?}", why);
        }
    }
}
