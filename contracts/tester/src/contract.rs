use cosmwasm_std::{
    entry_point, to_binary, Deps, DepsMut, Env, MessageInfo, QueryResponse, Response, StdError,
    StdResult,
};

use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use cudos_cosmwasm::{
    create_approve_all_msg, create_approve_nft_msg, create_burn_nft_msg, create_edit_nft_msg,
    create_issue_denom_msg, create_mint_nft_msg, create_revoke_msg, create_transfer_nft_msg,
    CudosMsg, CudosQuerier, DenomResponse, QueryNFTResponse,
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
    schema: String,
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
    name: String,
    uri: String,
    data: String,
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
    name: String,
    uri: String,
    data: String,
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
        QueryMsg::QueryDenomByName { denom_name } => {
            to_binary(&query_denom_by_name(deps, denom_name)?)
        }
        QueryMsg::QueryToken { denom_id, token_id } => {
            to_binary(&query_token(deps, denom_id, token_id)?)
        }
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

pub fn query_token(deps: Deps, denom_id: String, token_id: String) -> StdResult<QueryNFTResponse> {
    let querier = CudosQuerier::new(&deps.querier);
    let res: QueryNFTResponse = querier.query_token(denom_id, token_id)?;
    Ok(res)
}
