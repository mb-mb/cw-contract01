use cosmwasm_std::{
    entry_point, to_json_binary, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult
};
use error::ContractError;
use msg::{ExecMsg, InstantiateMsg};

mod contract;
pub mod error;
pub mod msg;
#[cfg(test)]
pub mod multitest;
mod state;

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    contract::instantiate(deps, info, msg)
    //Ok(Response::default())
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: msg::QueryMsg) -> StdResult<Binary> {
    use msg::QueryMsg::*;

    match msg {
        Value {} => to_json_binary(&contract::query::value(deps)?),
    }
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    inf: MessageInfo,
    msg: ExecMsg,
) -> Result<Response, ContractError> {
    use msg::ExecMsg::*;

    match msg {
        Donate {} => contract::exec::donate(deps, inf).map_err(ContractError::from),
        Withdraw {} => contract::exec::withdraw(deps, env, inf),
    }
}

#[entry_point]
pub fn migrate(deps: DepsMut, _env: Env, _msg: Empty) ->  Result<Response, ContractError> {
    contract::migrate(deps)

}

