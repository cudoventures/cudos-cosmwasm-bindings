use cosmwasm_std::{
    entry_point, to_binary, Deps, DepsMut, Env, MessageInfo, QueryResponse, Response, StdError,
    StdResult,
};

use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use cudos_cosmwasm::{
    create_approve_all_msg, create_approve_nft_msg, create_burn_nft_msg, create_edit_nft_msg,
    create_issue_denom_msg, create_mint_nft_msg, create_revoke_msg, create_transfer_nft_msg,
    create_transfer_denom_msg,
    CudosMsg, CudosQuerier, DenomResponse, QueryNFTResponse, DenomsResponse, CollectionResponse, SupplyResponse, 
    OwnerCollectionResponse, QueryApprovalsResponse, QueryApprovedForAllResponse,
};

#[entry_point]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response<CudosMsg>> {
    Ok(Response::new())
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
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
        } => execute_msg_issue_denom(deps, env, info, id, name, symbol, schema),
        ExecuteMsg::MintNftMsg {
            denom_id,
            name,
            uri,
            data,
            recipient,
        } => execute_msg_mint_nft(deps, env, info, denom_id, name, uri, data, recipient),
        ExecuteMsg::EditNftMsg {
            denom_id,
            token_id,
            name,
            uri,
            data,
        } => execute_msg_edit_nft(deps, env, info, denom_id, token_id, name, uri, data),
        ExecuteMsg::TransferNftMsg {
            denom_id,
            token_id,
            from,
            to,
        } => execute_msg_transfer_nft(deps, env, info, denom_id, token_id, from, to),
        ExecuteMsg::TransferDenomMsg {
            denom_id,
            to,
        } => execute_msg_transfer_denom(deps, env, info, denom_id, to),
        ExecuteMsg::BurnNftMsg { token_id, denom_id } => {
            execute_msg_burn_nft(deps, env, info, denom_id, token_id)
        }
        ExecuteMsg::ApproveNftMsg {
            denom_id,
            token_id,
            approved_address,
        } => execute_msg_approve_nft(deps, env, info, denom_id, token_id, approved_address),
        ExecuteMsg::ApproveAllMsg {
            approved_operator,
            approved,
        } => execute_msg_approve_all(deps, env, info, approved_operator, approved),
        ExecuteMsg::RevokeApprovalMsg {
            denom_id,
            token_id,
            address_to_revoke,
        } => execute_msg_revoke_nft(deps, env, info, denom_id, token_id, address_to_revoke),
    }
}

pub fn execute_msg_issue_denom(
    _deps: DepsMut,
    env: Env,
    info: MessageInfo,
    id: String,
    name: String,
    symbol: String,
    schema: Option<String>,
) -> StdResult<Response<CudosMsg>> {
    let msg = create_issue_denom_msg(
        id,
        name,
        symbol,
        schema,
        info.sender.to_string(),
        env.contract.address.to_string(),
    );

    Ok(Response::new().add_message(msg))
}

pub fn execute_msg_mint_nft(
    _deps: DepsMut,
    env: Env,
    info: MessageInfo,
    denom_id: String,
    name: Option<String>,
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
    _deps: DepsMut,
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
    _deps: DepsMut,
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
    _deps: DepsMut,
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
    _deps: DepsMut,
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
    _deps: DepsMut,
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
    _deps: DepsMut,
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
    _deps: DepsMut,
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

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<QueryResponse> {
    match msg {
        QueryMsg::QueryDenomById { denom_id } => to_binary(&query_denom_by_id(deps, denom_id)?),
        QueryMsg::QueryDenomByName { denom_name } => to_binary(&query_denom_by_name(deps, denom_name)?),
        QueryMsg::QueryDenomBySymbol {denom_symbol} => to_binary(&query_denom_by_symbol(deps, denom_symbol)?),
        QueryMsg::QueryDenoms {} => to_binary(&query_denoms(deps)?),
        QueryMsg::QueryCollection { denom_id } => to_binary(&query_collection(deps, denom_id)?),
        QueryMsg::QuerySupply { denom_id } => to_binary(&query_supply(deps, denom_id)?),
        QueryMsg::QueryOwner { denom_id , address} => to_binary(&query_owner(deps, denom_id, address)?),
        QueryMsg::QueryToken { denom_id, token_id } => to_binary(&query_token(deps, denom_id, token_id)?),
        QueryMsg::QueryApprovals { denom_id, token_id } => to_binary(&query_approvals(deps, denom_id, token_id)?),
        QueryMsg::QueryApprovedForAll { owner_address, operator_address } => to_binary(&query_approved_for_all(deps, owner_address, operator_address)?)
    }
}

pub fn query_denom_by_id(deps: Deps, denom_id: String) -> StdResult<DenomResponse> {
    let querier = CudosQuerier::new(&deps.querier);
    let res: DenomResponse = querier.query_denom_by_id(denom_id)?;

    Ok(res)
}

pub fn query_denom_by_name(deps: Deps, denom_name: String) -> StdResult<DenomResponse> {
    let querier = CudosQuerier::new(&deps.querier);
    let res: DenomResponse = querier.query_denom_by_name(denom_name)?;

    Ok(res)
}

pub fn query_denom_by_symbol(deps: Deps, denom_symbol: String) -> StdResult<DenomResponse> {
    let querier = CudosQuerier::new(&deps.querier);
    let res: DenomResponse = querier.query_denom_by_symbol(denom_symbol)?;

    Ok(res)
}

pub fn query_denoms(deps: Deps) -> StdResult<DenomsResponse> {
    let querier = CudosQuerier::new(&deps.querier);
    let res: DenomsResponse = querier.query_denoms()?;

    Ok(res)
}

pub fn query_collection(deps: Deps, denom_id: String) -> StdResult<CollectionResponse> {
    let querier = CudosQuerier::new(&deps.querier);
    let res: CollectionResponse = querier.query_collection(denom_id)?;

    Ok(res)
}

pub fn query_supply(deps: Deps, denom_id: String) -> StdResult<SupplyResponse> {
    let querier = CudosQuerier::new(&deps.querier);
    let res: SupplyResponse = querier.query_supply(denom_id)?;

    Ok(res)
}

pub fn query_owner(deps: Deps, denom_id: Option<String>, address: String) -> StdResult<OwnerCollectionResponse> {
    let querier = CudosQuerier::new(&deps.querier);
    let res: OwnerCollectionResponse = querier.query_owner(denom_id, address)?;

    Ok(res)
}

pub fn query_token(deps: Deps, denom_id: String, token_id: String) -> StdResult<QueryNFTResponse> {
    let querier = CudosQuerier::new(&deps.querier);
    let res: QueryNFTResponse = querier.query_token(denom_id, token_id)?;
    Ok(res)
}

pub fn query_approvals(deps: Deps, denom_id: String, token_id: String) -> StdResult<QueryApprovalsResponse> {
    let querier = CudosQuerier::new(&deps.querier);
    let res: QueryApprovalsResponse = querier.query_approvals(denom_id, token_id)?;
    Ok(res)
}

pub fn query_approved_for_all(deps: Deps, owner_address: String, operator_address: String) -> StdResult<QueryApprovedForAllResponse> {
    let querier = CudosQuerier::new(&deps.querier);
    let res: QueryApprovedForAllResponse = querier.query_approved_for_all(owner_address, operator_address)?;
    Ok(res)
}
