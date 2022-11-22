# AuthzDemo

The authz contract demo will demonstrate how to grant a contract the permissions to do a bank send on behalf of another account.

## Steps
1. Running the build script will create wasm file. `build.sh`
2. Store the contract on the chain by using `deploy-local.sh`
3. Instantiate contract with `instantiate-local.sh <contract-code>`
4. Assign authz rights for a bank send to contract using `authz-setup.sh`
5. Execute contract to do a bank send with `execute-local.sh` 

NOTE: `authz-setup.sh` and `execute-local.sh` will need the `CONTRACT_ADDRESS` variable updated to the instantiated contract address.