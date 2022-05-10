CHAIN_ID=$1 # cudos-network
CONTRACT_PATH=$2
CUDOS_NODED_RUNNING_INSTANCE_PATH=$4

alias CUDOS_NODED='cudos-noded'

CONTRACT=''
FEE_FLAGS='--gas auto --gas-adjustment 1.3 --gas-prices 5000000000000acudos'

exec_msg() {
    RES=$(CUDOS_NODED tx wasm execute $CONTRACT $1 --from $2 $FEE_FLAGS --keyring-backend test --chain-id="$CHAIN_ID" -y)
    echo "$RES"
}

trim_json() {
    data=$(echo "$1" | tr -d '[:blank:]' | tr -d '\r\n')
    echo "$data"
}

read_json() {
    data=`cat $1`
    data=$(trim_json "$data")
    echo "$data"
}

TEST_DIR=$PWD

cd $CUDOS_NODED_RUNNING_INSTANCE_PATH

# CREATE USERS
echo "CREATE USERS"
echo yes | CUDOS_NODED keys add alice --keyring-backend test
echo yes | CUDOS_NODED keys add bob --keyring-backend test

aliceAddress=$(CUDOS_NODED keys show -a alice --keyring-backend test)
bobAddress=$(CUDOS_NODED keys show -a bob --keyring-backend test)

# ADD FUNDS
echo "ADD FUNDS"
RES=$(CUDOS_NODED tx bank send faucet "$aliceAddress" 1000000000000000000000000acudos --keyring-backend test --chain-id="$CHAIN_ID" $FEE_FLAGS -y)
RES=$(CUDOS_NODED tx bank send faucet "$bobAddress" 1000000000000000000000000acudos --keyring-backend test --chain-id="$CHAIN_ID" $FEE_FLAGS -y)

# STORE CODE
echo "STORE CODE"
RES=$(CUDOS_NODED tx wasm store "$CONTRACT_PATH" --from alice $FEE_FLAGS --keyring-backend test --chain-id=$CHAIN_ID -y)
CODE_ID=$(echo $RES | jq -r '.logs[0].events[-1].attributes[0].value')
echo "Deployed code id: $CODE_ID"

# INSTANTIATE CONTRACT
echo "INSTANTIATE CONTRACT"
RES=$(CUDOS_NODED tx wasm instantiate $CODE_ID "{}" --label "test bindings" --no-admin --from alice $FEE_FLAGS --keyring-backend test --chain-id="$CHAIN_ID" -y)

# INSTANTIATION
echo "INSTANTIATION"
CUDOS_NODED query wasm list-contract-by-code "$CODE_ID"
CONTRACT=$(CUDOS_NODED query wasm list-contract-by-code "$CODE_ID" --output json | jq -r '.contracts[0]')
echo "Contract: $CONTRACT"
if [ "$CONTRACT" == "" ];then
    printf '%s\n' "Failed to init the smart contract on the chain. Check the tx result above for a detailed error info" >&2 # write error message to stderr
    printf '%s\n' "Failed INIT Transaction: ""$RES"" " >&2 # write error message to stderr

    exit 1
fi

# ADD FUNDS TO CONTRACT
echo "ADD FUNDS TO CONTRACT"
RES=$(CUDOS_NODED tx bank send faucet "$CONTRACT" 100000000000000000000acudos --keyring-backend test --chain-id="$CHAIN_ID" $FEE_FLAGS -y)

# BANK SEND FROM CONTRACT
echo "BANK SEND"
SEND_MSG=$(jq '.send_msg.to_address = "'$bobAddress'"' $TEST_DIR'/integration_tests/native_msgs/send_msg.json')
SEND_MSG=$(trim_json "$SEND_MSG")
RES=$(exec_msg $SEND_MSG "alice")
RES_CODE=$(echo $RES | jq -r '.code')

if [[ $RES_CODE != 0 ]];then
    printf '%s\n' "Bank send failed: ""$RES"" " >&2
    exit 1
fi

# BANK BURN CONTRACT BALANCE
echo "BANK BURN"
ISSUE_DENOM_MSG=$(read_json "$TEST_DIR"'/integration_tests/native_msgs/burn_msg.json')
RES=$(exec_msg $ISSUE_DENOM_MSG "alice")
RES_CODE=$(echo $RES | jq -r '.code')

if [[ $RES_CODE != 0 ]];then
    printf '%s\n' "Bank burn failed: ""$RES"" " >&2
    exit 1
fi

# QUERY BALANCE
echo "QUERY BALANCE"
QUERY_BALANCE=$(read_json $TEST_DIR'/integration_tests/native_msgs/query_balance.json')
QUERY_BALANCE=$(echo $QUERY_BALANCE | jq '.balance.address = "'$CONTRACT'"')
QUERY_BALANCE=$(trim_json "$QUERY_BALANCE")
QUERY_RES=$(CUDOS_NODED query wasm contract-state smart $CONTRACT $QUERY_BALANCE --output json)
BALANCE=$(echo $QUERY_RES | jq -r '.data.amount.amount')

if [[ $BALANCE != 0 ]];then
    printf '%s\n' "Query balance failed: ""$QUERY_RES"" " >&2
    exit 1
fi

# ADD FUNDS TO CONTRACT
echo "ADD FUNDS TO CONTRACT"
RES=$(CUDOS_NODED tx bank send faucet "$CONTRACT" 100000000000000000000acudos --keyring-backend test --chain-id="$CHAIN_ID" $FEE_FLAGS -y)

# QUERY ALL BALANCES
echo "QUERY ALL BALANCES"
QUERY_ALL_BALANCE=$(read_json $TEST_DIR'/integration_tests/native_msgs/query_all_balances.json')
QUERY_ALL_BALANCE=$(echo $QUERY_ALL_BALANCE | jq '.all_balances.address = "'$CONTRACT'"')
QUERY_ALL_BALANCE=$(trim_json "$QUERY_ALL_BALANCE")
QUERY_RES=$(CUDOS_NODED query wasm contract-state smart $CONTRACT $QUERY_ALL_BALANCE --output json)
BALANCE=$(echo $QUERY_RES | jq -r '.data.amount[0].amount')

if [[ $BALANCE != 100000000000000000000 ]];then
    printf '%s\n' "Query all balance failed: ""$QUERY_RES"" " >&2
    exit 1
fi

# QUERY BONDED DENOM
echo "QUERY BONDED DENOM"
QUERY_BONDED_DENOM=$(read_json $TEST_DIR'/integration_tests/native_msgs/query_bonded_denom.json')
QUERY_RES=$(CUDOS_NODED query wasm contract-state smart $CONTRACT $QUERY_BONDED_DENOM --output json)
DENOM=$(echo $QUERY_RES | jq -r '.data.denom')

if [ $DENOM != 'acudos' ];then
    printf '%s\n' "Query bonded denom failed: ""$QUERY_RES"" " >&2
    exit 1
fi

# GET VALIDATOR
QUERY_RES=$(CUDOS_NODED query staking validators --output json)
VALIDATOR_ADDR=$(echo $QUERY_RES | jq -r '.validators[0].operator_address')

# DELEGATE
echo "DELEGATE"
DELEGATE_MSG=$(jq '.delegate_msg.validator = "'$VALIDATOR_ADDR'"' $TEST_DIR'/integration_tests/native_msgs/delegate_msg.json')
DELEGATE_MSG=$(trim_json "$DELEGATE_MSG")
QUERY_RES=$(exec_msg $DELEGATE_MSG "alice")
RES_CODE=$(echo $RES | jq -r '.code')

if [[ $RES_CODE != 0 ]];then
    printf '%s\n' "Delegate failed: ""$QUERY_RES"" " >&2
    exit 1
fi

# QUERY ALL DELEGATIONS
echo "QUERY ALL DELEGATIONS"
QUERY_ALL_DELEGATIONS=$(read_json $TEST_DIR'/integration_tests/native_msgs/query_all_delegations.json')
QUERY_ALL_DELEGATIONS=$(echo $QUERY_ALL_DELEGATIONS | jq '.all_delegations.delegator = "'$CONTRACT'"')
QUERY_ALL_DELEGATIONS=$(trim_json "$QUERY_ALL_DELEGATIONS")
QUERY_RES=$(CUDOS_NODED query wasm contract-state smart $CONTRACT $QUERY_ALL_DELEGATIONS --output json)
BALANCE=$(echo $QUERY_RES | jq -r '.data.delegations[0].amount.amount')

if [[ $BALANCE != 1000 ]];then
    printf '%s\n' "Query all delegations failed: ""$QUERY_RES"" " >&2
    exit 1
fi

# UNDELEGATE
echo "UNDELEGATE"
UNDELEGATE_MSG=$(jq '.undelegate_msg.validator = "'$VALIDATOR_ADDR'"' $TEST_DIR'/integration_tests/native_msgs/undelegate_msg.json')
UNDELEGATE_MSG=$(trim_json "$UNDELEGATE_MSG")
RES=$(exec_msg $UNDELEGATE_MSG "alice")
RES_CODE=$(echo $RES | jq -r '.code')

if [[ $RES_CODE != 0 ]];then
    printf '%s\n' "Undelegate failed: ""$RES"" " >&2
    exit 1
fi

# QUERY DELEGATION
echo "QUERY DELEGATION"
QUERY_DELEGATION=$(read_json $TEST_DIR'/integration_tests/native_msgs/query_delegation.json')
QUERY_DELEGATION=$(echo $QUERY_DELEGATION | jq '.delegation.delegator = "'$CONTRACT'"')
QUERY_DELEGATION=$(echo $QUERY_DELEGATION | jq '.delegation.validator = "'$VALIDATOR_ADDR'"')
QUERY_DELEGATION=$(trim_json "$QUERY_DELEGATION")
QUERY_RES=$(CUDOS_NODED query wasm contract-state smart $CONTRACT $QUERY_DELEGATION --output json)
BALANCE=$(echo $QUERY_RES | jq -r '.data.delegation.amount.amount')

if [[ $BALANCE != 500 ]];then
    printf '%s\n' "Query delegation failed: ""$QUERY_RES"" " >&2
    exit 1
fi

# QUERY ALL VALIDATORS
echo "QUERY ALL VALIDATORS"
QUERY_ALL_VALIDATORS=$(read_json $TEST_DIR'/integration_tests/native_msgs/query_all_validators.json')
QUERY_RES=$(CUDOS_NODED query wasm contract-state smart $CONTRACT $QUERY_ALL_VALIDATORS --output json)
ADDR=$(echo $QUERY_RES | jq -r '.data.validators[0].address')

if [ $ADDR != $VALIDATOR_ADDR ];then
    printf '%s\n' "Query all validators failed: ""$QUERY_RES"" " >&2
    exit 1
fi

# QUERY VALIDATOR
echo "QUERY VALIDATOR"
QUERY_VALIDATOR=$(read_json $TEST_DIR'/integration_tests/native_msgs/query_validator.json')
QUERY_VALIDATOR=$(echo $QUERY_VALIDATOR | jq '.validator.address = "'$VALIDATOR_ADDR'"')
QUERY_VALIDATOR=$(trim_json "$QUERY_VALIDATOR")
QUERY_RES=$(CUDOS_NODED query wasm contract-state smart $CONTRACT $QUERY_VALIDATOR --output json)
ADDR=$(echo $QUERY_RES | jq -r '.data.validator.address')

if [ $ADDR != $VALIDATOR_ADDR ];then
    printf '%s\n' "Query all validators failed: ""$QUERY_RES"" " >&2
    exit 1
fi

# SET WITHDRAW ADDRESS
echo "SET WITHDRAW ADDRESS"
SET_WITHDRAW_ADDRESS=$(read_json $TEST_DIR'/integration_tests/native_msgs/set_withdraw_address.json')
SET_WITHDRAW_ADDRESS=$(echo $SET_WITHDRAW_ADDRESS | jq '.set_withdraw_address_msg.address = "'$aliceAddress'"')
SET_WITHDRAW_ADDRESS=$(trim_json "$SET_WITHDRAW_ADDRESS")
RES=$(exec_msg $SET_WITHDRAW_ADDRESS "alice")
RES_CODE=$(echo $RES | jq -r '.code')

if [[ $RES_CODE != 0 ]];then
    printf '%s\n' "Set withdraw address failed: ""$RES"" " >&2
    exit 1
fi

# WITHDRAW DELEGATOR REWARD
echo "WITHDRAW DELEGATOR REWARD"
WITHDRAW_DELEGATOR_REWARD=$(read_json $TEST_DIR'/integration_tests/native_msgs/withdraw_delegator_reward.json')
WITHDRAW_DELEGATOR_REWARD=$(echo $WITHDRAW_DELEGATOR_REWARD | jq '.withdraw_delegator_reward_msg.validator = "'$VALIDATOR_ADDR'"')
WITHDRAW_DELEGATOR_REWARD=$(trim_json "$WITHDRAW_DELEGATOR_REWARD")
RES=$(exec_msg $WITHDRAW_DELEGATOR_REWARD "alice")
RES_CODE=$(echo $RES | jq -r '.code')

if [[ $RES_CODE != 0 ]];then
    printf '%s\n' "Withdraw delegator reward failed: ""$RES"" " >&2
    exit 1
fi

# CREATE PROPOSAL
RES=$(CUDOS_NODED tx gov submit-proposal software-upgrade TestSoft --upgrade-height=200000 --from alice --title TestUpdate --description TestDescription --deposit 1acudos --keyring-backend test --chain-id="$CHAIN_ID" $FEE_FLAGS -y)
RES_CODE=$(echo $RES | jq -r '.code')

if [[ $RES_CODE != 0 ]];then
    printf '%s\n' "Create proposal failed: ""$RES"" " >&2
    exit 1
fi

PROPOSAL_ID=$(echo $RES | jq -r '.logs[0].events[-2].attributes[0].value')

# ACTIVATE PROPOSAL
RES=$(CUDOS_NODED tx gov deposit $PROPOSAL_ID 50000000000000000000000acudos --from $aliceAddress --keyring-backend test --chain-id="$CHAIN_ID" $FEE_FLAGS -y)
RES_CODE=$(echo $RES | jq -r '.code')

if [[ $RES_CODE != 0 ]];then
    printf '%s\n' "Activating proposal failed: ""$RES"" " >&2
    exit 1
fi

# VOTE ON PROPOSAL
echo "VOTE ON PROPOSAL"
VOTE=$(read_json $TEST_DIR'/integration_tests/native_msgs/vote_msg.json')
VOTE=$(echo $VOTE | jq '.vote_msg.proposal_id = '$PROPOSAL_ID)
VOTE=$(trim_json "$VOTE")
RES=$(exec_msg $VOTE "alice")
RES_CODE=$(echo $RES | jq -r '.code')

if [[ $RES_CODE != 0 ]];then
    printf '%s\n' "Vote failed: ""$RES"" " >&2
    exit 1
fi

# QUERY VOTES FOR PROPOSAL
VOTER=$(CUDOS_NODED query gov votes $PROPOSAL_ID --output json | jq -r '.votes[0].voter')

if [ "$VOTER" != "$CONTRACT" ];then
    printf '%s\n' "Vote failed: ""$RES"" " >&2
    exit 1
fi

# INSTANTIATE FROM CONTRACT
echo "INSTANTIATE FROM CONTRACT"
INSTANTIATE=$(read_json $TEST_DIR'/integration_tests/native_msgs/instantiate_msg.json')
INSTANTIATE=$(echo $INSTANTIATE | jq '.instantiate_msg.code_id = '$CODE_ID)
INSTANTIATE=$(trim_json "$INSTANTIATE")
RES=$(exec_msg $INSTANTIATE "alice")
RES_CODE=$(echo $RES | jq -r '.code')

if [[ $RES_CODE != 0 ]];then
    printf '%s\n' "Instantiate from contract failed: ""$RES"" " >&2
    exit 1
fi

INSTANTIATED_CONTRACT_ADDR=$(echo $RES | jq -r '.logs[0].events[-2].attributes[0].value')

echo "ADD FUNDS TO CONTRACT"
RES=$(CUDOS_NODED tx bank send faucet "$INSTANTIATED_CONTRACT_ADDR" 100000000000000000000acudos --keyring-backend test --chain-id="$CHAIN_ID" $FEE_FLAGS -y)

# EXECUTE FROM CONTRACT
echo "EXECUTE FROM CONTRACT"
EXECUTE=$(read_json $TEST_DIR'/integration_tests/native_msgs/execute_msg.json')
EXECUTE=$(echo $EXECUTE | jq '.execute_msg.contract_addr = "'$INSTANTIATED_CONTRACT_ADDR'"')
EXECUTE=$(trim_json "$EXECUTE")
RES=$(exec_msg $EXECUTE "alice")
RES_CODE=$(echo $RES | jq -r '.code')

if [[ $RES_CODE != 0 ]];then
    printf '%s\n' "Execute from contract failed: ""$RES"" " >&2
    exit 1
fi

cd $TEST_DIR