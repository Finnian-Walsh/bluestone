mod user_info;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready, id::ChannelId},
    prelude::*,
};
use std::sync::OnceLock;
use user_info::get_user_info;

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
        self.bot_mention.set(ready.user.mention().to_string()).ok().unwrap();
        self.bot_lower_username.set(ready.user.name.to_lowercase()).ok().unwrap();
    }

    fn get_bot_lower_username(&self) -> &str {
        self.bot_lower_username.get().unwrap()
    }

    fn get_bot_mention(&self) -> &str {
        self.bot_mention.get().unwrap()
    }

    async fn say(&self, ctx: &Context, channel_id: &ChannelId, message: &str) {
        if let Err(why) = channel_id.say(&ctx.http, message).await {
            println!("Error sending message {message}: {:?}", why);
        } else {
            println!("Sent message {}", message)
        }
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("{} is connected", ready.user.name);
        self.set_bot_data(&ready);

        println!("{} and {}", self.get_bot_mention(), self.get_bot_lower_username());
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        }

        let content = msg.content;
        let mut split_whitespace = content.split_whitespace();

        let has_mention = split_whitespace.next().is_some_and(|first_word| {
            let id_mentioned = first_word == self.get_bot_mention();
            if id_mentioned { return true; }
            let name_mentioned = first_word.to_lowercase() == self.get_bot_lower_username();
            name_mentioned
        });

        if !has_mention {
            return;
        }

        let Some(command_word) = split_whitespace.next() else {
            self.say(&ctx, &msg.channel_id, "yo").await;
            return;
        };

        match command_word {
            "hello" => {
                self.say(&ctx, &msg.channel_id, "👋 hello!").await;
            },
            "who" => {
                let Some(verb) = split_whitespace.next() else {
                    self.say(&ctx, &msg.channel_id, "no verb").await;
                    return;
                };

                match verb {
                    "is" => {
                        let Some(user) = split_whitespace.next() else {
                            self.say(&ctx, &msg.channel_id, "target not specified").await;
                            return;
                        };

                        if split_whitespace.count() > 0 {
                            self.say(&ctx, &msg.channel_id, "too many targets; use are instead").await;
                            return;
                        }
                        
                        self.say(&ctx, &msg.channel_id, &get_user_info(user)).await;
                    },
                    "are" => {
                        let mut user_info = String::new();

                        for user in split_whitespace.collect::<Vec<&str>>() {
                            user_info.push_str(&get_user_info(user));
                            user_info.push_str("\n\n")
                        }

                        self.say(&ctx, &msg.channel_id, &user_info).await;
                    },
                    _ => self.say(&ctx, &msg.channel_id, "unrecognised verb").await,
                }
            },
            _ => self.say(&ctx, &msg.channel_id, "idk").await,
        }
    }
}

