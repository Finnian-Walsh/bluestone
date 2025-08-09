pub mod prelude;
mod authoritative_users;

use authoritative_users::{AUTHORITATIVE_USERS, ExecutionAlias, ExecutionError};
use serenity::model::prelude::*;
use std::{env, fs::OpenOptions, io::Write, path::{Path, PathBuf}, str::SplitWhitespace};

pub struct ServerManager {
    servers_dir: PathBuf,
    servers: Vec<String>,
}

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

    fn execute(&self, command: &str) -> authoritative_users::Result<()> {
        let mut errors = vec![];

        for server in &self.servers {
            let path = format!("/tmp/{}", server);
            if let Err(e) = OpenOptions::new().write(true).open(&path).and_then(|mut fifo| fifo.write_all(format!("{}\n", command).as_bytes())) {
                errors.push((path, e));
            }
        }

        if errors.len() > 0 {
            return Err(ExecutionError::Io(errors));
        }

        Ok(())
    }

    fn whitelist_add(&self, player: &str) -> authoritative_users::Result<()> {
        self.execute(&format!("whitelist add {}, ", player))
    }

    fn whitelist_remove(&self, player: &str) -> authoritative_users::Result<()> {
        self.execute(&format!("whitelist remove {}", player))
    }

    pub fn user_execute(&self, user: &User, alias: ExecutionAlias, command: SplitWhitespace) -> authoritative_users::Result<()> {
        let Some(authority) = AUTHORITATIVE_USERS.get(&user.id) else {
            return Err(ExecutionError::InadequateAuthority);
        };

        if *authority < alias {
            return Err(ExecutionError::InadequateAuthority);
        }

        self.execute(&command.collect::<Vec<&str>>().join(" "))
    }
}


