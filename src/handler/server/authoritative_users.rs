use once_cell::sync::Lazy;
use serenity::model::id::UserId;
use std::collections::HashMap;

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
