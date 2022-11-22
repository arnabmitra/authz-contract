#!/bin/bash

export PROVENANCE_DIR="$HOME/provenance"
export BIN="$PROVENANCE_DIR/build/provenanced"
export RUN_HOME="$PROVENANCE_DIR/build/run/provenanced"
export GAS_FLAGS="--gas auto --gas-prices 1905nhash --gas-adjustment 2"
export CHAIN="$BIN -t --home $RUN_HOME"
export VALIDATOR1=$($CHAIN keys show validator -a)
export CONTRACT_ADDRESS="tp1ghd753shjuwexxywmgs4xz7x2q732vcnkm6h2pyv9s6ah3hylvrqdzfary"

~/provenance/build/provenanced -t tx wasm store /Users/arnabmitra/ibc/contracts/authz-contract/artifacts/authz_demo.wasm --from validator --home ~/provenance/build/run/provenanced --node http://localhost:26657 --chain-id testing --gas-prices 1905nhash --gas auto --gas-adjustment 1.3 --broadcast-mode block -y --output json -b block
