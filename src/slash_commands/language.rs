use anyhow::{Context as AnyhowContext, Result};
use serenity::{self, client::Context, model::prelude::*};
use std::{
    fmt::{self, Display, Formatter, Write},
    str::SplitWhitespace,
};

struct Language {
    language: String,
}

impl Display for Language {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.language.as_str() {
            "rust" | "rs" => write!(f, "Rust: the best programming language of all time!"),
            "c++" | "cpp" => write!(f, "C++! Segmentation faults"),
            "python" | "py" => write!(f, "Py~~thon~~slug"),
            _ => write!(f, "Unknown language {}", self.language),
        }
    }
}

pub async fn run(command: SplitWhitespace<'_>, ctx: &Context, msg: &Message) -> Result<()> {
    let channel_id = msg.channel_id;
    let http = &ctx.http;

    let mut language_info = String::new();

    for language in command {
        writeln!(
            &mut language_info,
            "{}",
            Language {
                language: language.to_lowercase()
            }
        )?;
    }

    if language_info.is_empty() {
        channel_id
            .say(http, "No language(s) specified")
            .await
            .context("Failed to send response")?;
        return Ok(());
    }

    channel_id
        .say(http, language_info)
        .await
        .context("Failed to send response")?;

    Ok(())
}
