mod authoritative_users;
pub mod prelude;

use super::error::{Error, Result};
use authoritative_users::{AUTHORITATIVE_USERS, ExecutionAlias};
use mcserver::config;
use serenity::model::prelude::*;
use std::{env, process::Command, str::SplitWhitespace, sync::OnceLock};

static SERVERS: OnceLock<Vec<String>> = OnceLock::new();

pub fn get_servers() -> Result<&'static Vec<String>> {
    if let Some(servers) = SERVERS.get() {
        return Ok(servers);
    }

    let args: Vec<String> = env::args().skip(1).collect();

    let servers = if args.len() == 0 {
        let default_server = &config::get()?.default_server;
        if default_server.is_empty() {
            return Err(Error::NoServers);
        }

        vec![default_server.to_string()]
    } else {
        args
    };

    Ok(SERVERS.get_or_init(|| servers))
}

pub fn execute(command: &str) -> Result<()> {
    for server in get_servers()? {
        let output = Command::new("tmux")
            .arg("send-keys")
            .arg("-t")
            .arg(server)
            .arg(command)
            .arg("Enter")
            .output()?;

        if !output.status.success() {
            return Err(Error::CommandFailure {
                command_info: format!("sending keys to tmux session {}", server),
                code: output.status.code(),
                stderr: output.stderr,
            });
        }
    }

    Ok(())
}

pub fn whitelist_add(_sender: &User, target_player: &str) -> Result<()> {
    execute(&format!("whitelist add {}", target_player))
}

pub fn whitelist_remove(sender: &User, target_player: &str) -> Result<()> {
    let authority = AUTHORITATIVE_USERS
        .get(&sender.id)
        .ok_or(Error::InsufficientPermissions)?;

    if authority < &ExecutionAlias::Please {
        return Err(Error::InsufficientPermissions);
    }

    execute(&format!("whitelist remove {}", target_player))?;
    Ok(())
}

pub fn execute_request(
    sender: &User,
    alias: ExecutionAlias,
    mut command: SplitWhitespace,
) -> Result<()> {
    let authority = AUTHORITATIVE_USERS
        .get(&sender.id)
        .ok_or(Error::InsufficientPermissions)?;

    if *authority < alias {
        return Err(Error::InsufficientPermissions);
    }

    let command_str = if let Some(view) = command.next() {
        command.fold(view.to_string(), |mut accumulator, word| {
            accumulator.push(' ');
            accumulator.push_str(word);
            accumulator
        })
    } else {
        String::new()
    };

    execute(&command_str)?;
    Ok(())
}
