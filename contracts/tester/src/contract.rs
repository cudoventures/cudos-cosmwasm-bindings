use cosmwasm_std::{
    entry_point, to_binary, Deps, DepsMut, Env, MessageInfo, QueryResponse, Response, StdError,
    StdResult,
};

use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use cudos_cosmwasm::{
    create_issue_denom_msg, create_mint_nft_msg, CudosMsg, CudosQuerier, DenomResponse,
    QueryNFTResponse,
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
            schema,
            sender,
        } => execute_msg_issue_denom(deps, env, info, id, name, schema, sender),
        ExecuteMsg::MintNftMsg {
            denom_id,
            name,
            uri,
            data,
            sender,
            recipient,
        } => execute_msg_mint_nft(
            deps, env, info, denom_id, name, uri, data, sender, recipient,
        ),
    }
}

pub fn execute_msg_issue_denom(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    id: String,
    name: String,
    schema: String,
    sender: String,
) -> StdResult<Response<CudosMsg>> {
    let msg = create_issue_denom_msg(id, name, schema, sender);

    Ok(Response::new().add_message(msg))
}

pub fn execute_msg_mint_nft(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    denom_id: String,
    name: String,
    uri: String,
    data: String,
    sender: String,
    recipient: String,
) -> StdResult<Response<CudosMsg>> {
    let msg = create_mint_nft_msg(denom_id, name, uri, data, sender, recipient);

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
