use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Coin};
use cw_storage_plus::Map;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Deposits {
    pub count: i32,
    pub owner: Addr,
    pub coins: Coin
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Offer {
    pub token_id: String,
    pub amount: Coin
}

//key is address, denom
pub const DEPOSITS: Map<(&str, &str), Deposits> = Map::new("deposits");

//key can be address, token_id
//or it could be token_id and address
pub const BIDS: Map<(&str, &str), Offer> = Map::new("bids");
pub const ASKS: Map<(&str, &str), Offer> = Map::new("asks");
