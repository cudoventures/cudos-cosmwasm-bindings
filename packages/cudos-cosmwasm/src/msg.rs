use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::CosmosMsg;

// this is a helper to be able to return these as CosmosMsg easier
impl From<CudosMsg> for CosmosMsg<CudosMsg> {
    fn from(original: CudosMsg) -> Self {
        CosmosMsg::Custom(original)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum CudosMsg {
    IssueDenom {
        id: String,
        name: String,
        schema: String,
        sender: String,
    },
}

// create_issue_denom_msg returns wrapped issue_denom msg
pub fn create_issue_denom_msg(
    id: String,
    name: String,
    schema: String,
    sender: String,
) -> CosmosMsg<CudosMsg> {
    CudosMsg::IssueDenom {
        id,
        name,
        schema,
        sender,
    }
    .into()
}
