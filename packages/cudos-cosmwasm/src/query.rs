use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::CustomQuery;

// implement custom query
impl CustomQuery for CudosQuery {}

/// CudosQuery is defines available query datas
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum CudosQuery {
    QueryDenomById {
        denom_id: String,
    },
    QueryDenomByName {
        denom_name: String,
    },
    QueryDenomBySymbol {
        denom_symbol: String,
    },
    QueryDenoms {},
    QueryCollection {
        denom_id: String,
    },
    QuerySupply {
        denom_id: String,
    },
    QueryOwner {
        denom_id: Option<String>,
        address: String,
    },
    QueryToken {
        denom_id: String,
        token_id: String,
    },
    QueryApprovals {
        denom_id: String,
        token_id: String,
    },
    QueryApprovedForAll {
        owner_address: String,
        operator_address: String,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct QueryApprovalsResponse {
    pub approved_addresses: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct QueryApprovedForAllResponse {
    pub is_approved: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct OwnerCollectionResponse {
    owner: Owner,
    pub pagination: Option<PageResponse>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Owner {
    address: String,
    id_collections: Vec<IDCollection>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct IDCollection {
    denom_id: String,
    token_ids: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct SupplyResponse {
    amount: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct DenomResponse {
    pub denom: Denom,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct DenomsResponse {
    pub denoms: Vec<Denom>,
    pub pagination: Option<PageResponse>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, JsonSchema)]
pub struct Denom {
    pub id: String,
    pub name: String,
    pub schema: Option<String>,
    pub creator: String,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, JsonSchema)]
pub struct CollectionResponse {
    pub collection: Collection,
    pub pagination: Option<PageResponse>,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PageResponse {
    pub next_key: Option<Vec<u8>>,
    pub total: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, JsonSchema)]
pub struct Collection {
    pub denom: Denom,
    pub nfts: Vec<NFT>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, JsonSchema)]
pub struct NFT {
    pub id: String,
    pub name: Option<String>,
    pub uri: Option<String>,
    pub data: Option<String>,
    pub owner: String,
    pub approved_addresses: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, JsonSchema)]
pub struct QueryNFTResponse {
    pub nft: NFT,
}
