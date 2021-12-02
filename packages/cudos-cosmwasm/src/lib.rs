mod msg;
mod querier;
mod query;

pub use msg::{create_edit_nft_msg, create_issue_denom_msg, create_mint_nft_msg, CudosMsg};
pub use querier::CudosQuerier;
pub use query::{
    Collection, CollectionResponse, CudosQuery, Denom, DenomResponse, DenomsResponse, IDCollection,
    Owner, OwnerCollectionResponse, PageResponse, QueryApprovalsResponse,
    QueryApprovedForAllResponse, QueryNFTResponse, SupplyResponse, NFT,
};

// TODO: Research how to enable "cudos" on the blockchain
// This export is added to all contracts that import this package, signifying that they require
// "cudos" support on the chain they run on.
// #[no_mangle]
// extern "C" fn requires_cudos() {}
