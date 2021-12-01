use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::CosmosMsg;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
/// CudosMsgWrapper is an override of CosmosMsg::Custom to show this works and can be extended in the contract
pub struct CudosMsgWrapper {
    pub msg_data: CudosMsg,
}

// this is a helper to be able to return these as CosmosMsg easier
impl From<CudosMsgWrapper> for CosmosMsg<CudosMsgWrapper> {
    fn from(original: CudosMsgWrapper) -> Self {
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
) -> CosmosMsg<CudosMsgWrapper> {
    CudosMsgWrapper {
        msg_data: CudosMsg::IssueDenom {
            id,
            name,
            schema,
            sender,
        },
    }
    .into()
}
