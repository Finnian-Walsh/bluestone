use once_cell::sync::Lazy;
use serenity::model::id::UserId;
use std::{collections::HashMap, fmt, io};

#[repr(u8)]
#[derive(Debug, PartialEq, PartialOrd)]
pub enum ExecutionAlias {
    Please,
    Execute,
}

pub static AUTHORITATIVE_USERS: Lazy<HashMap<UserId, ExecutionAlias>> = Lazy::new(|| {
    HashMap::from([
        (UserId::new(751484442454917231), ExecutionAlias::Execute), // silver
        (UserId::new(836287026327191582), ExecutionAlias::Execute), // redstone
    ])
});

#[derive(Debug)]
pub enum ExecutionError {
    Io(Vec<(String, io::Error)>),
    InadequateAuthority,
}

impl fmt::Display for ExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExecutionError::Io(errors) => {
                writeln!(f, "IO error(s):");
                for (path, err) in errors.iter() {
                    writeln!(f, "Error in {}: {}", path, err);
                }

                Ok(())
            },
            ExecutionError::InadequateAuthority => write!(f, "inadequate authority"), 
        }
    }
}

pub type Result<T> = std::result::Result<T, ExecutionError>;

