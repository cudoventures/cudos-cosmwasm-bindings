#!/bin/bash -e

# cleanup from previous runs
rm -rf $CUDOS_HOME
pkill cudos-noded || true
rm -rf $INSTALL_PATH

# clone cudos-node repo and install binary
if ! git clone --branch=$VERSION https://github.com/CudoVentures/cudos-node.git $INSTALL_PATH &> /dev/null; then
  echo "invalid cudos-node version $VERSION"
  exit 1
fi
cd $INSTALL_PATH
make install

# initialize node data folder
cudos-noded init cudos-test-node --home=$CUDOS_HOME --chain-id=$CHAIN_ID &> /dev/null

# add validator account
echo $VALIDATOR_MNEMONIC | cudos-noded keys add validator --home=$CUDOS_HOME --keyring-backend=test --recover
validatorAddr=$(cudos-noded keys show validator -a --home=$CUDOS_HOME --keyring-backend=test)
cudos-noded add-genesis-account $validatorAddr 90000000000000000000000000000000000000acudos,1000cudosAdmin --home=$CUDOS_HOME

# add orchestrator account
echo $ORCHESTRATOR_MNEMONIC | cudos-noded keys add orchestrator --home=$CUDOS_HOME --keyring-backend=test --recover
orchestratorAddr=$(cudos-noded keys show orchestrator --home=$CUDOS_HOME -a --keyring-backend=test)
cudos-noded add-genesis-account $orchestratorAddr 90000000000000000000000000000000000000acudos --home=$CUDOS_HOME

# create validator and add to genesis
cudos-noded gentx validator 2000000000000000000000000acudos 0x41d0b5762341b0fce6adccf69572c663481c7286 $orchestratorAddr --min-self-delegation=2000000000000000000000000  --home=$CUDOS_HOME --chain-id=$CHAIN_ID --keyring-backend=test
cudos-noded collect-gentxs --home=$CUDOS_HOME

# set denom name
sed -i.bak "s/\"stake\"/\"acudos\"/g" $CUDOS_HOME/config/genesis.json
# set validator self-delegate address
sed -i.bak "s/\"static_val_cosmos_addrs\"\: \[\]/\"static_val_cosmos_addrs\": [\"$validatorAddr\"]/" $CUDOS_HOME/config/genesis.json
# set token contract address on ethereum
sed -i.bak "s/\"erc20_to_denoms\"\: \[\]/\"erc20_to_denoms\": [{\"erc20\":\"0x28ea52f3ee46cac5a72f72e8b3a387c0291d586d\",\"denom\":\"acudos\"}]/" $CUDOS_HOME/config/genesis.json
# set minimum-gas-prices
sed -i.bak "s/minimum-gas-prices = \"\"/minimum-gas-prices = \"0acudos\"/" $CUDOS_HOME/config/app.toml
# set denom name
sed -i.bak 's/"denom": "cudos"/"denom": "acudos"/g' $CUDOS_HOME/config/genesis.json
sed -i.bak 's/"bond_denom": "cudos"/"bond_denom": "acudos"/g' $CUDOS_HOME/config/genesis.json

# start node as daemon (in background)
cudos-noded start --halt-height=80 --home=$CUDOS_HOME &> /dev/null &

# wait 8 secs and verify node is producing blocks
sleep 8
if ! cudos-noded status --home=$CUDOS_HOME &> /dev/null; then
  echo "cudos-node did not start successfully"
  exit 1
fi