use cosmwasm_std::{
    to_json_binary, BankMsg, DepsMut, Env, HexBinary, MessageInfo, Response, WasmMsg,
};
use gear::GearMetadata;
use sha2::{digest::Update, Digest, Sha256};

use crate::{
    error::ContractError,
    msg::{CheckinParams, MintParams},
    state::{CHECKIN_COUNTS, CONFIG, NETX_CAT_ID, NETX_GEAR_ID},
};

pub fn mint(
    deps: DepsMut,
    info: MessageInfo,
    params: MintParams,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let enough_funds = info
        .funds
        .iter()
        .any(|c| c.denom == config.mint_price.denom && c.amount >= config.mint_price.amount);
    if !enough_funds {
        return Err(ContractError::NotEnoughFundsToMint);
    }

    let cat_id = NETX_CAT_ID.load(deps.storage)?;
    let next_cat_id = cat_id + 1;
    NETX_CAT_ID.save(deps.storage, &next_cat_id)?;

    let mint_msg = cat::ExecuteMsg::Mint {
        token_id: cat_id.to_string(),
        owner: info.sender.to_string(),
        token_uri: Some(format!("{}/{}", config.token_id_prefix, cat_id)),
        extension: Some(cat::CatMetadata {
            name: params.name,
            color: params.color,
            gender: params.gender,
            variety: params.variety,
            level: 1,
        }),
    };
    let mint_msg = WasmMsg::Execute {
        contract_addr: config.cat_contract.to_string(),
        msg: to_json_binary(&mint_msg)?,
        funds: vec![],
    };

    let resp = Response::new().add_message(mint_msg);

    Ok(resp)
}

pub fn checkin(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    params: CheckinParams,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let gear_id = NETX_GEAR_ID.load(deps.storage)?;
    let next_gear_id = gear_id + 1;
    NETX_GEAR_ID.save(deps.storage, &next_gear_id)?;

    let sender = info.sender;

    let cats: cw721::TokensResponse = deps.querier.query_wasm_smart(
        config.cat_contract,
        &cat::QueryMsg::Tokens {
            owner: sender.to_string(),
            start_after: None,
            limit: None,
        },
    )?;
    if cats.tokens.is_empty() {
        return Err(ContractError::NoAnyCatNft);
    }

    let has_key = CHECKIN_COUNTS.has(deps.storage, sender.to_string());
    let checkin_count = if has_key {
        CHECKIN_COUNTS.load(deps.storage, sender.to_string())?
    } else {
        0
    };

    // 验证checkin参数
    let msg_should_be = format!("{}:{}:{}", env.block.chain_id, &sender, checkin_count);
    let msg_hash_should_be = Sha256::new().chain(msg_should_be.as_str()).finalize();

    let msg_hash = HexBinary::from_hex(&params.msg_hash)?;
    if msg_hash.as_slice() != msg_hash_should_be.as_slice() {
        return Err(ContractError::Unauthorized);
    }

    let msg_sign = HexBinary::from_hex(&params.msg_signature)?;
    let valid_pubkey = HexBinary::from_hex(&config.checkin_pubkey)?;
    let is_valid_msg = deps
        .api
        .secp256k1_verify(&msg_hash, &msg_sign, &valid_pubkey)?;
    if !is_valid_msg {
        return Err(ContractError::Unauthorized);
    }

    CHECKIN_COUNTS.save(deps.storage, sender.to_string(), &(checkin_count + 1))?;

    let mint_msg = gear::ExecuteMsg::Mint {
        token_id: gear_id.to_string(),
        owner: sender.to_string(),
        token_uri: None,
        extension: Some(GearMetadata { kind: params.kind }),
    };
    let mint_msg = WasmMsg::Execute {
        contract_addr: config.gear_contract.to_string(),
        msg: to_json_binary(&mint_msg)?,
        funds: vec![],
    };

    let resp = Response::new()
        .add_message(mint_msg)
        .add_attribute("action", "checkin");

    Ok(resp)
}

pub fn withdraw(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let owner = config.owner;
    let sender = info.sender;

    if sender != owner {
        return Err(ContractError::Unauthorized);
    }

    let all_balances = deps.querier.query_all_balances(env.contract.address)?;

    let bank_msg = BankMsg::Send {
        to_address: sender.to_string(),
        amount: all_balances,
    };

    let resp = Response::new()
        .add_message(bank_msg)
        .add_attribute("action", "withdraw");

    Ok(resp)
}
