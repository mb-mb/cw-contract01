use cosmwasm_schema::write_api;
use counting_contract::msg::{ExecMsg, InstantiateMsg};

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        execute: ExecMsg,
        query: counting_contract::msg::QueryMsg,
    }
    
    

}