use cosmwasm_std::{QuerierWrapper, StdResult};

use crate::query::{
    CollectionResponse, CudosQuery, DenomResponse, DenomsResponse, OwnerCollectionResponse,
    QueryNFTResponse, SupplyResponse,
};

pub struct CudosQuerier<'a> {
    querier: &'a QuerierWrapper<'a>,
}

impl<'a> CudosQuerier<'a> {
    pub fn new(querier: &'a QuerierWrapper<'a>) -> Self {
        CudosQuerier { querier }
    }

    pub fn query_denom_by_id<T: Into<String>>(&self, denom_id: T) -> StdResult<DenomResponse> {
        let request = CudosQuery::QueryDenomById {
            denom_id: denom_id.into(),
        }
        .into();

        self.querier.custom_query(&request)
    }

    pub fn query_denom_by_name<T: Into<String>>(&self, denom_name: T) -> StdResult<DenomResponse> {
        let request = CudosQuery::QueryDenomByName {
            denom_name: denom_name.into(),
        }
        .into();

        self.querier.custom_query(&request)
    }

    pub fn query_all_denoms<T: Into<String>>(&self) -> StdResult<DenomsResponse> {
        let request = CudosQuery::QueryDenoms {}.into();
        self.querier.custom_query(&request)
    }

    pub fn query_collection<T: Into<String>>(&self, denom_id: T) -> StdResult<CollectionResponse> {
        let request = CudosQuery::QueryCollection {
            denom_id: denom_id.into(),
        }
        .into();

        self.querier.custom_query(&request)
    }

    pub fn query_supply<T: Into<String>>(&self, denom_id: T) -> StdResult<SupplyResponse> {
        let request = CudosQuery::QuerySupply {
            denom_id: denom_id.into(),
        }
        .into();

        self.querier.custom_query(&request)
    }

    pub fn query_owner<T: Into<String>>(
        &self,
        denom_id: T,
        address: T,
    ) -> StdResult<OwnerCollectionResponse> {
        let request = CudosQuery::QueryCollectionByOwner {
            denom_id: denom_id.into(),
            address: address.into(),
        }
        .into();

        self.querier.custom_query(&request)
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

        self.querier.custom_query(&request)
    }

    pub fn query_approved_addresses<T: Into<String>>(
        &self,
        denom_id: T,
        token_id: T,
    ) -> StdResult<QueryNFTResponse> {
        let request = CudosQuery::QueryApprovals {
            denom_id: denom_id.into(),
            token_id: token_id.into(),
        }
        .into();

        self.querier.custom_query(&request)
    }
}
