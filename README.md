 # Cudos bindings for CosmWasm

 This crate provides the custom bindings that are used to communicate with the custom modules on the Cudos network from a CosmWasm smart contract.
 Currently only the NFT module bindings are exposed.

 # Installation
 Add the crate to your smart contract's' Cargo.toml
 ```toml
[dependencies]
cudos-cosmwasm = { version = "0.0.4" }
```

 # Exposed bindings
 This crate, as of now, exports binding only for the NFT module. In the future, more custom binding will be added.
 All the commands from the [`NFT Module`](https://github.com/CudoVentures/cudos-node#the-following-commands-are-available-click-on-them-for-further-info) are available and callable from a smart contract. 

## Creating Messages
​
**NOTE:** The Cudos bindings do not cover messages that have already been implemented by the CosmWasm team, such as staking-related messages and fundamental ones like `MsgSend`.
​
You may want your contract to perform messages such as `IssueDenom` and `MintNft` operations at the end of its execution. To do this, create a message using the predefined functions:
​
- `create_issue_denom_msg`
- `create_mint_nft_msg`
- `create_edit_nft_msg`
- `create_transfer_nft_msg`
- `create_transfer_denom_msg`
- `create_burn_nft_msg`
- `create_approve_nft_msg`
- `create_approve_all_msg`
- `create_revoke_msg`

And add it to your response, like below
​
```rust
use cosmwasm_std::CosmosMsg;
use cudos_cosmwasm::{create_mint_nft_msg};
​
...
​
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
```

### Querying
​
In order to use the query functions enabled by the bindings, create a `CudosQuerier` instance within your contract logic -- in either `init()` or `query()` entrypoints. You can access all the enabled queries through this object.
​
```rust
// src/contract.rs
use cudos_cosmwasm::{ CudosQuerier, DenomResponse, Denom};
​
...
​pub fn query_denom_by_id(deps: Deps, denom_id: String) -> StdResult<DenomResponse> {
    let querier = CudosQuerier::new(&deps.querier);
    let res: DenomResponse = querier.query_denom_by_id(denom_id)?;

    Ok(res)
}

```

# Example

Please consult the example smart contract in /contracts/tester - there you can see an example how to issue a transaction or make a query from the smart contract to the custom module.
You can upload it and interact with it ( and through it - with the cudos chain) with the following steps

```bash
clonedDir='path/to/the/test/smart/contract/binary'
cudos-noded tx wasm store $clonedDir/bindings_tester.wasm --from=<address> --chain-id=<chain-id> --gas=auto -y
INIT='{}'
CODE='1' 
cudos-noded tx wasm instantiate $CODE $INIT --from=<address> --label="tester" --chain-id=<chain-id> --gas=auto -y
TESTER=$(cudos-noded query wasm list-contract-by-code $CODE --output json | jq -r '.contracts[-1]')
echo $TESTER

# NOTE: sender field in the queries should be the address of your contract, in this case - $TESTER
# issueDenom
# NOTE: schema is optional field
issueDenomQuery='{
    "issue_denom_msg": {
        "id": "testdenom",
        "name": "TESTDENOM",
        "symbol": "testSymbol",
        "schema": "testschema"
    }
}'
cudos-noded tx wasm execute $TESTER $issueDenomQuery --from=<address> --chain-id=<chain-id> --gas=auto -y 

# query a denom by ID
denomByIdQuery='{
    "query_denom_by_id": {
        "denom_id": "testdenom"
    }
}'
cudos-noded query wasm contract-state smart $TESTER $denomByIdQuery --output json

# query a denom by Name
denomByNameQuery='{
    "query_denom_by_name": {
        "denom_name": "TESTDENOM"               
    }
}'
cudos-noded query wasm contract-state smart $TESTER $denomByNameQuery --output json

# query a denom by Symbol
denomBySymbolQuery='{
    "query_denom_by_symbol": {
        "denom_symbol": "testSymbol"               
    }
}'
cudos-noded query wasm contract-state smart $TESTER $denomBySymbolQuery --output json

# query all denoms
denomsQuery='{
    "query_denoms": {}
}'
cudos-noded query wasm contract-state smart $TESTER $denomsQuery --output json

# query all NFTs related with a given denom
collectionQuery='{
    "query_collection": {
        "denom_id": "testdenom"
    }
}'
cudos-noded query wasm contract-state smart $TESTER $collectionQuery --output json

# query the total count of minted NFTs from a given denom
supplyQuery='{
    "query_supply": {
        "denom_id": "testdenom"
    }
}'
cudos-noded query wasm contract-state smart $TESTER $supplyQuery --output json

# query the NFTs owned by the given address from the given denom
# denom_id is mandatory field. If not provided, returns all NFTs from all denoms owned by the address
ownerQuery='{
    "query_owner": {
        "address": "",
        "denom_id": "testdenom"
    }
}'
cudos-noded query wasm contract-state smart $TESTER $ownerQuery --output json

# query for a NFT
nftQuery='{
    "query_token": {
        "denom_id": "testdenom",
        "token_id": "1"
    }
}'
cudos-noded query wasm contract-state smart $TESTER $nftQuery --output json

# query for the approved addresses associated with the given token of the denom
approvalsQuery='{
    "query_approvals": {
        "denom_id": "testdenom",
        "token_id": "1"
    }
}'
cudos-noded query wasm contract-state smart $TESTER $approvalsQuery --output json

# Query if an address is an authorized operator for another address
approvedForAllQuery='{
    "query_approved_for_all": {
        "owner_address": "",
        "operator_address": ""
    }
}'
cudos-noded query wasm contract-state smart $TESTER $approvedForAllQuery --output json

# mint a NFT
# put the desired recipient address in the json below. Only denom_id, name and recipient are mandatory fields.
mintNft='{
    "mint_nft_msg": {
        "denom_id": "testdenom",
        "name": "",
        "uri": "",
        "data": "",
        "recipient": ""
    }
}'
cudos-noded tx wasm execute $TESTER $mintNft --from=<address> --chain-id=<chain-id> --gas=auto -y 

# edit a NFT
# NOTE: only denomId and tokenId are mandatory. You can provide some or all of the other optional fields
editNft='{
    "edit_nft_msg": {
        "denom_id": "testdenom",
        "token_id": "1",
        "name": "",
        "uri": ""
    }
}'
cudos-noded tx wasm execute $TESTER $editNft --from=<address> --chain-id=<chain-id> --gas=auto -y 

# transfer a NFT
# put the desired addresses in from and to fields in the json below
transferNft='{
    "transfer_nft_msg": {
        "denom_id": "testdenom",
        "token_id": "1",
        "from": "",
        "to": ""
    }
}'
cudos-noded tx wasm execute $TESTER $transferNft --from=<address> --chain-id=<chain-id> --gas=auto -y 

# transfer a NFT collection
# put the desired recipient adress in to field in the json below
transferDenom='{
    "transfer_denom_msg": {
        "denom_id": "testdenom",
        "to": ""
    }
}'
cudos-noded tx wasm execute $TESTER $transferDenom --from=<address> --chain-id=<chain-id> --gas=auto -y 

# add approved address for a NFT
addApprovedAddress='{
    "approve_nft_msg": {
        "denom_id": "testdenom",
        "token_id": "1",
        "approved_address": ""
    }
}'
cudos-noded tx wasm execute $TESTER $addApprovedAddress --from=<address> --chain-id=<chain-id> --gas=auto -y 

# add approve all for an address. PLEASE NOTE the lack of quotes around the boolean.
addApproveAll='{
    "approve_all_msg": {
        "approved_operator": "",
        "approved": true
    }
}'
cudos-noded tx wasm execute $TESTER $addApproveAll --from=<address> --chain-id=<chain-id> --gas=auto -y 


# revoke approval for a NFT
revokeApprovalNFT='{
    "revoke_approval_msg": {
        "denom_id": "testdenom",
        "token_id": "1",
        "address_to_revoke": ""
    }
}'
cudos-noded tx wasm execute $TESTER $revokeNFT --from=<address> --chain-id=<chain-id> --gas=auto -y 

# burn nft
burnNft='{
    "burn_nft_msg": {
        "denom_id": "testdenom",
        "token_id": "1"
    }
}'


cudos-noded tx wasm execute $TESTER $burnNft --from=<address> --chain-id=<chain-id> --gas=auto -y 