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
    EditNftMsg {
        denom_id: String,
        token_id: String,
        name: String,
        uri: String,
        data: String,
    },
    TransferNftMsg {
        denom_id: String,
        token_id: String,
        from: String,
        to: String,
    },
    BurnNftMsg {
        denom_id: String,
        token_id: String,
    },
    ApproveNftMsg {
        denom_id: String,
        token_id: String,
        approved_address: String,
    },
    ApproveAllMsg {
        approved_operator: String,
        approved: bool,
    },
    RevokeApprovalMsg {
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
