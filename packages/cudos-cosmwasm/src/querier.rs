use cosmwasm_std::{QuerierWrapper, StdResult};

use crate::query::{CudosQuery, DenomResponse};

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
}
