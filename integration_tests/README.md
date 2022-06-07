# Tests:
* interactions between smart contracts and cudos-noded NFT module are working.
* interactions between smart contracts and native modules bank/gov/staking.
* smart contract to smart contract instantiation and execution.

# How to start tests
1) Start cudos-node instance by executing ```init.sh``` inside  ```/cudos-node/```
2) Compile the native_tester and nft_bindings_tester smart contracts.
3) From the ```cudos-cosmwasm-bindings``` directory execute the tests ```./integration_tests/test.sh cudos-local-network path_to_native_tester.wasm path_to_nft_bindings_tester.wasm path_to_cudos_node_root_dir```