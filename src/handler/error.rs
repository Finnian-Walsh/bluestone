use mcserver;
use serenity;
use std::{
    fmt::{self, Display, Formatter},
    io, result,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    CommandFailure {
        code: Option<i32>,
        command_info: String,
        stderr: Vec<u8>,
    },
    InadequateAuthority,
    Io(#[from] io::Error),
    McServer(#[from] mcserver::Error),
    Serenity(#[from] serenity::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::CommandFailure {
                code,
                command_info,
                stderr,
            } => write!(
                f,
                "Command {} failed with code {}: {}",
                command_info,
                code.map(|c| c.to_string())
                    .unwrap_or_else(|| String::from("none")),
                String::from_utf8_lossy(stderr)
            ),
            Self::InadequateAuthority => write!(f, "Inadequate authority"),
            Self::Io(err) => write!(f, "{}", err),
            Self::McServer(err) => write!(f, "MC Server: {}", err),
            Self::Serenity(err) => write!(f, "Serenity: {}", err),
        }
    }
}

pub type Result<T> = result::Result<T, Error>;
