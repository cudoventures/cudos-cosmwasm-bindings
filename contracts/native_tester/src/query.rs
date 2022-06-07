use schemars::JsonSchema;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Balance { 
        address: String, 
        denom: String,
    },
    AllBalances { 
        address: String,
    },
    BondedDenom { },
    AllDelegations {
        delegator: String,
    },
    Delegation {
        delegator: String,
        validator: String,
    },
    AllValidators { },
    Validator {
        address: String,
    },
}