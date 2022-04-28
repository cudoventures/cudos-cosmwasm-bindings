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
    QueryDenoms {
        pagination: Option<PaginationRequest>,
    },
    QueryCollection {
        denom_id: String,
        pagination: Option<PaginationRequest>,
    },
    QuerySupply {
        denom_id: String,
    },
    QueryOwner {
        denom_id: Option<String>,
        address: String,
        pagination: Option<PaginationRequest>,
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
    pub denoms: Option<Vec<Denom>>,
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
    pub collection: Option<Collection>,
    pub pagination: Option<PageResponse>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PaginationRequest {
    // key is a value returned in PageResponse.next_key to begin
	// querying the next page most efficiently. Only one of offset or key
	// should be set.
	pub key: Option<String>,
	// offset is a numeric offset that can be used when key is unavailable.
	// It is less efficient than using key. Only one of offset or key should
	// be set.
	pub offset: Option<u64>,
	// limit is the total number of results to be returned in the result page.
	// If left empty it will default to a value to be set by each app.
	pub limit: Option<u64>,
	// count_total is set to true  to indicate that the result set should include
	// a count of the total number of items available for pagination in UIs.
	// count_total is only respected when offset is used. It is ignored when key
	// is set.
	pub count_total: Option<bool>,
	// reverse is set to true if results are to be returned in the descending order.
	//
	// Since: cosmos-sdk 0.43
	pub reverse: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PageResponse {
    pub next_key: Option<String>,
    pub total: Option<u64>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, JsonSchema)]
pub struct Collection {
    pub denom: Denom,
    pub nfts: Option<Vec<NFT>>,
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
