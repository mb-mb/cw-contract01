use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Coin, Decimal};

#[cw_serde]
pub struct Parent {
    pub addr: String,
    pub donating_period: u64,
    pub part: Decimal,
}

#[cw_serde]
pub struct InstantiateMsg {
    pub minimal_donation: Coin,
    pub parent: Option<Parent>,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ValueResp)]
    Value {},    
}

#[cw_serde]
pub enum ExecMsg {
    Donate {},
    Withdraw {},
}

#[cw_serde]
pub struct ValueResp {
    pub value: u128,
}

#[cw_serde]
pub struct  MigrateMsg {
    pub parent: Option<Parent>
}