use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cudos_cosmwasm::{
    PaginationRequest
};

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    IssueDenomMsg {
        id: String,
        name: String,
        symbol: String,
        schema: Option<String>,
    },
    MintNftMsg {
        denom_id: String,
        name: String,
        uri: Option<String>,
        data: Option<String>,
        recipient: String,
    },
    EditNftMsg {
        denom_id: String,
        token_id: String,
        name: Option<String>,
        uri: Option<String>,
        data: Option<String>,
    },
    TransferNftMsg {
        denom_id: String,
        token_id: String,
        from: String,
        to: String,
    },
    TransferDenomMsg {
        denom_id: String,
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
    QueryDenomBySymbol { denom_symbol: String },
    QueryDenoms { pagination: Option<PaginationRequest> },
    QueryCollection { denom_id: String, pagination: Option<PaginationRequest> },
    QuerySupply { denom_id: String },
    QueryOwner { denom_id: Option<String>, address: String, pagination: Option<PaginationRequest> },
    QueryToken { denom_id: String, token_id: String },
    QueryApprovals { denom_id: String, token_id: String },
    QueryApprovedForAll { owner_address: String, operator_address: String},
}
