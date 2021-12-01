use cosmwasm_std::{QuerierWrapper, StdResult};

use crate::query::{CudosQuery, CudosQueryWrapper, DenomResponse};

/// This is a helper wrapper to easily use our custom queries
pub struct CudosQuerier<'a> {
    querier: &'a QuerierWrapper<'a>,
}

impl<'a> CudosQuerier<'a> {
    pub fn new(querier: &'a QuerierWrapper<'a>) -> Self {
        CudosQuerier { querier }
    }

    pub fn query_denom<T: Into<String>>(&self, denom_id: T) -> StdResult<DenomResponse> {
        let request = CudosQueryWrapper {
            query_data: CudosQuery::Denom {
                denom_id: denom_id.into(),
            },
        }
        .into();

        self.querier.custom_query(&request)
    }
}
