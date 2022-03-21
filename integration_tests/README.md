Tests to ensure that interactions between smart contracts and cudos-noded NFT module are working.

# How to start tests
1) Copy ```test.sh``` and ```msgs``` folder inside ```/cudos-node/cosmwasm-testing/```
2) Compile the tester smart contract
3) Execute ```./init.sh``` from ```cudos-node``` folder to start the node.
4) From the ```cudos-node``` directory execute the tests ```./cosmwasm-testing/test.sh cudos-network path_to_bindings_tester.wasm```