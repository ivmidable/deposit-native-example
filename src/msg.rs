use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::state::Deposits;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct InstantiateMsg {

}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Deposit { },
    Withdraw { amount:u128, denom:String },
    AddBid { token_id:String },
    AddAsk { token_id:String, amount:u128, denom:String },
    RemoveOffer { token_id: String },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Deposits { address: String },
    AddressOffers { address: String, bid:bool },
    TokenIdOffers { token_id: String, bid:bool },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct DepositResponse {
    pub deposits: Vec<(String, Deposits)>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct BidsResponse {
    pub bids: Vec<(String, Deposits)>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum MigrateMsg {}
