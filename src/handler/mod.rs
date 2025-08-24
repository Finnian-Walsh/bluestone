mod server;
mod who;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use server::prelude::*;
use std::{str::SplitWhitespace, sync::OnceLock};

pub struct Handler {
    bot_mention: OnceLock<String>,
    bot_lower_username: OnceLock<String>,
}

impl Handler {
    pub fn new() -> Self {
        Self {
            bot_lower_username: OnceLock::new(),
            bot_mention: OnceLock::new(),
        }
    }

    fn set_bot_data(&self, ready: &Ready) {
        self.bot_mention
            .set(ready.user.mention().to_string())
            .ok()
            .unwrap();
        self.bot_lower_username
            .set(ready.user.name.to_lowercase())
            .ok()
            .unwrap();
    }

    fn get_bot_lower_username(&self) -> &str {
        self.bot_lower_username.get().unwrap()
    }

    fn get_bot_mention(&self) -> &str {
        self.bot_mention.get().unwrap()
    }

    async fn handle_command(&self, mut command: SplitWhitespace<'_>, ctx: Context, msg: &Message) -> Result<(), Box<dyn std::error::Error>> {
        let channel_id = msg.channel_id;
        let http = &ctx.http;

        let Some(command_word) = command.next() else {
            channel_id.say(http, "yo").await?;
            return Ok(());
        };

        match command_word {
            "hello" => {
                channel_id.say(http, "👋 hello!").await?;
            }
            "who" => {
                who::who(command, ctx, msg).await?;
            }
            "add" => {
                server::whitelist_add(&msg.author, match command.next() {
                    Some(target) => &target,
                    None => "",
                })?;
            }
            "remove" => {
                server::whitelist_remove(&msg.author, match command.next() {
                    Some(target) => &target,
                    None => ""
                })?;
            }
            "please" => {
                server::execute_request(&msg.author, ExecutionAlias::Please, command)?;
            }
            "execute" => {
                println!("execute");
                server::execute_request(&msg.author, ExecutionAlias::Execute, command)?;
            }
            _ => {
                channel_id.say(http, "idk").await?;
            }
        };

        Ok(())
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("{} is connected", ready.user.name);
        self.set_bot_data(&ready);

        println!(
            "To mention, use {}\n{} starting...",
            self.get_bot_mention(),
            self.get_bot_lower_username()
        );
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        }

        let mut split_whitespace = msg.content.split_whitespace();

        let has_mention = split_whitespace.next().is_some_and(|first_word| {
            let id_mentioned = first_word == self.get_bot_mention();
            if id_mentioned {
                return true;
            }
            let name_mentioned = first_word.to_lowercase() == self.get_bot_lower_username();
            name_mentioned
        });

        if !has_mention {
            return;
        }

        println!("Mentioned by {}", msg.author.name);

        if let Err(err) = self.handle_command(split_whitespace, ctx, &msg).await {
            eprintln!("{}", err);
        }
    }
}
