use cosmwasm_std::{
    entry_point, to_binary, Deps, DepsMut, Env, MessageInfo,
    QueryResponse, Response, StdError, StdResult, Coin,
};

use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use cudos_cosmwasm::{
    create_approve_all_msg, create_approve_nft_msg, create_burn_nft_msg,
    create_edit_nft_msg, create_issue_denom_msg, create_mint_nft_msg,
    create_revoke_msg, create_transfer_denom_msg, create_transfer_nft_msg,
    CollectionResponse, CollectionsResponse, CudosMsg, CudosQuerier,
    CudosQuery, DenomResponse, DenomsResponse, OwnerCollectionResponse,
    PaginationRequest, QueryAllCollectionsResponse, QueryAllNftsResponse,
    QueryApprovalsResponse, QueryApprovedForAllResponse,
    QueryCollectionByDenomIdResponse, QueryCollectionMarketplaceResponse,
    QueryListAdminsResponse, QueryNFTResponse, QueryNftMarketplaceResponse, SupplyResponse,Royalty
};

#[entry_point]
pub fn instantiate(
    _deps: DepsMut<CudosQuery>,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response<CudosMsg>> {
    Ok(Response::new())
}

#[entry_point]
pub fn execute(
    deps: DepsMut<CudosQuery>,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<CudosMsg>, StdError> {
    match msg {
        ExecuteMsg::IssueDenomMsg {
            id,
            name,
            symbol,
            schema,
            traits,
            minter,
            description,
            data,
        } => execute_msg_issue_denom(
            deps,
            env,
            info,
            id,
            name,
            symbol,
            schema,
            traits,
            minter,
            description,
            data,
        ),
        ExecuteMsg::MintNftMsg {
            denom_id,
            name,
            uri,
            data,
            recipient,
        } => execute_msg_mint_nft(
            deps, env, info, denom_id, name, uri, data, recipient,
        ),
        ExecuteMsg::EditNftMsg {
            denom_id,
            token_id,
            name,
            uri,
            data,
        } => execute_msg_edit_nft(
            deps, env, info, denom_id, token_id, name, uri, data,
        ),
        ExecuteMsg::TransferNftMsg {
            denom_id,
            token_id,
            from,
            to,
        } => execute_msg_transfer_nft(
            deps, env, info, denom_id, token_id, from, to,
        ),
        ExecuteMsg::TransferDenomMsg { denom_id, to } => {
            execute_msg_transfer_denom(deps, env, info, denom_id, to)
        }
        ExecuteMsg::BurnNftMsg { token_id, denom_id } => {
            execute_msg_burn_nft(deps, env, info, denom_id, token_id)
        }
        ExecuteMsg::ApproveNftMsg {
            denom_id,
            token_id,
            approved_address,
        } => execute_msg_approve_nft(
            deps,
            env,
            info,
            denom_id,
            token_id,
            approved_address,
        ),
        ExecuteMsg::ApproveAllMsg {
            approved_operator,
            approved,
        } => execute_msg_approve_all(
            deps,
            env,
            info,
            approved_operator,
            approved,
        ),
        ExecuteMsg::RevokeApprovalMsg {
            denom_id,
            token_id,
            address_to_revoke,
        } => execute_msg_revoke_nft(
            deps,
            env,
            info,
            denom_id,
            token_id,
            address_to_revoke,
        ),
        ExecuteMsg::PublishCollectionMsg {
            denom_id,
            mint_royalties,
            resale_royalties,
        } => execute_msg_publish_collection(
            deps,
            env,
            info,
            denom_id,
            mint_royalties,
            resale_royalties,
        ),
        ExecuteMsg::PublishNftMsg {
            token_id,
            denom_id,
            price,
        } => {
            execute_msg_publish_nft(deps, env, info, token_id, denom_id, price)
        }
        ExecuteMsg::BuyNftMsg { id } => {
            execute_msg_buy_nft(deps, env, info, id)
        }
        ExecuteMsg::MintNftMarketplaceMsg {
            denom_id,
            recipient,
            price,
            name,
            uri,
            data,
            uid,
        } => execute_msg_mint_marketplace_nft(
            deps, env, info, denom_id, recipient, price, name, uri, data, uid,
        ),
        ExecuteMsg::RemoveNftMsg { id } => {
            execute_msg_remove_nft(deps, env, info, id)
        }
        ExecuteMsg::VerifyCollectionMsg { id } => {
            execute_msg_verify_collection(deps, env, info, id)
        }
        ExecuteMsg::UnverifyCollectionMsg { id } => {
            execute_msg_unverify_collection(deps, env, info, id)
        }
        ExecuteMsg::CreateCollectionMsg {
            id,
            name,
            symbol,
            schema,
            traits,
            minter,
            description,
            data,
            mint_royalties,
            resale_royalties,
            verified,
        } => execute_msg_create_collection(
            deps,
            env,
            info,
            id,
            name,
            symbol,
            schema,
            traits,
            minter,
            description,
            data,
            mint_royalties,
            resale_royalties,
            verified,
        ),
        ExecuteMsg::UpdateRoyaltiesMsg {
            id,
            mint_royalties,
            resale_royalties,
        } => execute_msg_update_royalties(
            deps,
            env,
            info,
            id,
            mint_royalties,
            resale_royalties,
        ),
        ExecuteMsg::UpdatePriceMsg { id, price } => {
            execute_msg_update_price(deps, env, info, id, price)
        }
        ExecuteMsg::AddAdminMsg { address } => {
            execute_msg_add_admin(deps, env, info, address)
        }
        ExecuteMsg::RemoveAdminMsg { address } => {
            execute_msg_remove_admin(deps, env, info, address)
        }
    }
}

pub fn execute_msg_issue_denom(
    _deps: DepsMut<CudosQuery>,
    env: Env,
    info: MessageInfo,
    id: String,
    name: String,
    symbol: String,
    schema: Option<String>,
    traits: Option<String>,
    minter: Option<String>,
    description: Option<String>,
    data: Option<String>,
) -> StdResult<Response<CudosMsg>> {
    let msg = create_issue_denom_msg(
        id,
        name,
        symbol,
        schema,
        info.sender.to_string(),
        env.contract.address.to_string(),
        traits,
        minter,
        description,
        data,
    );

    Ok(Response::new().add_message(msg))
}

pub fn execute_msg_mint_nft(
    _deps: DepsMut<CudosQuery>,
    env: Env,
    info: MessageInfo,
    denom_id: String,
    name: String,
    uri: Option<String>,
    data: Option<String>,
    recipient: String,
) -> StdResult<Response<CudosMsg>> {
    let msg = create_mint_nft_msg(
        denom_id,
        name,
        uri,
        data,
        recipient,
        info.sender.to_string(),
        env.contract.address.to_string(),
    );

    Ok(Response::new().add_message(msg))
}

pub fn execute_msg_edit_nft(
    _deps: DepsMut<CudosQuery>,
    env: Env,
    info: MessageInfo,
    denom_id: String,
    token_id: String,
    name: Option<String>,
    uri: Option<String>,
    data: Option<String>,
) -> StdResult<Response<CudosMsg>> {
    let msg = create_edit_nft_msg(
        denom_id,
        token_id,
        name,
        uri,
        data,
        info.sender.to_string(),
        env.contract.address.to_string(),
    );

    Ok(Response::new().add_message(msg))
}

pub fn execute_msg_transfer_nft(
    _deps: DepsMut<CudosQuery>,
    env: Env,
    info: MessageInfo,
    denom_id: String,
    token_id: String,
    from: String,
    to: String,
) -> StdResult<Response<CudosMsg>> {
    let msg = create_transfer_nft_msg(
        denom_id,
        token_id,
        from,
        to,
        info.sender.to_string(),
        env.contract.address.to_string(),
    );

    Ok(Response::new().add_message(msg))
}

pub fn execute_msg_transfer_denom(
    _deps: DepsMut<CudosQuery>,
    env: Env,
    info: MessageInfo,
    denom_id: String,
    to: String,
) -> StdResult<Response<CudosMsg>> {
    let msg = create_transfer_denom_msg(
        denom_id,
        to,
        info.sender.to_string(),
        env.contract.address.to_string(),
    );

    Ok(Response::new().add_message(msg))
}

pub fn execute_msg_burn_nft(
    _deps: DepsMut<CudosQuery>,
    env: Env,
    info: MessageInfo,
    denom_id: String,
    token_id: String,
) -> StdResult<Response<CudosMsg>> {
    let msg = create_burn_nft_msg(
        denom_id,
        token_id,
        info.sender.to_string(),
        env.contract.address.to_string(),
    );

    Ok(Response::new().add_message(msg))
}

pub fn execute_msg_approve_nft(
    _deps: DepsMut<CudosQuery>,
    env: Env,
    info: MessageInfo,
    denom_id: String,
    token_id: String,
    approved_address: String,
) -> StdResult<Response<CudosMsg>> {
    let msg = create_approve_nft_msg(
        denom_id,
        token_id,
        approved_address,
        info.sender.to_string(),
        env.contract.address.to_string(),
    );

    Ok(Response::new().add_message(msg))
}

pub fn execute_msg_approve_all(
    _deps: DepsMut<CudosQuery>,
    env: Env,
    info: MessageInfo,
    approved_operator: String,
    approved: bool,
) -> StdResult<Response<CudosMsg>> {
    let msg = create_approve_all_msg(
        approved_operator,
        approved,
        info.sender.to_string(),
        env.contract.address.to_string(),
    );

    Ok(Response::new().add_message(msg))
}

pub fn execute_msg_revoke_nft(
    _deps: DepsMut<CudosQuery>,
    env: Env,
    info: MessageInfo,
    denom_id: String,
    token_id: String,
    address_to_revoke: String,
) -> StdResult<Response<CudosMsg>> {
    let msg = create_revoke_msg(
        denom_id,
        token_id,
        address_to_revoke,
        info.sender.to_string(),
        env.contract.address.to_string(),
    );

    Ok(Response::new().add_message(msg))
}

fn execute_msg_publish_collection(
    _deps: DepsMut<CudosQuery>,
    _env: Env,
    info: MessageInfo,
    denom_id: String,
    mint_royalties: Vec<Royalty>,
    resale_royalties: Vec<Royalty>,
) -> StdResult<Response<CudosMsg>> {
    let msg = CudosMsg::PublishCollectionMsg {
        creator: info.sender.to_string(),
        denom_id,
        mint_royalties,
        resale_royalties,
    };

    Ok(Response::new().add_message(msg))
}

fn execute_msg_publish_nft(
    _deps: DepsMut<CudosQuery>,
    _env: Env,
    info: MessageInfo,
    token_id: String,
    denom_id: String,
    price: Coin,
) -> StdResult<Response<CudosMsg>> {
    let msg = CudosMsg::PublishNftMsg {
        creator: info.sender.to_string(),
        token_id,
        denom_id,
        price,
    };

    Ok(Response::new().add_message(msg))
}

fn execute_msg_buy_nft(
    _deps: DepsMut<CudosQuery>,
    _env: Env,
    info: MessageInfo,
    id: u64,
) -> StdResult<Response<CudosMsg>> {
    let msg = CudosMsg::BuyNftMsg {
        creator: info.sender.to_string(),
        id,
    };

    Ok(Response::new().add_message(msg))
}

fn execute_msg_mint_marketplace_nft(
    _deps: DepsMut<CudosQuery>,
    _env: Env,
    info: MessageInfo,
    denom_id: String,
    recipient: String,
    price: Coin,
    name: String,
    uri: Option<String>,
    data: Option<String>,
    uid: String,
) -> StdResult<Response<CudosMsg>> {
    let msg = CudosMsg::MintNftMarketplaceMsg {
        creator: info.sender.to_string(),
        denom_id,
        recipient,
        price,
        name,
        uri,
        data,
        uid,
    };

    Ok(Response::new().add_message(msg))
}

fn execute_msg_remove_nft(
    _deps: DepsMut<CudosQuery>,
    _env: Env,
    info: MessageInfo,
    id: u64,
) -> StdResult<Response<CudosMsg>> {
    let msg = CudosMsg::RemoveNftMsg {
        creator: info.sender.to_string(),
        id,
    };

    Ok(Response::new().add_message(msg))
}

fn execute_msg_verify_collection(
    _deps: DepsMut<CudosQuery>,
    _env: Env,
    info: MessageInfo,
    id: u64,
) -> StdResult<Response<CudosMsg>> {
    let msg = CudosMsg::VerifyCollectionMsg {
        creator: info.sender.to_string(),
        id,
    };

    Ok(Response::new().add_message(msg))
}

fn execute_msg_unverify_collection(
    _deps: DepsMut<CudosQuery>,
    _env: Env,
    info: MessageInfo,
    id: u64,
) -> StdResult<Response<CudosMsg>> {
    let msg = CudosMsg::UnverifyCollectionMsg {
        creator: info.sender.to_string(),
        id,
    };

    Ok(Response::new().add_message(msg))
}

fn execute_msg_create_collection(
    _deps: DepsMut<CudosQuery>,
    _env: Env,
    info: MessageInfo,
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
) -> StdResult<Response<CudosMsg>> {
    let msg = CudosMsg::CreateCollectionMsg {
        creator: info.sender.to_string(),
        id,
        name,
        symbol,
        schema,
        traits,
        minter,
        description,
        data,
        mint_royalties,
        resale_royalties,
        verified,
    };

    Ok(Response::new().add_message(msg))
}

fn execute_msg_update_royalties(
    _deps: DepsMut<CudosQuery>,
    _env: Env,
    info: MessageInfo,
    id: u64,
    mint_royalties: Vec<Royalty>,
    resale_royalties: Vec<Royalty>,
) -> StdResult<Response<CudosMsg>> {
    let msg = CudosMsg::UpdateRoyaltiesMsg {
        creator: info.sender.to_string(),
        id,
        mint_royalties,
        resale_royalties,
    };

    Ok(Response::new().add_message(msg))
}

fn execute_msg_update_price(
    _deps: DepsMut<CudosQuery>,
    _env: Env,
    info: MessageInfo,
    id: u64,
    price: Coin,
) -> StdResult<Response<CudosMsg>> {
    let msg = CudosMsg::UpdatePriceMsg {
        creator: info.sender.to_string(),
        id,
        price,
    };

    Ok(Response::new().add_message(msg))
}

fn execute_msg_add_admin(
    _deps: DepsMut<CudosQuery>,
    _env: Env,
    info: MessageInfo,
    address: String,
) -> StdResult<Response<CudosMsg>> {
    let msg = CudosMsg::AddAdminMsg {
        creator: info.sender.to_string(),
        address,
    };

    Ok(Response::new().add_message(msg))
}

fn execute_msg_remove_admin(
    _deps: DepsMut<CudosQuery>,
    _env: Env,
    info: MessageInfo,
    address: String,
) -> StdResult<Response<CudosMsg>> {
    let msg = CudosMsg::RemoveAdminMsg {
        creator: info.sender.to_string(),
        address,
    };

    Ok(Response::new().add_message(msg))
}

#[entry_point]
pub fn query(
    deps: Deps<CudosQuery>,
    _env: Env,
    msg: QueryMsg,
) -> StdResult<QueryResponse> {
    match msg {
        QueryMsg::QueryDenomById { denom_id } => {
            to_binary(&query_denom_by_id(deps, denom_id)?)
        }
        QueryMsg::QueryDenomByName { denom_name } => {
            to_binary(&query_denom_by_name(deps, denom_name)?)
        }
        QueryMsg::QueryDenomBySymbol { denom_symbol } => {
            to_binary(&query_denom_by_symbol(deps, denom_symbol)?)
        }
        QueryMsg::QueryDenoms { pagination } => {
            to_binary(&query_denoms(deps, pagination)?)
        }
        QueryMsg::QueryCollection {
            denom_id,
            pagination,
        } => to_binary(&query_collection(deps, denom_id, pagination)?),
        QueryMsg::QueryCollectionsByDenomIds { denom_ids } => {
            to_binary(&query_collections_by_denom_ids(deps, denom_ids)?)
        }
        QueryMsg::QuerySupply { denom_id } => {
            to_binary(&query_supply(deps, denom_id)?)
        }
        QueryMsg::QueryOwner {
            denom_id,
            address,
            pagination,
        } => to_binary(&query_owner(deps, denom_id, address, pagination)?),
        QueryMsg::QueryToken { denom_id, token_id } => {
            to_binary(&query_token(deps, denom_id, token_id)?)
        }
        QueryMsg::QueryApprovals { denom_id, token_id } => {
            to_binary(&query_approvals(deps, denom_id, token_id)?)
        }
        QueryMsg::QueryApprovedForAll {
            owner_address,
            operator_address,
        } => to_binary(&query_approved_for_all(
            deps,
            owner_address,
            operator_address,
        )?),
        QueryMsg::QueryCollectionMarketplace { id } => {
            to_binary(&query_collection_marketplace(deps, id)?)
        }
        QueryMsg::QueryAllCollections { pagination } => {
            to_binary(&query_all_collections(deps, pagination)?)
        }
        QueryMsg::QueryCollectionByDenomId { denom_id } => {
            to_binary(&query_collection_by_denom_id(deps, denom_id)?)
        }
        QueryMsg::QueryNft { id } => to_binary(&query_nft(deps, id)?),
        QueryMsg::QueryAllNfts { pagination } => {
            to_binary(&query_all_nfts(deps, pagination)?)
        }
        QueryMsg::QueryListAdmins {} => to_binary(&query_list_admins(deps)?),
    }
}

pub fn query_denom_by_id(
    deps: Deps<CudosQuery>,
    denom_id: String,
) -> StdResult<DenomResponse> {
    let querier = CudosQuerier::new(&deps.querier);
    let res: DenomResponse = querier.query_denom_by_id(denom_id)?;

    Ok(res)
}

pub fn query_denom_by_name(
    deps: Deps<CudosQuery>,
    denom_name: String,
) -> StdResult<DenomResponse> {
    let querier = CudosQuerier::new(&deps.querier);
    let res: DenomResponse = querier.query_denom_by_name(denom_name)?;

    Ok(res)
}

pub fn query_denom_by_symbol(
    deps: Deps<CudosQuery>,
    denom_symbol: String,
) -> StdResult<DenomResponse> {
    let querier = CudosQuerier::new(&deps.querier);
    let res: DenomResponse = querier.query_denom_by_symbol(denom_symbol)?;

    Ok(res)
}

pub fn query_denoms(
    deps: Deps<CudosQuery>,
    pagination: Option<PaginationRequest>,
) -> StdResult<DenomsResponse> {
    let querier = CudosQuerier::new(&deps.querier);
    let res: DenomsResponse = querier.query_denoms(pagination)?;

    Ok(res)
}

pub fn query_collection(
    deps: Deps<CudosQuery>,
    denom_id: String,
    pagination: Option<PaginationRequest>,
) -> StdResult<CollectionResponse> {
    let querier = CudosQuerier::new(&deps.querier);
    let res: CollectionResponse =
        querier.query_collection(denom_id, pagination)?;

    Ok(res)
}

pub fn query_collections_by_denom_ids(
    deps: Deps<CudosQuery>,
    denom_ids: Vec<String>,
) -> StdResult<CollectionsResponse> {
    let querier = CudosQuerier::new(&deps.querier);
    let res: CollectionsResponse =
        querier.query_collections_by_denom_ids(denom_ids)?;

    Ok(res)
}

pub fn query_supply(
    deps: Deps<CudosQuery>,
    denom_id: String,
) -> StdResult<SupplyResponse> {
    let querier = CudosQuerier::new(&deps.querier);
    let res: SupplyResponse = querier.query_supply(denom_id)?;

    Ok(res)
}

pub fn query_owner(
    deps: Deps<CudosQuery>,
    denom_id: Option<String>,
    address: String,
    pagination: Option<PaginationRequest>,
) -> StdResult<OwnerCollectionResponse> {
    let querier = CudosQuerier::new(&deps.querier);
    let res: OwnerCollectionResponse =
        querier.query_owner(denom_id, address, pagination)?;

    Ok(res)
}

pub fn query_token(
    deps: Deps<CudosQuery>,
    denom_id: String,
    token_id: String,
) -> StdResult<QueryNFTResponse> {
    let querier = CudosQuerier::new(&deps.querier);
    let res: QueryNFTResponse = querier.query_token(denom_id, token_id)?;
    Ok(res)
}

pub fn query_approvals(
    deps: Deps<CudosQuery>,
    denom_id: String,
    token_id: String,
) -> StdResult<QueryApprovalsResponse> {
    let querier = CudosQuerier::new(&deps.querier);
    let res: QueryApprovalsResponse =
        querier.query_approvals(denom_id, token_id)?;
    Ok(res)
}

pub fn query_approved_for_all(
    deps: Deps<CudosQuery>,
    owner_address: String,
    operator_address: String,
) -> StdResult<QueryApprovedForAllResponse> {
    let querier = CudosQuerier::new(&deps.querier);
    let res: QueryApprovedForAllResponse =
        querier.query_approved_for_all(owner_address, operator_address)?;
    Ok(res)
}

pub fn query_collection_marketplace(
    deps: Deps<CudosQuery>,
    id: u64,
) -> StdResult<QueryCollectionMarketplaceResponse> {
    let querier = CudosQuerier::new(&deps.querier);
    let res: QueryCollectionMarketplaceResponse =
        querier.query_collection_marketplace(id)?;
    Ok(res)
}

pub fn query_all_collections(
    deps: Deps<CudosQuery>,
    pagination: Option<PaginationRequest>,
) -> StdResult<QueryAllCollectionsResponse> {
    let querier = CudosQuerier::new(&deps.querier);
    let res: QueryAllCollectionsResponse =
        querier.query_all_collections(pagination)?;
    Ok(res)
}

pub fn query_collection_by_denom_id(
    deps: Deps<CudosQuery>,
    denom_id: String,
) -> StdResult<QueryCollectionByDenomIdResponse> {
    let querier = CudosQuerier::new(&deps.querier);
    let res: QueryCollectionByDenomIdResponse =
        querier.query_collection_by_denom_id(denom_id)?;
    Ok(res)
}

pub fn query_nft(
    deps: Deps<CudosQuery>,
    id: u64,
) -> StdResult<QueryNftMarketplaceResponse> {
    let querier = CudosQuerier::new(&deps.querier);
    let res: QueryNftMarketplaceResponse = querier.query_nft(id)?;
    Ok(res)
}

pub fn query_all_nfts(
    deps: Deps<CudosQuery>,
    pagination: Option<PaginationRequest>,
) -> StdResult<QueryAllNftsResponse> {
    let querier = CudosQuerier::new(&deps.querier);
    let res: QueryAllNftsResponse = querier.query_all_nfts(pagination)?;
    Ok(res)
}

pub fn query_list_admins(
    deps: Deps<CudosQuery>,
) -> StdResult<QueryListAdminsResponse> {
    let querier = CudosQuerier::new(&deps.querier);
    let res: QueryListAdminsResponse = querier.query_list_admins()?;
    Ok(res)
}
