 # Cudos bindings for CosmWasm

 This crate provides the custom bindings that are used to communicate with the custom modules on the Cudos network from a CosmWasm smart contract.
 Currently only the NFT module bindings are exposed.

 # Installation
 Add the crate to your smart contract's' Cargo.toml
 ```toml
[dependencies]
cudos-cosmwasm = { version = "0.0.1" }
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
cudos-noded tx wasm store $clonedDir/bindings_tester.wasm --from=validator-02 --chain-id=cudos-network --gas=auto -y
INIT='{}'
CODE='1' 
cudos-noded tx wasm instantiate $CODE $INIT --from=validator-02 --label="tester" --chain-id=cudos-network --gas=auto -y
TESTER=$(cudos-noded query wasm list-contract-by-code $CODE --output json | jq -r '.contracts[-1]')
echo $TESTER

# NOTE: sender field in the queries should be the address of your contract, in this case - $TESTER
# issueDenom
issueDenomQuery='{
    "issue_denom_msg": {
        "id": "testdenom",
        "name": "TESTDENOM",
        "name": "testSymbol",
        "schema": "testschema"
    }
}'
cudos-noded tx wasm execute $TESTER $issueDenomQuery --from=validator-02 --chain-id=cudos-network --gas=auto -y 

# query a denom
denomQuery='{
    "query_denom_by_id": {
        "denom_id": "testdenom"
    }
}'
cudos-noded query wasm contract-state smart $TESTER $denomQuery --output json

# mint a NFT
mintNft='{
    "mint_nft_msg": {
        "denom_id": "testdenom",
        "name": "testtoken",
        "uri": "",
        "data": "testData",
        "recipient": ""
    }
}'
cudos-noded tx wasm execute $TESTER $mintNft --from=validator-02 --chain-id=cudos-network --gas=auto -y 

# query for a NFT
nftQuery='{
    "query_token": {
        "denom_id": "testdenom",
        "token_id": "1"
    }
}'
cudos-noded query wasm contract-state smart $TESTER $nftQuery --output json


# edit a NFT
editNft='{
    "edit_nft_msg": {
        "denom_id": "testdenom",
        "token_id": "1",
        "name": "testtokenChanged",
        "uri": "",
        "data": "testData"
    }
}'
cudos-noded tx wasm execute $TESTER $editNft --from=validator-02 --chain-id=cudos-network --gas=auto -y 

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
cudos-noded tx wasm execute $TESTER $transferNft --from=validator-02 --chain-id=cudos-network --gas=auto -y 

# add approved address for a NFT
addApprovedAddress='{
    "approve_nft_msg": {
        "denom_id": "testdenom",
        "token_id": "1",
        "approved_address": ""
    }
}'
cudos-noded tx wasm execute $TESTER $addApprovedAddress --from=validator-02 --chain-id=cudos-network --gas=auto -y 

# add approve all for an address
addApproveAll='{
    "approve_all_msg": {
        "approved_operator": "",
        "approved": "true"
    }
}'
cudos-noded tx wasm execute $TESTER $addApproveAll --from=validator-02 --chain-id=cudos-network --gas=auto -y 


# revoke approval for a NFT
revokeApprovalNFT='{
    "revoke_approval_msg": {
        "denom_id": "testdenom",
        "token_id": "1",
        "address_to_revoke": ""
    }
}'
cudos-noded tx wasm execute $TESTER $revokeApprovalNFT --from=validator-02 --chain-id=cudos-network --gas=auto -y 

# burn nft
burnNft='{
    "burn_nft_msg": {
        "denom_id": "testdenom",
        "token_id": "1"
    }
}'


cudos-noded tx wasm execute $TESTER $burnNft --from=validator-02 --chain-id=cudos-network --gas=auto -y 

```
# Known issues:
When querying for an nft, which has a non-nil ApprovedAddresses(map[string]bool) this is returned:
```
Error: rpc error: code = InvalidArgument desc = Error calling the VM: Error executing Wasm: Wasmer runtime error: RuntimeError: unreachable: query wasm contract failed: invalid request
```
As from my findings, this is because the wasm module cannot properly serialize map[string]bool for some reason..works correctly if the map is nil..