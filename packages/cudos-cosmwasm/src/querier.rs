use cosmwasm_std::{QuerierWrapper, StdResult};

use crate::query::{
    CollectionResponse, CudosQuery, DenomResponse, DenomsResponse, OwnerCollectionResponse,
    QueryNFTResponse, SupplyResponse, QueryApprovalsResponse, QueryApprovedForAllResponse, PaginationRequest,
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

    pub fn query_denoms<>(&self, pagination: Option<PaginationRequest>) -> StdResult<DenomsResponse> {
        let request = CudosQuery::QueryDenoms {
            pagination: pagination,
        }
        .into();

        self.querier.query(&request)
    }

    pub fn query_collection<T: Into<String>>(&self, denom_id: T, pagination: Option<PaginationRequest>) -> StdResult<CollectionResponse> {
        let request = CudosQuery::QueryCollection {
            denom_id: denom_id.into(),
            pagination: pagination.into(),
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
}
