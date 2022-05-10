use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{
    entry_point, to_binary, Deps, DepsMut, Env, MessageInfo, QueryResponse, Response, StdError,
    StdResult, CosmosMsg, Coin, BankMsg, StakingMsg, DistributionMsg, GovMsg, BalanceResponse, 
    AllBalanceResponse, VoteOption, BondedDenomResponse, AllDelegationsResponse, FullDelegation,
    AllValidatorsResponse, ValidatorResponse, wasm_instantiate, wasm_execute,
};

use crate::msg::{ExecuteMsg, InstantiateMsg, ExecuteBurnMsg};
use crate::query::{QueryMsg};

#[entry_point]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    Ok(Response::new())
}

#[entry_point]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, StdError> {
    match msg {
        ExecuteMsg::SendMsg {
            to_address,
            amount,
        } => execute_msg_send(to_address, amount),
        ExecuteMsg::BurnMsg {
            amount,
        } => execute_msg_burn(amount),
        ExecuteMsg::DelegateMsg {
            validator,
            amount,
        } => execute_msg_delegate(validator, amount),
        ExecuteMsg::UndelegateMsg {
            validator, 
            amount, 
        } => execute_msg_undelegate(validator, amount),
        ExecuteMsg::RedelegateMsg {
            src_validator,
            dst_validator,
            amount,
        } => execute_msg_redelegate(src_validator, dst_validator, amount),
        ExecuteMsg::SetWithdrawAddressMsg {
            address
        } => execute_msg_set_withdraw_address(address),
        ExecuteMsg::WithdrawDelegatorRewardMsg {
            validator
        } => execute_msg_withdraw_delegator_reward(validator),
        ExecuteMsg::VoteMsg {
            proposal_id, 
            vote 
        } => execute_msg_vote(proposal_id, vote),
        ExecuteMsg::InstantiateMsg {
            admin,
            code_id,
            funds,
            label,   
        } => execute_msg_instantiate(admin, code_id, funds, label),
        ExecuteMsg::ExecuteMsg {
            contract_addr,
            msg,
            funds,
        } => execute_msg_execute(contract_addr, msg, funds),
    }
}

pub fn execute_msg_send(
    to_address: String,
    amount: Vec<Coin>,
) -> StdResult<Response> {
    let send = BankMsg::Send { to_address, amount };
    let msg: CosmosMsg = send.clone().into();
    Ok(Response::new().add_message(msg))
}

pub fn execute_msg_burn(
    amount: Vec<Coin>,
) -> StdResult<Response> {
    let burn = BankMsg::Burn { amount };
    let msg: CosmosMsg = burn.clone().into();
    Ok(Response::new().add_message(msg))
}

pub fn execute_msg_delegate(
    validator: String,
    amount: Coin,
) -> StdResult<Response> {
    let delegate = StakingMsg::Delegate { validator, amount };
    let msg: CosmosMsg = delegate.clone().into();
    Ok(Response::new().add_message(msg))
}

pub fn execute_msg_undelegate(
    validator: String,
    amount: Coin,
) -> StdResult<Response> {
    let undelegate = StakingMsg::Undelegate { validator, amount };
    let msg: CosmosMsg = undelegate.clone().into();
    Ok(Response::new().add_message(msg))
}

pub fn execute_msg_redelegate(
    src_validator: String,
    dst_validator: String,
    amount: Coin,
) -> StdResult<Response> {
    let redelegate = StakingMsg::Redelegate { src_validator, dst_validator, amount };
    let msg: CosmosMsg = redelegate.clone().into();
    Ok(Response::new().add_message(msg))
}

pub fn execute_msg_set_withdraw_address(
    address: String
) -> StdResult<Response> {
    let set_withdraw_address = DistributionMsg::SetWithdrawAddress { address };
    let msg: CosmosMsg = set_withdraw_address.clone().into();
    Ok(Response::new().add_message(msg))
}

pub fn execute_msg_withdraw_delegator_reward(
    validator: String
) -> StdResult<Response> {
    let withdraw_reward = DistributionMsg::WithdrawDelegatorReward { validator };
    let msg: CosmosMsg = withdraw_reward.clone().into();
    Ok(Response::new().add_message(msg))
}

pub fn execute_msg_vote(
    proposal_id: u64,
    vote: VoteOption
) -> StdResult<Response> {
    let vote = GovMsg::Vote { proposal_id, vote };
    let msg: CosmosMsg = vote.clone().into();
    Ok(Response::new().add_message(msg))
}

pub fn execute_msg_instantiate(
    _admin: Option<String>,
    code_id: u64,
    funds: Vec<Coin>,
    label: String,
) -> StdResult<Response> {
    let instantiate_msg = InstantiateMsg{};
    let instantiate = wasm_instantiate(code_id, &instantiate_msg, funds, label)?;
    let msg: CosmosMsg = instantiate.clone().into();
    Ok(Response::new().add_message(msg))
}

pub fn execute_msg_execute(
    contract_addr: String,
    msg: ExecuteBurnMsg,
    funds: Vec<Coin>,
) -> StdResult<Response> {
    let execute = wasm_execute(contract_addr, &msg, funds)?;
    let msg: CosmosMsg = execute.clone().into();
    Ok(Response::new().add_message(msg))
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<QueryResponse> {
    match msg {
        QueryMsg::Balance { address, denom, } => to_binary(&query_balance(deps, address, denom)?),
        QueryMsg::AllBalances { address } => to_binary(&query_all_balances(deps, address)?),
        QueryMsg::BondedDenom { } => to_binary(&query_bonded_denom(deps)?),
        QueryMsg::AllDelegations { delegator } => to_binary(&query_all_delegations(deps, delegator)?),
        QueryMsg::Delegation { delegator, validator } => to_binary(&query_delegation(deps, delegator, validator)?),
        QueryMsg::AllValidators { } => to_binary(&query_all_validators(deps)?),
        QueryMsg::Validator { address } => to_binary(&query_validator(deps, address)?),
    }
}

pub fn query_validator(
    deps: Deps,
    address: String,
) -> StdResult<ValidatorResponse> {
    let validator = deps.querier.query_validator(address)?;
    Ok(ValidatorResponse{validator})
}

pub fn query_balance(
    deps: Deps,
    address: String,
    denom: String,
) -> StdResult<BalanceResponse> {
    let amount = deps.querier.query_balance(address, denom)?;
    Ok(BalanceResponse{amount})
}

pub fn query_all_balances(
    deps: Deps,
    address: String,
) -> StdResult<AllBalanceResponse> {
    let amount = deps.querier.query_all_balances(address)?;
    Ok(AllBalanceResponse{amount})
}

pub fn query_bonded_denom(
    deps: Deps
) -> StdResult<BondedDenomResponse> {
    let denom = deps.querier.query_bonded_denom()?;
    Ok(BondedDenomResponse{denom})
}

pub fn query_all_delegations(
    deps: Deps,
    delegator: String
) -> StdResult<AllDelegationsResponse> {
    let delegations = deps.querier.query_all_delegations(delegator)?;
    Ok(AllDelegationsResponse{delegations})
}

pub fn query_delegation(
    deps: Deps,
    delegator: String,
    validator: String,
) -> StdResult<DelegationResponse> {
    let delegation = deps.querier.query_delegation(delegator, validator)?;
    Ok(DelegationResponse{delegation})
}

pub fn query_all_validators(
    deps: Deps
) -> StdResult<AllValidatorsResponse> {
    let validators = deps.querier.query_all_validators()?;
    Ok(AllValidatorsResponse{validators})
}

// Copied here because its not exported from cosmwasm-std - https://github.com/CosmWasm/cosmwasm/issues/1300
/// DelegationResponse is data format returned from StakingRequest::Delegation query
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct DelegationResponse {
    pub delegation: Option<FullDelegation>,
}
