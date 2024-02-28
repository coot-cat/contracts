use cosmwasm_std::{to_json_binary, Binary, Deps};

use crate::{error::ContractError, state::CHECKIN_COUNTS};

pub fn checkin_count(deps: Deps, addr: String) -> Result<Binary, ContractError> {
    let has_key = CHECKIN_COUNTS.has(deps.storage, addr.clone());

    let count = if has_key {
        CHECKIN_COUNTS.load(deps.storage, addr)?
    } else {
        0
    };
    let result = to_json_binary(&count)?;
    Ok(result)
}
