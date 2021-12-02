use std::collections::HashMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::CustomQuery;

// implement custom query
impl CustomQuery for CudosQuery {}

/// CudosQuery is defines available query datas
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum CudosQuery {
    QueryDenomById { denom_id: String },
    QueryDenomByName { denom_name: String },
    QueryToken { denom_id: String, token_id: String },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct DenomResponse {
    pub denom: Denom,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, JsonSchema)]
pub struct Denom {
    pub id: String,
    pub name: String,
    pub schema: String,
    pub creator: String,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, JsonSchema)]
pub struct NFT {
    pub id: String,
    pub name: String,
    pub uri: String,
    pub data: String,
    pub owner: String,
    pub approved_addresses: HashMap<String, bool>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, JsonSchema)]
pub struct QueryNFTResponse {
    pub nft: NFT,
}
