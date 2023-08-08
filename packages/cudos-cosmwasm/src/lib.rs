mod msg;
mod querier;
mod query;

pub use msg::{
    create_approve_all_msg, create_approve_nft_msg, create_burn_nft_msg, create_edit_nft_msg,
    create_issue_denom_msg, create_mint_nft_msg, create_revoke_msg, create_transfer_denom_msg,
    create_transfer_nft_msg, CudosMsg, Royalty,
};
pub use querier::CudosQuerier;
pub use query::{
    Collection, CollectionResponse, CollectionsResponse, CudosQuery, Denom, DenomResponse,
    DenomsResponse, IDCollection, MarketplaceCollection, MarketplaceNft, Owner,
    OwnerCollectionResponse, PageResponse, PaginationRequest, QueryAdressResponse, QueryAllAdressesResponse,
    QueryAllCollectionsResponse, QueryAllNftsResponse, QueryApprovalsResponse,
    QueryApprovedForAllResponse, QueryCollectionByDenomIdResponse,
    QueryCollectionMarketplaceResponse, QueryListAdminsResponse, QueryNFTResponse,
    QueryNftMarketplaceResponse, SupplyResponse, NFT,
};

// TODO: Research how to enable "cudos" on the blockchain
// This export is added to all contracts that import this package, signifying that they require
// "cudos" support on the chain they run on.
// #[no_mangle]
// extern "C" fn requires_cudos() {}
