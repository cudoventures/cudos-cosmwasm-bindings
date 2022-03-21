Tests to ensure that interactions between smart contracts and cudos-noded NFT module are working.

# How to start tests
1) Start cudos-node instance by executing ```init.sh``` inside  ```/cudos-node/```
2) Compile the tester smart contract.
3) From the ```cudos-cosmwasm-bindings``` directory execute the tests ```./integration_tests/test.sh cudos-network path_to_bindings_tester.wasm path_to_cudos_node_root_dir```