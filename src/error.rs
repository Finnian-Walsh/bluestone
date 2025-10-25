use std::{fmt, io, result};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Command {} failed with code {}: {}", command_info, code
                .map(|c| c.to_string())
                .unwrap_or_else(|| String::from("none")),
            String::from_utf8_lossy(stderr)
    )]
    CommandFailure {
        code: Option<i32>,
        command_info: String,
        stderr: Vec<u8>,
    },

    #[error(transparent)]
    Fmt(#[from] fmt::Error),

    #[error("Insufficient permissions")]
    InsufficientPermissions,

    #[error(transparent)]
    Io(#[from] io::Error),

    #[error(transparent)]
    McServer(#[from] mcserver::Error),

    #[error("No servers were found")]
    NoServers,

    #[error(transparent)]
    Serenity(#[from] serenity::Error),

    #[error("Expected argument `{0}`")]
    ExpectedArgument(&'static str),
}

pub type Result<T> = result::Result<T, Error>;
