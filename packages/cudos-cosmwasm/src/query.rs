use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Coin, CustomQuery};

use crate::msg::Royalty;

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
    QueryCollectionsByDenomIds {
        denom_ids: Vec<String>,
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
    QueryCollectionMarketplace {
        id: u64,
    },
    QueryAllCollections {
        pagination: Option<PaginationRequest>,
    },
    QueryCollectionByDenomId {
        denom_id: String,
    },
    QueryNft {
        id: u64,
    },
    QueryAllNfts {
        pagination: Option<PaginationRequest>,
    },
    QueryListAdmins {},
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
    pub owner: Owner,
    pub pagination: Option<PageResponse>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Owner {
    pub address: String,
    pub id_collections: Vec<IDCollection>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct IDCollection {
    pub denom_id: String,
    pub token_ids: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct SupplyResponse {
    pub amount: u64,
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
    pub symbol: String,
    pub traits: Option<String>,
    pub minter: Option<String>,
    pub description: Option<String>,
    pub data: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, JsonSchema)]
pub struct CollectionResponse {
    pub collection: Option<Collection>,
    pub pagination: Option<PageResponse>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, JsonSchema)]
pub struct CollectionsResponse {
    pub collections: Option<Vec<Collection>>,
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

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, JsonSchema)]
pub struct QueryCollectionMarketplaceResponse {
    #[serde(alias = "Collection")]
    pub collection: MarketplaceCollection,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, JsonSchema)]
pub struct QueryAllCollectionsResponse {
    #[serde(alias = "Collection")]
    pub collections: Vec<MarketplaceCollection>,
    pub pagination: Option<PageResponse>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, JsonSchema)]
pub struct QueryCollectionByDenomIdResponse {
    #[serde(alias = "Collection")]
    pub collection: MarketplaceCollection,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, JsonSchema)]
pub struct QueryNftMarketplaceResponse {
    #[serde(alias = "Nft")]
    pub nft: MarketplaceNft,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, JsonSchema)]
pub struct QueryAllNftsResponse {
    #[serde(alias = "Nft", default = "empty_marketplace_nft_array")]
    pub nfts: Vec<MarketplaceNft>,
    pub pagination: Option<PageResponse>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, JsonSchema)]
pub struct QueryListAdminsResponse {
    #[serde(alias = "Admins", default = "empty_string_array")]
    pub admins: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, JsonSchema)]
pub struct MarketplaceCollection {
    pub id: u64,
    #[serde(alias = "denomId")]
    pub denom_id: String,
    #[serde(alias = "mintRoyalties")]
    pub mint_royalties: Vec<Royalty>,
    #[serde(alias = "resaleRoyalties")]
    pub resale_royalties: Vec<Royalty>,
    pub verified: bool,
    pub owner: String,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, JsonSchema)]
pub struct MarketplaceNft {
    pub id: u64,
    #[serde(alias = "tokenId")]
    pub token_id: String,
    #[serde(alias = "denomId")]
    pub denom_id: String,
    pub price: Coin,
    pub owner: String,
}

fn empty_string_array() -> Vec<String> {
    Vec::new()
}

fn empty_marketplace_nft_array() -> Vec<MarketplaceNft> {
    Vec::new()
}
