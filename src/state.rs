use cosmwasm_std::Addr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub granter: Addr,
    pub allowed: Vec<Addr>,
}

// We might as well have one-letter keys like "c" or "n" to save ones and zeroes.
// CONFIG will typically store values that might be updated later using a special function that checks if the sender is allowed to change it.
pub const CONFIG: Item<Config> = Item::new("c");
