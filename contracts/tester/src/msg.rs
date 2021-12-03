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
        sender: String,
    },
    MintNftMsg {
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
        denom_id: String,
        token_id: String,
        sender: String,
    },
    ApproveNftRequest {
        denom_id: String,
        token_id: String,
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    QueryDenomById { denom_id: String },
    QueryDenomByName { denom_name: String },
    QueryToken { denom_id: String, token_id: String },
}
