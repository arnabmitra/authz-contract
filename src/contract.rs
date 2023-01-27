#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Addr, Api, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Config, CONFIG};
use protobuf::Message;
// Get the protobuf file we care about
include!("protos/mod.rs");
use CosmosAuthz::MsgExec;
use CosmosBankSend::Coin;
use CosmosBankSend::MsgSend;

// Version info for migration (boilerplate stuff)
const CONTRACT_NAME: &str = "crates.io:authz-demo";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

// Taken from the cw-plus crate's cw1-whitelist
fn map_validate(api: &dyn Api, admins: &[String]) -> StdResult<Vec<Addr>> {
    admins.iter().map(|addr| api.addr_validate(addr)).collect()
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    // Validate that they sent us good addresses
    let config = Config {
        granter: info.sender,
        allowed: map_validate(deps.api, &msg.allowed)?,
    };

    // This sets the version, imported from cw2, just a normal thing to do
    // Boilerplate, don't worry about it
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new()
        .add_attribute("contract", CONTRACT_NAME)
        .add_attribute("method", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    let contract_address = env.contract.address.to_string();
    match msg {
        ExecuteMsg::TransferAuthFunds {
            to_address,
            granter_address,
            denom,
            amount,
        } => execute_transfer(
            deps,
            info,
            to_address,
            granter_address,
            contract_address,
            denom,
            amount,
        ),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Granter {} => {
            let config = CONFIG.load(deps.storage)?;
            to_binary(&config.granter)
        }
    }
}

pub fn execute_transfer(
    deps: DepsMut,
    _info: MessageInfo,
    to_address: String,
    granter_address: String,
    contract_address: String,
    denom: String,
    amount: String,
) -> Result<Response, ContractError> {
    deps.api.addr_validate(&to_address)?;
    deps.api.addr_validate(&granter_address)?;
    deps.api.addr_validate(&contract_address)?;

    // send from smart contract to a random address
    let mut send = MsgSend::new();
    send.from_address = granter_address; // bob;s address
    send.to_address = to_address; // alice's address
    let mut coin: Coin = Coin::new();
    coin.denom = denom;
    coin.amount = amount;
    send.amount = vec![coin];

    let mut exec = MsgExec::new();
    exec.grantee = contract_address; // contract address
    exec.msgs = vec![send.to_any().unwrap()];
    let exec_bytes: Vec<u8> = exec.write_to_bytes().unwrap();

    let msg = CosmosMsg::Stargate {
        type_url: "/cosmos.authz.v1beta1.MsgExec".to_string(),
        value: Binary::from(exec_bytes),
    };
    Ok(Response::new()
        .add_attribute("contract", "authz_demo")
        .add_attribute("method", "execute_transfer")
        .add_message(msg))
}
