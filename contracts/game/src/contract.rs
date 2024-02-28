use cosmwasm_std::{to_json_binary, Addr, DepsMut, Env, Reply, Response, SubMsg, WasmMsg};
use cw2::set_contract_version;

use crate::{
    error::ContractError,
    msg::InstantiateMsg,
    state::{Config, CONFIG},
};

const CONTRACT_NAME: &str = "coolcat-game-contract";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

const INIT_CAT_CONTRACT_REPLY_ID: u64 = 1;
// const INIT_GEAR_CONTRACT_REPLY_ID: u64 = 2;

pub fn instantiate(
    deps: DepsMut,
    env: Env,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let owner = deps.api.addr_validate(&msg.owner)?;
    let cat_contract = Addr::unchecked("");
    let gear_contract = Addr::unchecked("");

    let config = Config {
        owner,
        cat_contract,
        gear_contract,
    };

    CONFIG.save(deps.storage, &config)?;

    // Cat Init
    let cat_init_msg = cw721_base::msg::InstantiateMsg {
        minter: env.contract.address.to_string(),
        name: "CoolCat".to_string(),
        symbol: "CAT".to_string(),
    };
    let cat_init_msg = WasmMsg::Instantiate {
        admin: None,
        code_id: msg.cat_code_id,
        msg: to_json_binary(&cat_init_msg)?,
        funds: vec![],
        label: "CoolCat contract".to_string(),
    };
    let cat_init_msg = SubMsg::reply_on_success(cat_init_msg, INIT_CAT_CONTRACT_REPLY_ID);

    // Gear Init
    // let gear_init_msg = cw721_base::msg::InstantiateMsg {
    //     minter: env.contract.address.to_string(),
    //     name: "CoolCatGear".to_string(),
    //     symbol: "GEAR".to_string(),
    // };
    // let gear_init_msg = WasmMsg::Instantiate {
    //     admin: None,
    //     code_id: msg.gear_code_id,
    //     msg: to_json_binary(&gear_init_msg)?,
    //     funds: vec![],
    //     label: "CoolCat Gear contract".to_string(),
    // };
    // let gear_init_msg = SubMsg::reply_on_success(gear_init_msg, INIT_GEAR_CONTRACT_REPLY_ID);

    let resp = Response::new().add_submessages(vec![cat_init_msg]);
    Ok(resp)
}

pub fn reply(deps: DepsMut, env: Env, reply: Reply) -> Result<Response, ContractError> {
    println!("game reply");
    match reply.id {
        INIT_CAT_CONTRACT_REPLY_ID => Ok(Response::new()),
        id => Err(ContractError::UnrecognizedReplyId(id)),
    }
}
