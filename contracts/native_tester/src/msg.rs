use cosmwasm_std::{Coin, VoteOption};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct MigrateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct BurnMsg {
    amount: Vec<Coin>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ExecuteBurnMsg {
    burn_msg: BurnMsg,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    SendMsg {
        to_address: String,
        amount: Vec<Coin>,
    },
    BurnMsg {
        amount: Vec<Coin>,
    },
    DelegateMsg {
        validator: String,
        amount: Coin,
    },
    UndelegateMsg {
        validator: String,
        amount: Coin,
    },
    RedelegateMsg {
        src_validator: String,
        dst_validator: String,
        amount: Coin,
    },
    SetWithdrawAddressMsg {
        address: String,
    },
    WithdrawDelegatorRewardMsg {
        validator: String,
    },
    VoteMsg {
        proposal_id: u64,
        vote: VoteOption,
    },
    InstantiateMsg {
        admin: Option<String>,
        code_id: u64,
        funds: Vec<Coin>,
        label: String,
    },
    ExecuteMsg {
        contract_addr: String,
        msg: ExecuteBurnMsg,
        funds: Vec<Coin>,
    },
}
