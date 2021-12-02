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
    MintNft {
        denom_id: String,
        name: String,
        uri: String,
        data: String,
        sender: String,
        recipient: String,
    },
    EditNft {
        denom_id: String,
        name: String,
        uri: String,
        data: String,
        sender: String,
    },
}

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

pub fn create_mint_nft_msg(
    denom_id: String,
    name: String,
    uri: String,
    data: String,
    sender: String,
    recipient: String,
) -> CosmosMsg<CudosMsg> {
    CudosMsg::MintNft {
        denom_id,
        name,
        uri,
        data,
        sender,
        recipient,
    }
    .into()
}

pub fn create_edit_nft_msg(
    denom_id: String,
    name: String,
    uri: String,
    data: String,
    sender: String,
) -> CosmosMsg<CudosMsg> {
    CudosMsg::EditNft {
        denom_id,
        name,
        uri,
        data,
        sender,
    }
    .into()
}
