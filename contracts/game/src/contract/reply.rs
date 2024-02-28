use cosmwasm_std::{DepsMut, Reply, Response};

use crate::{
    error::ContractError,
    state::{Config, CONFIG},
};

pub enum CatOrGear {
    Cat,
    Gear,
}

pub fn sub_contract_inited(
    deps: DepsMut,
    reply: Reply,
    cat_or_gear: CatOrGear,
) -> Result<Response, ContractError> {
    let resp = cw_utils::parse_reply_instantiate_data(reply)?;
    let contract_addr = deps.api.addr_validate(&resp.contract_address)?;

    let config = CONFIG.load(deps.storage)?;
    let config = match cat_or_gear {
        CatOrGear::Cat => Config {
            cat_contract: contract_addr,
            ..config
        },
        CatOrGear::Gear => Config {
            gear_contract: contract_addr,
            ..config
        },
    };

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new())
}
