// This imports the state module from the contract crate, which contains the Entry, Priority, and Status structs, as well as the ENTRY_SEQ and LIST constants.
use crate::state::{Entry, Priority, Status};
// This imports cw_serde and QueryResponses from the cosmwasm_schema crate.
use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde] // Allows this struct to be serialized and deserialized
pub struct InstantiateMsg {
    // Optional string field to set the owner of the contract if one is not set the sender becomes the owner
    pub owner: Option<String>,
}

#[cw_serde]
pub enum ExecuteMsg {
    // Define the different types of messages that can be sent to the contract
    NewEntry {
        description: String,
        priority: Option<Priority>,
        owner: String,
    },
    UpdateEntry {
        id: u64,
        description: Option<String>,
        status: Option<Status>,
        priority: Option<Priority>,
        owner: String,
    },
    DeleteEntry {
        id: u64,
        owner: String,
    },
}

#[cw_serde]
// Define the different types of queries that can be sent to the contract
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ListResponse)]
    QueryUserList { 
        user: String, 
        // Tells the contract where to start the query by giving the ID of the last entry returned
        start_after: Option<u64>, 
        // Tells the contract how many entries to return
        limit: Option<u32> 
    },
}

#[cw_serde]
pub struct ListResponse {
    pub entries: Vec<Entry>,
}