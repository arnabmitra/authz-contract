#!/bin/bash
export PROVENANCE_DIR="$HOME/provenance"
# Run this after deploying and getting the code ID
# User should pass in the code ID to the contract like:
# ./instantiate-local.sh 2
if [ -z "$1" ]
then
    echo "Must provide code ID (Example ./instantiate-local.sh 19 tp...)"
    exit 1
else
    CODE_ID=$1
fi

#INIT='{"allowed": ["tp1gvc0l4upc88arx673tmg7u3g7zsssnyyle5ph5"]}'
INIT='{"allowed": []}'
${PROVENANCE_DIR}/build/provenanced -t tx wasm instantiate "$CODE_ID" "$INIT" --label "authz_demo" --from validator --home ${PROVENANCE_DIR}/build/run/provenanced  --node http://localhost:26657 --chain-id testing --gas-prices 1905nhash --gas auto --gas-adjustment 2 --output json -b block --no-admin -y | jq