#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    has_coins, Binary, Coin, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128, to_binary,
};
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, DepositResponse};
use crate::state::{Deposits, DEPOSITS};

/*
const CONTRACT_NAME: &str = "crates.io:deposit-native-example";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
 */

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Deposit { amount, denom } => execute_deposit(deps, info, amount, denom),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Deposits { address } => {
            to_binary(&query_deposits(deps, address)?)
        }
    }
}

pub fn execute_deposit(
    deps: DepsMut,
    info: MessageInfo,
    amount: u128,
    denom: String,
) -> Result<Response, ContractError> {
    let sender = info.sender.clone().into_string();
    let p_coin = Coin {
        amount: Uint128::from(amount),
        denom: denom,
    };
    //check to see if coins are
    match DEPOSITS.load(deps.storage, &sender) {
        Ok(mut deposit) => {
            //user exists, check to see if they have enough coins
            //add coins to their account
            deposit.coins.amount = deposit.coins.amount.checked_add(p_coin.amount).unwrap();
            deposit.count = deposit.count.checked_add(1).unwrap();
            DEPOSITS.save(deps.storage, &sender, &deposit).unwrap();
        }
        Err(_) => {
            //user does not exist, add them.
            let deposit = Deposits {
                count: 1,
                owner: info.sender,
                coins: p_coin,
            };
            DEPOSITS.save(deps.storage, &sender, &deposit).unwrap();
        }
    }
    Ok(Response::new().add_attribute("execute", "deposit"))
}

fn query_deposits(deps: Deps, address:String) -> StdResult<DepositResponse> {
    let deposits = DEPOSITS.load(deps.storage, &address)?;
    Ok(DepositResponse { deposits })
}

#[cfg(test)]
mod tests {}
