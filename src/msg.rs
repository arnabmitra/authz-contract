use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub allowed: Vec<String>, //optional if exists will only transfer to these addesses
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    TransferAuthFunds {
        /// The `to_address`
        to_address: String,
        granter_address: String,
        denom: String,
        amount: String,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    /// Returns the address that is associated to this contract.
    /// The address returned is the one that granted authz access to
    /// claim rewards and delegate.
    Granter {},
}
