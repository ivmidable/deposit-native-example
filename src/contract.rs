#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    coin, to_binary, BankMsg, Binary, Deps, DepsMut, Env, MessageInfo, Order, Response, StdResult, Uint128, WasmQuery
};
use cw2::set_contract_version;
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{DepositResponse, ExecuteMsg, InstantiateMsg, OffersResponse, QueryMsg};
use crate::state::{Deposits, Offer, ASKS, BIDS, DEPOSITS};

const CONTRACT_NAME: &str = "deposit-native-example";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
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
        ExecuteMsg::Deposit {} => execute_deposit(deps, info),
        ExecuteMsg::Withdraw { amount, denom } => execute_withdraw(deps, info, amount, denom),
        ExecuteMsg::AddBid { token_id } => execute_add_bid(deps, info, token_id),
        ExecuteMsg::AddAsk {
            token_id,
            amount,
            denom,
        } => execute_add_ask(deps, info, token_id, amount, denom),
        ExecuteMsg::RemoveOffer { token_id } => execute_remove_offer(deps, info, token_id),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Deposits { address } => to_binary(&query_deposits(deps, address)?),
        QueryMsg::AddressOffers { address, bid } => {
            to_binary(&query_address_offers(deps, address, bid)?)
        }
        QueryMsg::TokenIdOffers { token_id, bid } => {
            to_binary(&query_token_id_offers(deps, token_id, bid)?)
        }
    }
}

pub fn execute_deposit(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    let sender = info.sender.clone().into_string();
    let d_coins = info.funds[0].clone();
    //check to see if u
    match DEPOSITS.load(deps.storage, (&sender, d_coins.denom.as_str())) {
        Ok(mut deposit) => {
            //add coins to their account
            deposit.coins.amount = deposit.coins.amount.checked_add(d_coins.amount).unwrap();
            deposit.count = deposit.count.checked_add(1).unwrap();
            DEPOSITS
                .save(deps.storage, (&sender, d_coins.denom.as_str()), &deposit)
                .unwrap();
        }
        Err(_) => {
            //user does not exist, add them.
            let deposit = Deposits {
                count: 1,
                owner: info.sender,
                coins: d_coins.clone(),
            };
            DEPOSITS
                .save(deps.storage, (&sender, d_coins.denom.as_str()), &deposit)
                .unwrap();
        }
    }
    Ok(Response::new()
        .add_attribute("execute", "deposit")
        .add_attribute("denom", d_coins.denom)
        .add_attribute("amount", d_coins.amount))
}

pub fn execute_withdraw(
    deps: DepsMut,
    info: MessageInfo,
    amount: u128,
    denom: String,
) -> Result<Response, ContractError> {
    let sender = info.sender.clone().into_string();

    let mut deposit = DEPOSITS
        .load(deps.storage, (&sender, denom.as_str()))
        .unwrap();
    deposit.coins.amount = deposit
        .coins
        .amount
        .checked_sub(Uint128::from(amount))
        .unwrap();
    deposit.count = deposit.count.checked_sub(1).unwrap();
    DEPOSITS
        .save(deps.storage, (&sender, denom.as_str()), &deposit)
        .unwrap();

    let msg = BankMsg::Send {
        to_address: sender.clone(),
        amount: vec![coin(amount, denom.clone())],
    };

    Ok(Response::new()
        .add_attribute("execute", "withdraw")
        .add_attribute("denom", denom)
        .add_attribute("amount", amount.to_string())
        .add_message(msg))
}

pub fn execute_add_bid(
    deps: DepsMut,
    info: MessageInfo,
    token_id: String,
) -> Result<Response, ContractError> {
    let sender = info.sender.clone().into_string();
    let d_coins = info.funds[0].clone();
    //check to see if u
    if BIDS.has(deps.storage, (&sender, &token_id)) {
        return Err(ContractError::InvalidBid {});
    }
    //user does not exist, add them.
    let offer = Offer {
        token_id: token_id.clone(),
        amount: d_coins.clone(),
    };
    BIDS.save(deps.storage, (&sender, token_id.as_str()), &offer)
        .unwrap();
    BIDS.save(deps.storage, (&token_id.as_str(), sender.as_str()), &offer)
        .unwrap();
    Ok(Response::new()
        .add_attribute("execute", "add_bid")
        .add_attribute("denom", d_coins.denom)
        .add_attribute("amount", d_coins.amount)
        .add_attribute("token_id", token_id))
}

pub fn execute_add_ask(
    deps: DepsMut,
    info: MessageInfo,
    token_id: String,
    amount: u128,
    denom: String,
) -> Result<Response, ContractError> {
    unimplemented!()
}

pub fn execute_remove_offer(
    deps: DepsMut,
    info: MessageInfo,
    token_id: String,
) -> Result<Response, ContractError> {
    unimplemented!()
}

fn query_deposits(deps: Deps, address: String) -> StdResult<DepositResponse> {
    let res: StdResult<Vec<_>> = DEPOSITS
        .prefix(&address)
        .range(deps.storage, None, None, Order::Ascending)
        .collect();
    let deposits = res?;
    Ok(DepositResponse { deposits })
}

fn query_address_offers(deps: Deps, address: String, bid: bool) -> StdResult<OffersResponse> {
    unimplemented!()
}

fn query_token_id_offers(deps: Deps, token_id: String, bid: bool) -> StdResult<OffersResponse> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coin, from_binary};

    const SENDER: &str = "sender_address";
    const AMOUNT: u128 = 100000;
    const DENOM: &str = "utest";

    fn setup_contract(deps: DepsMut) {
        let msg = InstantiateMsg {};
        let info = mock_info(SENDER, &[]);
        let res = instantiate(deps, mock_env(), info, msg).unwrap();
        println!("{:?}", res);
        assert_eq!(0, res.messages.len());
    }

    fn deposit_coins(deps: DepsMut) {
        let msg = ExecuteMsg::Deposit {};
        let coins = vec![coin(AMOUNT, DENOM.to_string())];
        let info = mock_info(SENDER, &coins);
        let res = execute(deps, mock_env(), info, msg).unwrap();
        assert_eq!("deposit".to_string(), res.attributes[0].value);
        assert_eq!(DENOM.to_string(), res.attributes[1].value);
        assert_eq!(AMOUNT.to_string(), res.attributes[2].value);
    }

    fn withdraw_coins(deps: DepsMut) {}

    fn query_coins(deps: Deps) {
        let msg: QueryMsg = QueryMsg::Deposits {
            address: SENDER.to_string(),
        };
        let res = query(deps, mock_env(), msg).unwrap();
        let query = from_binary::<DepositResponse>(&res).unwrap();
        assert_eq!(SENDER, query.deposits[0].1.owner);
        assert_eq!(DENOM, query.deposits[0].1.coins.denom);
        assert_eq!(
            AMOUNT.to_string(),
            query.deposits[0].1.coins.amount.to_string()
        );
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
