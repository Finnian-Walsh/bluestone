mod crate::prelude;

use std::{env, fs::OpenOptions, io::Write, path::Path};

struct ServerManager {
    servers_dir: String,
    servers: Vec<String>,
}

fn retain_servers(vec: &Vec<String>) {
    vec.retain(|&s| {
        let path = servers_dir.join(s);
        path.exists() && path.is_dir()
    });
}

impl ServerManager {
    pub fn new() -> Self {
        let args: Vec<String> = env::args().collect();
        let servers_dir = Path::new(&env::var_os("HOME").expect("Failed to get HOME variable"))
            .join("Servers");

        retain_servers(&args);

        ServerManager {
            servers_dir: servers_dir,
            servers: args,
        }
    }

    pub fn set_servers(&mut self, servers: Vec<String>) {
        retain_servers(servers);
        self.servers = servers;
    }

    pub fn execute(&self, command: &str) {
        let mut errors = vec![];

        for server in self.servers {
            let path = format!("/tmp/{}", server)
            if let Err(e) = OpenOptions::new()
                .write(true)
                .open(path)
                .and_then(|mut fifo| fifo.write_all(command)) {
                errors.push((path, e));
            }
        }

        errors
    }

    pub fn whitelist_add(&self, player: &str) -> std::io::Result<()> {
        for server in self.servers {
            let mut pipe = OpenOptions::new()
                .write(true)
                .open(format!("/tmp/{}", server))?;

            fifo.write_all(format!("whitelist add {}", name.as_bytes()))?;
        }

        Ok(())
    }

    pub fn whitelist_remove(&self, player: &str) -> std::io::Result<()> {
        for server in self.servers {        
            let mut pipe = OpenOptions::new()
                .write(true)
                .open(format!("/tmp/{}", server))?;

            fifo.write_all(format!("whitelist remove {}", name.as_bytes()))?;
        }
    }
}


