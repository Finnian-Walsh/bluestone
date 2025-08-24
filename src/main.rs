mod handler;

use dotenvy::dotenv;
use handler::Handler;
use serenity::prelude::*;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let token = env::var("BLUESTONE_TOKEN").expect("Expected BLUESTONE_TOKEN in environment");
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler::new())
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {}", why);
    }
}
