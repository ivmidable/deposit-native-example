#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    Order, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128, to_binary, coin, BankMsg
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
    CONFIG.save(_deps.storage, &Config {
        owner: _info.sender.clone(),
    })?;
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
        ExecuteMsg::Deposit { } => execute::execute_deposit(deps, info),
        ExecuteMsg::Withdraw { amount, denom } => execute::execute_withdraw(deps, info, amount, denom),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        //TODO: fix the reason query_deposits can't be called from here.
        QueryMsg::Deposits { address } => {
            to_binary(&query::query_deposits(deps, address)?)
        }
        //TODO: add a querymsg for the get config function.
    }
}

// TODO: move execute_deposit and execute_withdraw to the execute module.
pub mod execute {
    use super::*;

    pub fn execute_deposit(
        _deps: DepsMut,
        _info: MessageInfo,
    ) -> Result<Response, ContractError> {
        unimplemented!()
    }

    pub fn execute_withdraw(
        _deps: DepsMut,
        _info: MessageInfo,
        _amount:u128,
        _denom:String
    ) -> Result<Response, ContractError> {
        unimplemented!()
    }

    pub fn update_config(
        deps: DepsMut,
        info: MessageInfo,
        owner: Option<String>,
    ) -> Result<Response, ContractError> {
        let mut config = CONFIG.load(deps.storage)?;
        if config.owner != info.sender {
            return Err(ContractError::InvalidOwner {});
        }
        if let Some(owner) = owner {
            config.owner = deps.api.addr_validate(&owner)?;
        }
        CONFIG.save(deps.storage, &config)?;
        Ok(Response::default())
    }
}

pub mod query {
    use super::*;

    pub fn get_config(deps: Deps) -> StdResult<Config> {
        let config = CONFIG.load(deps.storage)?;
        Ok(config)
    }

    //TODO: move the query deposits code to this function.
    fn query_deposits(deps: Deps, address:String) -> StdResult<DepositResponse> {
        unimplemented!()
    }
}

pub fn execute_deposit(
    deps: DepsMut,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let sender = info.sender.clone().into_string();
    let d_coins = info.funds[0].clone();
    
    //TODO: Make sure sender is the owner in config

    //TODO: make sure funds array is a length of 1

    //check to see if deposit exists
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

pub fn execute_withdraw(
    deps: DepsMut,
    info: MessageInfo,
    amount:u128,
    denom:String
) -> Result<Response, ContractError> {
    let sender = info.sender.clone().into_string();

    let mut deposit = DEPOSITS.load(deps.storage, (&sender, denom.as_str())).unwrap();
    deposit.coins.amount = deposit.coins.amount.checked_sub(Uint128::from(amount)).unwrap();
    deposit.count = deposit.count.checked_sub(1).unwrap();
    DEPOSITS.save(deps.storage, (&sender, denom.as_str()), &deposit).unwrap();

    let msg = BankMsg::Send {
        to_address: sender.clone(),
        amount: vec![coin(amount, denom.clone())],
    };

    Ok(Response::new()
        .add_attribute("execute", "withdraw")
        .add_attribute("denom", denom)
        .add_attribute("amount", amount.to_string())
        .add_message(msg)
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

    fn withdraw_coins(deps: DepsMut) {

    }

    fn query_coins(deps: Deps) {
        let msg: QueryMsg = QueryMsg::Deposits { address: SENDER.to_string() };
        let res = query(deps, mock_env(), msg).unwrap();
        let query = from_binary::<DepositResponse>(&res).unwrap();
        assert_eq!(SENDER, query.deposits[0].1.owner);
        assert_eq!(DENOM, query.deposits[0].1.coins.denom);
        assert_eq!(AMOUNT.to_string(), query.deposits[0].1.coins.amount.to_string());
        assert_eq!(1, query.deposits[0].1.count);
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
    fn _2_query_deposit() {
        let mut deps = mock_dependencies();
        setup_contract(deps.as_mut());
        deposit_coins(deps.as_mut());
        query_coins(deps.as_ref());
    }

    #[test]
    fn _1_deposit_then_withdraw() {
        let mut deps = mock_dependencies();
        setup_contract(deps.as_mut());
        deposit_coins(deps.as_mut());
    }

}
