use cosmwasm_std::Coin;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cudos_cosmwasm::{PaginationRequest, Royalty};

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
        traits: Option<String>,
        minter: Option<String>,
        description: Option<String>,
        data: Option<String>,
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
    PublishCollectionMsg {
        denom_id: String,
        mint_royalties: Vec<Royalty>,
        resale_royalties: Vec<Royalty>,
    },
    PublishNftMsg {
        token_id: String,
        denom_id: String,
        price: Coin,
    },
    BuyNftMsg {
        id: u64,
    },
    MintNftMarketplaceMsg {
        denom_id: String,
        recipient: String,
        price: Coin,
        name: String,
        uri: Option<String>,
        data: Option<String>,
        uid: String,
    },
    RemoveNftMsg {
        id: u64,
    },
    VerifyCollectionMsg {
        id: u64,
    },
    UnverifyCollectionMsg {
        id: u64,
    },
    CreateCollectionMsg {
        id: String,
        name: String,
        symbol: String,
        schema: Option<String>,
        traits: Option<String>,
        minter: Option<String>,
        description: Option<String>,
        data: Option<String>,
        mint_royalties: Vec<Royalty>,
        resale_royalties: Vec<Royalty>,
        verified: bool,
    },
    UpdateRoyaltiesMsg {
        id: u64,
        mint_royalties: Vec<Royalty>,
        resale_royalties: Vec<Royalty>,
    },
    UpdatePriceMsg {
        id: u64,
        price: Coin,
    },
    AddAdminMsg {
        address: String,
    },
    RemoveAdminMsg {
        address: String,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
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
