use cosmwasm_std::{QuerierWrapper, StdResult};

use crate::{
    query::{
        CollectionResponse, CollectionsResponse, CudosQuery, DenomResponse, DenomsResponse,
        OwnerCollectionResponse, PaginationRequest, QueryAllAdressesResponse,
        QueryAllCollectionsResponse, QueryAllNftsResponse, QueryApprovalsResponse,
        QueryApprovedForAllResponse, QueryCollectionByDenomIdResponse,
        QueryCollectionMarketplaceResponse, QueryListAdminsResponse, QueryNFTResponse,
        QueryNftMarketplaceResponse, SupplyResponse,
    },
    QueryAdressResponse,
};

pub struct CudosQuerier<'a> {
    querier: &'a QuerierWrapper<'a, CudosQuery>,
}

impl<'a> CudosQuerier<'a> {
    pub fn new(querier: &'a QuerierWrapper<'a, CudosQuery>) -> Self {
        CudosQuerier { querier }
    }

    pub fn query_denom_by_id<T: Into<String>>(&self, denom_id: T) -> StdResult<DenomResponse> {
        let request = CudosQuery::QueryDenomById {
            denom_id: denom_id.into(),
        }
        .into();

        self.querier.query(&request)
    }

    pub fn query_denom_by_name<T: Into<String>>(&self, denom_name: T) -> StdResult<DenomResponse> {
        let request = CudosQuery::QueryDenomByName {
            denom_name: denom_name.into(),
        }
        .into();

        self.querier.query(&request)
    }

    pub fn query_denom_by_symbol<T: Into<String>>(&self, symbol: T) -> StdResult<DenomResponse> {
        let request = CudosQuery::QueryDenomBySymbol {
            denom_symbol: symbol.into(),
        }
        .into();

        self.querier.query(&request)
    }

    pub fn query_denoms(&self, pagination: Option<PaginationRequest>) -> StdResult<DenomsResponse> {
        let request = CudosQuery::QueryDenoms {
            pagination: pagination,
        }
        .into();

        self.querier.query(&request)
    }

    pub fn query_collection<T: Into<String>>(
        &self,
        denom_id: T,
        pagination: Option<PaginationRequest>,
    ) -> StdResult<CollectionResponse> {
        let request = CudosQuery::QueryCollection {
            denom_id: denom_id.into(),
            pagination: pagination.into(),
        }
        .into();

        self.querier.query(&request)
    }

    pub fn query_collections_by_denom_ids(
        &self,
        denom_ids: Vec<String>,
    ) -> StdResult<CollectionsResponse> {
        let request = CudosQuery::QueryCollectionsByDenomIds {
            denom_ids: denom_ids.into(),
        }
        .into();

        self.querier.query(&request)
    }

    pub fn query_supply<T: Into<String>>(&self, denom_id: T) -> StdResult<SupplyResponse> {
        let request = CudosQuery::QuerySupply {
            denom_id: denom_id.into(),
        }
        .into();

        self.querier.query(&request)
    }

    pub fn query_owner<T: Into<Option<String>>, D: Into<String>>(
        &self,
        denom_id: T,
        address: D,
        pagination: Option<PaginationRequest>,
    ) -> StdResult<OwnerCollectionResponse> {
        let request = CudosQuery::QueryOwner {
            denom_id: denom_id.into(),
            address: address.into(),
            pagination: pagination.into(),
        }
        .into();

        self.querier.query(&request)
    }

    pub fn query_token<T: Into<String>>(
        &self,
        denom_id: T,
        token_id: T,
    ) -> StdResult<QueryNFTResponse> {
        let request = CudosQuery::QueryToken {
            denom_id: denom_id.into(),
            token_id: token_id.into(),
        }
        .into();

        self.querier.query(&request)
    }

    pub fn query_approvals<T: Into<String>>(
        &self,
        denom_id: T,
        token_id: T,
    ) -> StdResult<QueryApprovalsResponse> {
        let request = CudosQuery::QueryApprovals {
            denom_id: denom_id.into(),
            token_id: token_id.into(),
        }
        .into();

        self.querier.query(&request)
    }

    pub fn query_approved_for_all<T: Into<String>>(
        &self,
        owner_address: T,
        operator_address: T,
    ) -> StdResult<QueryApprovedForAllResponse> {
        let request = CudosQuery::QueryApprovedForAll {
            owner_address: owner_address.into(),
            operator_address: operator_address.into(),
        }
        .into();

        self.querier.query(&request)
    }

    pub fn query_collection_marketplace(
        &self,
        id: u64,
    ) -> StdResult<QueryCollectionMarketplaceResponse> {
        let request = CudosQuery::QueryCollectionMarketplace { id }.into();

        self.querier.query(&request)
    }

    pub fn query_all_collections(
        &self,
        pagination: Option<PaginationRequest>,
    ) -> StdResult<QueryAllCollectionsResponse> {
        let request = CudosQuery::QueryAllCollections {
            pagination: pagination.into(),
        }
        .into();

        self.querier.query(&request)
    }

    pub fn query_collection_by_denom_id<T: Into<String>>(
        &self,
        denom_id: T,
    ) -> StdResult<QueryCollectionByDenomIdResponse> {
        let request = CudosQuery::QueryCollectionByDenomId {
            denom_id: denom_id.into(),
        }
        .into();

        self.querier.query(&request)
    }

    pub fn query_nft(&self, id: u64) -> StdResult<QueryNftMarketplaceResponse> {
        let request = CudosQuery::QueryNft { id }.into();

        self.querier.query(&request)
    }

    pub fn query_all_nfts(
        &self,
        pagination: Option<PaginationRequest>,
    ) -> StdResult<QueryAllNftsResponse> {
        let request = CudosQuery::QueryAllNfts {
            pagination: pagination.into(),
        }
        .into();

        self.querier.query(&request)
    }

    pub fn query_list_admins(&self) -> StdResult<QueryListAdminsResponse> {
        let request = CudosQuery::QueryListAdmins {}.into();

        self.querier.query(&request)
    }

    //Addressbook

    pub fn query_all_addresses(
        &self,
        pagination: Option<PaginationRequest>,
    ) -> StdResult<QueryAllAdressesResponse> {
        let request = CudosQuery::QueryAllAddresses {
            pagination: pagination.into(),
        }
        .into();

        match self.querier.query(&request) {
            Ok(result) => Ok(result),
            Err(_) => Ok(QueryAllAdressesResponse {
                address: Vec::new(),
                pagination: None,
            }),
        }
    }

    pub fn query_address(
        &self,
        creator: String,
        network: String,
        label: String,
    ) -> StdResult<QueryAdressResponse> {
        let request = CudosQuery::QueryAddress {
            creator,
            network,
            label,
        }
        .into();

        self.querier.query(&request)
    }
}
