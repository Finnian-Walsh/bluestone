mod authoritative_users;
pub mod prelude;

use authoritative_users::{AUTHORITATIVE_USERS, ExecutionAlias};
use serenity::model::prelude::*;
use std::{
    env, fmt,
    fs::OpenOptions,
    io,
    path::{Path, PathBuf},
    process::Command,
    result,
    str::SplitWhitespace,
};

pub struct ServerManager {
    servers_dir: PathBuf,
    servers: Vec<String>,
}

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

fn retain_servers(servers_dir: &PathBuf, vec: &mut Vec<String>) {
    vec.retain(|s| {
        let path = servers_dir.join(s);
        path.exists() && path.is_dir()
    });
}

impl ServerManager {
    pub fn new() -> Self {
        let mut args: Vec<String> = env::args().collect();
        let servers_dir = Path::new(&env::var_os("HOME").expect("Failed to get HOME variable"))
            .join("Servers")
            .to_path_buf();

        retain_servers(&servers_dir, &mut args);

        ServerManager {
            servers_dir: servers_dir,
            servers: args,
        }
    }

    fn set_servers(&mut self, mut servers: Vec<String>) {
        retain_servers(&self.servers_dir, &mut servers);
        self.servers = servers;
    }

    fn execute(&self, command: &str) -> Result<()> {
        let mut errors = vec![];

        for server in &self.servers {
            match Command::new("tmux")
                .arg("send-keys")
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

    fn whitelist_add(&self, player: &str) -> Result<()> {
        self.execute(&format!("whitelist add {}, ", player))
    }

    fn whitelist_remove(&self, player: &str) -> Result<()> {
        self.execute(&format!("whitelist remove {}", player))
    }

    pub fn user_execute(
        &self,
        user: &User,
        alias: ExecutionAlias,
        command: SplitWhitespace,
    ) -> Result<()> {
        let Some(authority) = AUTHORITATIVE_USERS.get(&user.id) else {
            return Err(Error::InadequateAuthority);
        };

        if *authority < alias {
            return Err(Error::InadequateAuthority);
        }

        self.execute(&command.collect::<Vec<&str>>().join(" "))
    }
}
