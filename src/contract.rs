#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    Order, Binary, Coin, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128, to_binary, coin
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
        ExecuteMsg::Deposit { } => execute_deposit(deps, info),
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
) -> Result<Response, ContractError> {
    let sender = info.sender.clone().into_string();
    let d_coins = info.funds[0].clone();
    //check to see if u
    match DEPOSITS.load(deps.storage, (&sender, d_coins.denom.as_str())) {
        Ok(mut deposit) => {
            //add coins to their account
            deposit.coins.amount = deposit.coins.amount.checked_add(d_coins.amount).unwrap();
            deposit.count = deposit.count.checked_add(1).unwrap();
            DEPOSITS.save(deps.storage, (&sender, d_coins.denom.as_str()), &deposit).unwrap();
        }
        Err(_) => {
            //user does not exist, add them.
            let deposit = Deposits {
                count: 1,
                owner: info.sender,
                coins: d_coins.clone(),
            };
            DEPOSITS.save(deps.storage, (&sender, d_coins.denom.as_str()), &deposit).unwrap();
        }
    }
    Ok(Response::new()
        .add_attribute("execute", "deposit")
        .add_attribute("denom", d_coins.denom)
        .add_attribute("amount", d_coins.amount)
    )
}

fn query_deposits(deps: Deps, address:String) -> StdResult<DepositResponse> {
    let res: StdResult<Vec<_>> = DEPOSITS.prefix(&address).range(deps.storage, None, None, Order::Ascending).collect();
    let deposits = res?;
    Ok(DepositResponse { deposits })
}

#[cfg(test)]
mod tests { 
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coin, from_binary};

    const SENDER: &str = "sender_address";
    const AMOUNT:u128 = 100000;
    const DENOM:&str = "utest";

    fn setup_contract(deps: DepsMut) {
        let msg = InstantiateMsg { };
        let info = mock_info(SENDER, &[]);
        let res = instantiate(deps, mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());
    }

    fn deposit_coins(deps: DepsMut) {
        let msg = ExecuteMsg::Deposit { };
        let coins = vec![coin(AMOUNT, DENOM.to_string())];
        let info = mock_info(SENDER, &coins);
        let res = execute(deps, mock_env(), info, msg).unwrap();
        assert_eq!("deposit".to_string(), res.attributes[0].value);
        assert_eq!(DENOM.to_string(), res.attributes[1].value);
        assert_eq!(AMOUNT.to_string(), res.attributes[2].value);
    }

    #[test]
    fn _0_instantiate() {
        let mut deps = mock_dependencies();
        setup_contract(deps.as_mut());
    }

    #[test]
    fn _1_deposit() {
        let mut deps = mock_dependencies();
        setup_contract(deps.as_mut());
        deposit_coins(deps.as_mut());
    }

    //Add code to query the deposits and check if they were properly stored
    #[test]
    fn _0_query_deposit() {
        let mut deps = mock_dependencies();
        setup_contract(deps.as_mut());
        deposit_coins(deps.as_mut());
    }
}
