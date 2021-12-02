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
        token_id: String,
        name: String,
        uri: String,
        data: String,
        sender: String,
    },
    TransferNft {
        denom_id: String,
        token_id: String,
        from: String,
        to: String,
        sender: String,
    },
    BurnNft {
        token_id: String,
        denom_id: String,
        sender: String,
    },
    ApproveNftRequest {
        token_id: String,
        denom_id: String,
        approved_address: String,
        sender: String,
    },
    ApproveAllRequest {
        approved_operator: String,
        approved: bool,
        sender: String,
    },
    RevokeApprovalRequest {
        address_to_revoke: String,
        denom_id: String,
        token_id: String,
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
    token_id: String,
    name: String,
    uri: String,
    data: String,
    sender: String,
) -> CosmosMsg<CudosMsg> {
    CudosMsg::EditNft {
        token_id,
        denom_id,
        name,
        uri,
        data,
        sender,
    }
    .into()
}

pub fn create_transfer_nft_msg(
    denom_id: String,
    token_id: String,
    from: String,
    to: String,
    sender: String,
) -> CosmosMsg<CudosMsg> {
    CudosMsg::TransferNft {
        denom_id,
        token_id,
        from,
        to,
        sender,
    }
    .into()
}

pub fn create_burn_nft_msg(
    denom_id: String,
    token_id: String,
    sender: String,
) -> CosmosMsg<CudosMsg> {
    CudosMsg::BurnNft {
        token_id,
        denom_id,
        sender,
    }
    .into()
}

pub fn create_approve_nft_msg(
    denom_id: String,
    token_id: String,
    approved_address: String,
    sender: String,
) -> CosmosMsg<CudosMsg> {
    CudosMsg::ApproveNftRequest {
        token_id,
        denom_id,
        approved_address,
        sender,
    }
    .into()
}

pub fn create_approve_all_msg(
    approved_operator: String,
    approved: bool,
    sender: String,
) -> CosmosMsg<CudosMsg> {
    CudosMsg::ApproveAllRequest {
        approved_operator,
        approved,
        sender,
    }
    .into()
}

pub fn create_revoke_msg(
    address_to_revoke: String,
    denom_id: String,
    token_id: String,
    sender: String,
) -> CosmosMsg<CudosMsg> {
    CudosMsg::RevokeApprovalRequest {
        address_to_revoke,
        denom_id,
        token_id,
        sender,
    }
    .into()
}
