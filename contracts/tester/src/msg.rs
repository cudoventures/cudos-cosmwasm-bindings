use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    IssueDenomMsg {
        id: String,
        name: String,
        schema: String,
    },
    MintNftMsg {
        denom_id: String,
        name: String,
        uri: String,
        data: String,
        recipient: String,
    },
    EditNft {
        denom_id: String,
        token_id: String,
        name: String,
        uri: String,
        data: String,
    },
    TransferNft {
        denom_id: String,
        token_id: String,
        from: String,
        to: String,
    },
    BurnNft {
        denom_id: String,
        token_id: String,
    },
    ApproveNftRequest {
        denom_id: String,
        token_id: String,
        approved_address: String,
    },
    ApproveAllRequest {
        approved_operator: String,
        approved: bool,
    },
    RevokeApprovalRequest {
        denom_id: String,
        token_id: String,
        address_to_revoke: String,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    QueryDenomById { denom_id: String },
    QueryDenomByName { denom_name: String },
    QueryToken { denom_id: String, token_id: String },
}
