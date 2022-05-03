CHAIN_ID=$1 # cudos-network
CONTRACT_PATH=$2
CUDOS_NODED_RUNNING_INSTANCE_PATH=$3

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
echo yes | CUDOS_NODED keys add wasm-tester --keyring-backend test
echo yes | CUDOS_NODED keys add wasm-nft-receiver --keyring-backend test

wasmTesterAddress=$(CUDOS_NODED keys show -a wasm-tester --keyring-backend test)
wasmNFTReceiverAddress=$(CUDOS_NODED keys show -a wasm-nft-receiver --keyring-backend test)

# ADD FUNDS
echo "ADD FUNDS"
CUDOS_NODED tx bank send faucet "$wasmTesterAddress" 100000000000000000000acudos --keyring-backend test --chain-id="$CHAIN_ID" $FEE_FLAGS -y
CUDOS_NODED tx bank send faucet "$wasmNFTReceiverAddress" 100000000000000000000acudos --keyring-backend test --chain-id="$CHAIN_ID" $FEE_FLAGS -y

# STORE CODE
echo "STORE CODE"
RES=$(CUDOS_NODED tx wasm store "$CONTRACT_PATH" --from wasm-tester $FEE_FLAGS --keyring-backend test --chain-id=$CHAIN_ID -y)
CODE_ID=$(echo $RES | jq -r '.logs[0].events[-1].attributes[0].value')
echo "Deployed code id: $CODE_ID"

# INSTANTIATE CONTRACT
echo "INSTANTIATE CONTRACT"
RES=$(CUDOS_NODED tx wasm instantiate $CODE_ID "{}" --label "test bindings" --no-admin --from wasm-tester $FEE_FLAGS --keyring-backend test --chain-id="$CHAIN_ID" -y)

# VERIFY INSTANTIATION
echo "VERIFY INSTANTIATION"
CUDOS_NODED query wasm list-contract-by-code "$CODE_ID"
CONTRACT=$(CUDOS_NODED query wasm list-contract-by-code "$CODE_ID" --output json | jq -r '.contracts[0]')
echo "Contract: $CONTRACT"
if [ "$CONTRACT" == "" ];then
    printf '%s\n' "Failed to init the smart contract on the chain. Check the tx result above for a detailed error info" >&2 # write error message to stderr
    printf '%s\n' "Failed INIT Transaction: ""$RES"" " >&2 # write error message to stderr

    exit 1
fi

# ISSUE DENOM
echo "ISSUE DENOM"
ISSUE_DENOM_MSG=$(read_json "$TEST_DIR"'/integration_tests/msgs/issue_denom_msg.json')
RES=$(exec_msg $ISSUE_DENOM_MSG "wasm-tester")

# VERIFY DENOM
echo "VERIFY DENOM"
QUERY_DENOM_BY_ID=$(read_json "$TEST_DIR"'/integration_tests/msgs/query_denom_by_id.json')
DENOM_ID=$(echo $ISSUE_DENOM_MSG | jq -r '.issue_denom_msg.id')
ISSUED_DENOM_ID=$(CUDOS_NODED query wasm contract-state smart $CONTRACT $QUERY_DENOM_BY_ID --output json | jq -r '.data.denom.id')

if [ "$ISSUED_DENOM_ID" != "$DENOM_ID" ];then
    printf '%s\n' "Failed to issue denom: ""$RES"" " >&2
    exit 1
fi

# VERIFY QUERY DENOM BY NAME
echo "VERIFY QUERY DENOM BY NAME"
QUERY_DENOM_BY_NAME=$(read_json "$TEST_DIR"'/integration_tests/msgs/query_denom_by_name.json')
ISSUED_DENOM_ID=$(CUDOS_NODED query wasm contract-state smart $CONTRACT $QUERY_DENOM_BY_NAME --output json | jq -r '.data.denom.id')

if [ "$ISSUED_DENOM_ID" != "$DENOM_ID" ];then
    printf '%s\n' "Failed to query denom by name: ""$RES"" " >&2
    exit 1
fi

# VERIFY QUERY DENOM BY SYMBOL
echo "VERIFY QUERY DENOM BY SYMBOL"
QUERY_DENOM_BY_SYMBOL=$(read_json "$TEST_DIR"'/integration_tests/msgs/query_denom_by_symbol.json')
ISSUED_DENOM_ID=$(CUDOS_NODED query wasm contract-state smart $CONTRACT $QUERY_DENOM_BY_SYMBOL --output json | jq -r '.data.denom.id')

if [ "$ISSUED_DENOM_ID" != "$DENOM_ID" ];then
    printf '%s\n' "Failed to query denom by symbol: ""$RES"" " >&2
    exit 1
fi

# VERIFY QUERY ALL DENOMS
echo "VERIFY QUERY ALL DENOMS"
QUERY_DENOMS=$(read_json "$TEST_DIR"'/integration_tests/msgs/query_denoms.json')
QUERY_RES=$(CUDOS_NODED query wasm contract-state smart $CONTRACT $QUERY_DENOMS --output json)
DENOM_ID=$(echo $QUERY_RES | jq -r '.data.denoms[0].id')

if [ "$ISSUED_DENOM_ID" != "$DENOM_ID" ];then
    printf '%s\n' "Failed to query all denoms: ""$RES"" " >&2
    exit 1
fi

# MINT NFT
echo "MINT NFT"
MINT_NFT_MSG=$(jq '.mint_nft_msg.recipient = "'$wasmTesterAddress'"' $TEST_DIR'/integration_tests/msgs/mint_nft_msg.json')
MINT_NFT_MSG=$(trim_json "$MINT_NFT_MSG")
RES=$(exec_msg $MINT_NFT_MSG "wasm-tester")

# VERIFY NFT MINT
echo "VERIFY NFT MINT"
QUERY_NFT=$(read_json "$TEST_DIR"'/integration_tests/msgs/query_nft.json')
NFT_ID=$(echo $QUERY_NFT | jq -r '.query_token.token_id')
MINTED_NFT_ID=$(CUDOS_NODED query wasm contract-state smart $CONTRACT $QUERY_NFT --output json | jq -r '.data.nft.id')

if [ "$MINTED_NFT_ID" != "$NFT_ID" ];then
    printf '%s\n' "Failed to mint NFT: ""$RES"" " >&2
    exit 1
fi

# EDIT NFT
echo "EDIT NFT"
EDIT_NFT_MSG=$(read_json "$TEST_DIR"'/integration_tests/msgs/edit_nft_msg.json')
RES=$(exec_msg $EDIT_NFT_MSG "wasm-tester")

# VERIFY NFT EDIT
echo "VERIFY NFT EDIT"
QUERY_NFT=$(read_json "$TEST_DIR"'/integration_tests/msgs/query_nft.json')
NEW_NFT_NAME=$(echo $EDIT_NFT_MSG | jq -r '.edit_nft_msg.name')
NEW_NFT_URI=$(echo $EDIT_NFT_MSG | jq -r '.edit_nft_msg.uri')
QUERY_RES=$(CUDOS_NODED query wasm contract-state smart $CONTRACT $QUERY_NFT --output json)
NFT_NAME=$(echo $QUERY_RES | jq -r '.data.nft.name')
NFT_URI=$(echo $QUERY_RES | jq -r '.data.nft.uri')

if [ "$NFT_NAME" != "$NEW_NFT_NAME" ] || [ "$NFT_URI" != "$NEW_NFT_URI" ];then
    printf '%s\n' "Failed to edit NFT: ""$RES"" " >&2
    exit 1
fi

# TRANSFER NFT
echo "TRASNFER NFT"
TRANSFER_NFT_MSG=$(jq '.transfer_nft_msg.to = "'$wasmNFTReceiverAddress'"' $TEST_DIR'/integration_tests/msgs/transfer_nft_msg.json')
TRANSFER_NFT_MSG=$(echo $TRANSFER_NFT_MSG | jq '.transfer_nft_msg.from = "'$wasmTesterAddress'"')
TRANSFER_NFT_MSG=$(trim_json "$TRANSFER_NFT_MSG")
RES=$(exec_msg $TRANSFER_NFT_MSG "wasm-tester")

# VERIFY TRANSFER
echo "VERIFY TRANSFER"
QUERY_RES=$(CUDOS_NODED query wasm contract-state smart $CONTRACT $QUERY_NFT --output json)
NEW_NFT_OWNER=$(echo $QUERY_RES | jq -r '.data.nft.owner')

if [ "$NEW_NFT_OWNER" != "$wasmNFTReceiverAddress" ];then
    printf '$s\n' "Failed to transfer NFT: ""$RES"" " >&2
    exit 1
fi

# APPROVE NFT
echo "APPROVE NFT"
APPROVE_NFT_MSG=$(jq '.approve_nft_msg.approved_address = "'$wasmTesterAddress'"' $TEST_DIR'/integration_tests/msgs/approve_nft_msg.json')
APPROVE_NFT_MSG=$(trim_json "$APPROVE_NFT_MSG")
RES=$(exec_msg $APPROVE_NFT_MSG "wasm-nft-receiver")

# VERIFY APPROVE
echo "VERIFY APPROVE"
QUERY_NFT_APPROVALS=$(read_json $TEST_DIR'/integration_tests/msgs/query_nft_approvals.json')
QUERY_RES=$(CUDOS_NODED query wasm contract-state smart $CONTRACT $QUERY_NFT_APPROVALS --output json)
APPROVED_ADDRESS=$(echo $QUERY_RES | jq -r '.data.approved_addresses[0]')

if [ "$APPROVED_ADDRESS" != "$wasmTesterAddress" ];then
    printf '%s\n' "Failed to verify approve: ""$RES"" " >&2
    exit 1
fi

# REVOKE APPROVE NFT
echo "REVOKE APPROVE NFT"
REVOKE_APPROVE_NFT=$(read_json $TEST_DIR'/integration_tests/msgs/revoke_approve_nft_msg.json')
REVOKE_APPROVE_NFT=$(echo $REVOKE_APPROVE_NFT | jq '.revoke_approval_msg.address_to_revoke = "'$wasmTesterAddress'"')
REVOKE_APPROVE_NFT=$(trim_json "$REVOKE_APPROVE_NFT")
RES=$(exec_msg $REVOKE_APPROVE_NFT "wasm-nft-receiver")

# VERIFY REVOKE APPROVE NFT
echo "VERIFY REVOKE APPROVE NFT"
QUERY_NFT=$(read_json $TEST_DIR'/integration_tests/msgs/query_nft.json')
QUERY_RES=$(CUDOS_NODED query wasm contract-state smart $CONTRACT $QUERY_NFT --output json)
APPROVED_ADDRESS=$(echo $QUERY_RES | jq -r '.data.nft.approved_addresses[0]')

if [ "$APPROVED_ADDRESS" == "$wasmTesterAddress" ];then
    printf '%s\n' "Failed to revoke approve: ""$RES"" " >&2
    exit 1
fi

# APPROVE ALL
echo "APPROVE ALL"
APPROVE_ALL_MSG=$(jq '.approve_all_msg.approved_operator = "'$wasmNFTReceiverAddress'"' $TEST_DIR'/integration_tests/msgs/approve_all_msg.json')
APPROVE_ALL_MSG=$(trim_json "$APPROVE_ALL_MSG")
RES=$(exec_msg $APPROVE_ALL_MSG "wasm-tester")

# VERIFY APPROVE ALL
echo "VERIFY APPROVE ALL"
QUERY_APPROVED_FOR_ALL=$(read_json $TEST_DIR'/integration_tests/msgs/query_approved_for_all.json')
QUERY_APPROVED_FOR_ALL=$(echo $QUERY_APPROVED_FOR_ALL | jq '.query_approved_for_all.owner_address = "'$wasmTesterAddress'"')
QUERY_APPROVED_FOR_ALL=$(echo $QUERY_APPROVED_FOR_ALL | jq '.query_approved_for_all.operator_address = "'$wasmNFTReceiverAddress'"')
QUERY_APPROVED_FOR_ALL=$(trim_json "$QUERY_APPROVED_FOR_ALL")
QUERY_RES=$(CUDOS_NODED query wasm contract-state smart $CONTRACT $QUERY_APPROVED_FOR_ALL --output json)
IS_APPROVED=$(echo $QUERY_RES | jq -r '.data.is_approved')

if [ $IS_APPROVED != true ];then
    printf '%s\n' "Failed to approve all: ""$RES"" " >&2
    exit 1
fi

# REVOKE APPROVE ALL
echo "REVOKE APPROVE ALL"
REVOKE_APPROVE_ALL_MSG=$(jq '.approve_all_msg.approved_operator = "'$wasmNFTReceiverAddress'"' $TEST_DIR'/integration_tests/msgs/approve_all_msg.json')
REVOKE_APPROVE_ALL_MSG=$(echo $REVOKE_APPROVE_ALL_MSG | jq '.approve_all_msg.approved = false')
REVOKE_APPROVE_ALL_MSG=$(trim_json "$REVOKE_APPROVE_ALL_MSG")
RES=$(exec_msg $REVOKE_APPROVE_ALL_MSG "wasm-tester")

# VERIFY REVOKE APPROVE ALL
echo "VERIFY REVOKE APPROVE ALL"
QUERY_RES=$(CUDOS_NODED query wasm contract-state smart $CONTRACT $QUERY_APPROVED_FOR_ALL --output json)
IS_APPROVED=$(echo $QUERY_RES | jq -r '.data.is_approved')

if [ $IS_APPROVED != false ];then
    printf '%s\n' "Failed to revoke approve all: ""$RES"" " >&2
    exit 1
fi

# VERIFY QUERY COLLECTION
echo "VERIFY QUERY COLLECTION"
QUERY_COLLECTION=$(read_json $TEST_DIR'/integration_tests/msgs/query_collection.json')
QUERY_RES=$(CUDOS_NODED query wasm contract-state smart $CONTRACT $QUERY_COLLECTION --output json)
NFT_NAME=$(echo $QUERY_RES | jq -r '.data.collection.nfts[0].name')

if [ "$NFT_NAME" != "$NEW_NFT_NAME" ];then
    printf '%s\n' "Failed to query collection: ""$RES"" " >&2
    exit 1
fi

# VERIFY QUERY SUPPLY
echo "VERIFY QUERY SUPPLY"
QUERY_SUPPLY=$(read_json $TEST_DIR'/integration_tests/msgs/query_supply.json')
QUERY_RES=$(CUDOS_NODED query wasm contract-state smart $CONTRACT $QUERY_SUPPLY --output json)
SUPPLY=$(echo $QUERY_RES | jq -r '.data.amount')

if [ $SUPPLY != 1 ];then
    printf '%s\n' "Failed to query supply: ""$RES"" " >&2
    exit 1
fi

# VERIFY QUERY OWNER NFTs
echo "VERIFY QUERY OWNER NFTs"
QUERY_OWNER=$(read_json $TEST_DIR'/integration_tests/msgs/query_owner.json')
QUERY_OWNER=$(echo $QUERY_OWNER | jq '.query_owner.address = "'$wasmNFTReceiverAddress'"')
QUERY_OWNER=$(trim_json "$QUERY_OWNER")
QUERY_RES=$(CUDOS_NODED query wasm contract-state smart $CONTRACT $QUERY_OWNER --output json)
OWNED_TOKEN_ID=$(echo $QUERY_RES | jq -r '.data.owner.id_collections[0].token_ids[0]')

if [ $OWNED_TOKEN_ID != 1 ];then
    printf '%s\n' "Failed to query owner NFTs: ""$RES"" " >&2
    exit 1
fi

# BURN NFT
echo "BURN NFT"
BURN_NFT=$(read_json $TEST_DIR'/integration_tests/msgs/burn_nft_msg.json')
RES=$(exec_msg $BURN_NFT "wasm-nft-receiver")

# VERIFY BURN NFT
echo "VERIFY BURN NFT"
QUERY_RES=$(CUDOS_NODED query wasm contract-state smart $CONTRACT $QUERY_OWNER --output json)
COLLECTION_LENGTH=$(echo $QUERY_RES | jq -r '.data.owner.id_collections | length')

if [ $COLLECTION_LENGTH != 0 ];then
    printf '%s\n' "Failed to burn NFT: ""$RES"" " >&2
    exit 1
fi