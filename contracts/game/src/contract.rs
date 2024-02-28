use cosmwasm_std::{
    to_json_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, SubMsg, WasmMsg,
};
use cw2::set_contract_version;

use crate::{
    error::ContractError,
    msg::{ExecMsg, InstantiateMsg, QueryMsg},
    state::{Config, CONFIG, NETX_CAT_ID, NETX_GEAR_ID},
};

use self::reply::CatOrGear;

mod exec;
mod query;
mod reply;

const CONTRACT_NAME: &str = "coolcat-game-contract";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

const INIT_CAT_CONTRACT_REPLY_ID: u64 = 1;
const INIT_GEAR_CONTRACT_REPLY_ID: u64 = 2;

pub fn instantiate(
    deps: DepsMut,
    env: Env,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let owner = deps.api.addr_validate(&msg.owner)?;
    let token_id_prefix = msg.token_id_prefix;
    let mint_price = msg.mint_price;
    let cat_contract = Addr::unchecked("");
    let gear_contract = Addr::unchecked("");
    let checkin_pubkey = msg.checkin_pubkey;

    let config = Config {
        token_id_prefix,
        owner,
        mint_price,
        cat_contract,
        gear_contract,
        checkin_pubkey,
    };

    CONFIG.save(deps.storage, &config)?;
    NETX_CAT_ID.save(deps.storage, &1)?;
    NETX_GEAR_ID.save(deps.storage, &1)?;
    let contract_addr = env.contract.address.to_string();

    // Cat Init
    let cat_init_msg = cw721_base::msg::InstantiateMsg {
        minter: env.contract.address.to_string(),
        name: "CoolCat".to_string(),
        symbol: "CAT".to_string(),
    };
    let cat_init_msg = WasmMsg::Instantiate {
        admin: Some(contract_addr.clone()),
        code_id: msg.cat_code_id,
        msg: to_json_binary(&cat_init_msg)?,
        funds: vec![],
        label: "CoolCat Cat contract".to_string(),
    };
    let cat_init_msg = SubMsg::reply_on_success(cat_init_msg, INIT_CAT_CONTRACT_REPLY_ID);

    // Gear Init
    let gear_init_msg = cw721_base::msg::InstantiateMsg {
        minter: env.contract.address.to_string(),
        name: "CoolCatGear".to_string(),
        symbol: "GEAR".to_string(),
    };
    let gear_init_msg = WasmMsg::Instantiate {
        admin: Some(contract_addr),
        code_id: msg.gear_code_id,
        msg: to_json_binary(&gear_init_msg)?,
        funds: vec![],
        label: "CoolCat Gear contract".to_string(),
    };
    let gear_init_msg = SubMsg::reply_on_success(gear_init_msg, INIT_GEAR_CONTRACT_REPLY_ID);

    let resp = Response::new().add_submessages(vec![cat_init_msg, gear_init_msg]);
    Ok(resp)
}

pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecMsg::Mint(params) => exec::mint(deps, info, params),
        ExecMsg::Checkin(params) => exec::checkin(deps, env, info, params),
        ExecMsg::Withdraw => exec::withdraw(deps, env, info),
    }
}

pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    match msg {
        QueryMsg::CheckinCount(addr) => query::checkin_count(deps, addr),
    }
}

pub fn reply(deps: DepsMut, _env: Env, reply: Reply) -> Result<Response, ContractError> {
    match reply.id {
        INIT_CAT_CONTRACT_REPLY_ID => reply::sub_contract_inited(deps, reply, CatOrGear::Cat),
        INIT_GEAR_CONTRACT_REPLY_ID => reply::sub_contract_inited(deps, reply, CatOrGear::Gear),
        id => Err(ContractError::UnrecognizedReplyId(id)),
    }
}
