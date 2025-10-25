mod language;
mod server;
mod who;

// use anyhow::{Context as AnyhowContext, Result};
use serenity::{
    all::Context,
    // builder::{CreateInteractionResponse, CreateInteractionResponseMessage},
    model::prelude::*,
};

pub async fn run(_ctx: Context, command: CommandInteraction) -> Result<()> {
    match command.data.name.as_str() {
        "ping" => {
            println!("POng");
        }
        _ => todo!(),
    };
    Ok(())
}
