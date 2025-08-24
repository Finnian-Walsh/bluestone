pub mod prelude;
mod authoritative_users;

use authoritative_users::{AUTHORITATIVE_USERS, ExecutionAlias};
use mc_server::config;
use serenity::model::prelude::*;
use std::{
    env, fmt, io,
    process::Command,
    result,
    str::SplitWhitespace,
    sync::OnceLock
};

#[derive(Debug)]
pub enum CommandExecutionError {
    NotExecuted(io::Error),
    Failure {
        code: String,
        command: String,
        stderr: Vec<u8>,
    },
}

impl fmt::Display for CommandExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommandExecutionError::NotExecuted(err) => write!(f, "{}", err),
            CommandExecutionError::Failure {
                code,
                command,
                stderr,
            } => write!(
                f,
                "Failed to execute command '{}' (error code {}): {}",
                command,
                code,
                String::from_utf8_lossy(stderr)
            ),
        }
    }
}

impl std::error::Error for CommandExecutionError {}

#[derive(Debug)]
pub enum Error {
    CommandExecution(Vec<CommandExecutionError>),
    InadequateAuthority,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::CommandExecution(errors) => {
                for err in errors {
                    write!(f, "{}", err)?;
                }
                Ok(())
            }
            Error::InadequateAuthority => write!(f, "inadequate authority"),
        }
    }
}

pub type Result<T> = result::Result<T, Error>;

impl std::error::Error for Error {}

static SERVERS: OnceLock<Vec<String>> = OnceLock::new();

pub fn execute(command: &str) -> Result<()> {
    let mut errors = vec![];

    for server in SERVERS.get_or_init(|| {
        let args: Vec<String> = env::args().skip(1).collect();

        if args.len() == 0 {
            match config::get_default() {
                Ok(server) => vec![server],
                Err(err) => {
                    match err.kind() {
                        io::ErrorKind::NotFound => {}
                        _ => {
                            eprintln!("{}", err);
                        }
                    };

                    vec![]
                }
            }
        } else {
            args
        }
    }) {
        match Command::new("tmux")
            .arg("send-keys")
            .arg("-t")
            .arg(server)
            .arg(command)
            .arg("Enter")
            .output()
        {
            Ok(output) => {
                if !output.status.success() {
                    errors.push(CommandExecutionError::Failure {
                        code: match output.status.code() {
                            Some(code) => code.to_string(),
                            None => "none".to_string(),
                        },
                        command: command.to_string(),
                        stderr: output.stderr,
                    });
                }
            }
            Err(err) => errors.push(CommandExecutionError::NotExecuted(err)),
        }
    }

    if errors.len() > 0 {
        return Err(Error::CommandExecution(errors));
    }

    Ok(())
}

pub fn whitelist_add(_sender: &User, target_player: &str) -> Result<()> {
    execute(&format!("whitelist add {}", target_player))
}

pub fn whitelist_remove(sender: &User, target_player: &str) -> Result<()> {
    let authority = AUTHORITATIVE_USERS
        .get(&sender.id)
        .ok_or(Error::InadequateAuthority)?;

    if authority < &ExecutionAlias::Please {
        return Err(Error::InadequateAuthority);
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
        .ok_or(Error::InadequateAuthority)?;

    if *authority < alias {
        return Err(Error::InadequateAuthority);
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
