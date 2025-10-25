use serenity::{self, client::Context, model::prelude::*};
use std::str::SplitWhitespace;

fn get_user_info(user: &str) -> String {
    format!("[user info for {user}]")
}

pub async fn run(
    mut command: SplitWhitespace<'_>,
    ctx: &Context,
    msg: &Message,
) -> serenity::Result<()> {
    let channel_id = msg.channel_id;
    let http = &ctx.http;

    let Some(verb) = command.next() else {
        channel_id.say(http, "no verb").await?;
        return Ok(());
    };

    match verb {
        "is" => {
            let Some(user) = command.next() else {
                channel_id.say(http, "target not specified").await?;
                return Ok(());
            };

            if command.count() > 0 {
                channel_id
                    .say(http, "too many targets; use verb are instead")
                    .await?;
                return Ok(());
            }

            channel_id.say(http, &get_user_info(user)).await?;
        }
        "are" => {
            let mut user_info = String::new();

            for user in command.collect::<Vec<&str>>() {
                user_info.push_str(&get_user_info(user));
                user_info.push_str("\n\n")
            }

            channel_id.say(http, &user_info).await?;
        }
        _ => {
            channel_id.say(http, "unrecognised verb").await?;
        }
    };

    Ok(())
}
