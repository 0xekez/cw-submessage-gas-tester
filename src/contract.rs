#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdResult, SubMsg, WasmMsg,
};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    Ok(Response::default().add_attribute("method", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Count { number } => Ok(if number != 0 {
            Response::default()
                .add_message(WasmMsg::Execute {
                    contract_addr: env.contract.address.into_string(),
                    msg: to_binary(&ExecuteMsg::CountInSubMsg { number: number - 2 })?,
                    funds: vec![],
                })
                .add_attribute("count", format!("{}", number - 2))
        } else {
            Response::default()
        }),
        ExecuteMsg::CountInSubMsg { number } => Ok(Response::default()
            .add_submessage(SubMsg::reply_on_error(
                WasmMsg::Execute {
                    contract_addr: env.contract.address.into_string(),
                    msg: to_binary(&ExecuteMsg::Count { number: number + 1 })?,
                    funds: vec![],
                },
                0,
            ))
            .add_attribute("count", format!("{}", number + 1))),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(_deps: DepsMut, _env: Env, _msg: Reply) -> Result<Response, ContractError> {
    Ok(Response::default().add_attribute("method", "reply"))
}
