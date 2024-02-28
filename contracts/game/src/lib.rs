#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Empty, Env, MessageInfo, Reply, Response};
use error::ContractError;
use msg::InstantiateMsg;

mod contract;
mod error;
mod msg;
mod state;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    contract::instantiate(deps, env, msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: Empty,
) -> Result<Response, ContractError> {
    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, env: Env, reply: Reply) -> Result<Response, ContractError> {
    contract::reply(deps, env, reply)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: Empty) -> Result<Binary, ContractError> {
    Ok(Binary::default())
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::Addr;
    use cw_multi_test::{App, ContractWrapper, Executor};

    const ADMIN: &str = "admin";
    const SENDER: &str = "sender";
    const OWNER: &str = "owner";

    use crate::{execute, instantiate, msg::InstantiateMsg, query, reply};

    #[test]
    fn game_instantiate() {
        let contract = ContractWrapper::new(execute, instantiate, query).with_reply(reply);
        let mut app = App::default();
        let code_id = app.store_code(Box::new(contract));
        let cat_code_id = cat::multitest::CodeId::store_code(&mut app);

        let msg = InstantiateMsg {
            owner: OWNER.to_string(),
            cat_code_id: cat_code_id.id(),
            gear_code_id: 3,
        };

        let _contract_addr = app
            .instantiate_contract(
                code_id,
                Addr::unchecked(SENDER),
                &msg,
                &[],
                "CoolCat game contract",
                Some(ADMIN.to_string()),
            )
            .unwrap();
    }
}
