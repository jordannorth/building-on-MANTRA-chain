// This imports cw_serde from the cosmwasm_schema crate, which is a serialization library for Cosmos SDK messages.
use cosmwasm_schema::cw_serde;
// This imports the Item and Map types from the cw_storage_plus crate, which are used to store data in the contract's storage.
use cw_storage_plus::{Item, Map};

// This annotation is used to automatically implement the Serialize and Deserialize traits for the Entry struct.
#[cw_serde]
// Public enum with public fields 
pub struct Entry {
    pub id: u64,
    pub description: String,
    pub status: Status,
    pub priority: Priority,
    pub owner: String,
}

#[cw_serde]
pub enum Status {
    ToDo,
    InProgress,
    Done,
    Cancelled,
}

#[cw_serde]
pub enum Priority {
    None,
    Low,
    Medium,
    High,
}

// ENTRY_SEQ is a constant with a key of "entry_seq" and a value of type u64. Item allows us to persist a single value in the contract's storage.
pub const ENTRY_SEQ: Item<u64> = Item::new("entry_seq");
pub const LIST: Map<u64, Entry> = Map::new("list");