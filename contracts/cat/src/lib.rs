#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    to_json_binary, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult,
};
pub use cw721_base::{ContractError, InstantiateMsg, MinterResponse};

// Version info for migration
const CONTRACT_NAME: &str = "coolcat-cat-contract";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

// see: https://docs.opensea.io/docs/metadata-standards
#[cw_serde]
#[derive(Default)]
pub struct CatMetadata {
    pub name: String,
    pub gender: u8,
    pub color: u8,
    pub variety: u8,
    pub level: u16,
}

pub type Extension = Option<CatMetadata>;

pub type Cw721MetadataContract<'a> = cw721_base::Cw721Contract<'a, Extension, Empty, Empty, Empty>;
pub type ExecuteMsg = cw721_base::ExecuteMsg<Extension, Empty>;
pub type QueryMsg = cw721_base::QueryMsg<Empty>;

#[cfg(any(feature = "mt", test))]
pub mod multitest;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    let contract_addr = to_json_binary(&env.contract.address.to_string())?;

    Cw721MetadataContract::default()
        .instantiate(deps, env, info, msg)
        .map(|resp| resp.set_data(contract_addr))
        .map_err(ContractError::Std)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    Cw721MetadataContract::default().execute(deps, env, info, msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    Cw721MetadataContract::default().query(deps, env, msg)
}

#[cfg(test)]
mod tests {
    use super::*;

    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cw721::Cw721Query;

    const CREATOR: &str = "creator";

    /// Make sure cw2 version info is properly initialized during instantiation,
    /// and NOT overwritten by the base contract.
    #[test]
    fn proper_cw2_initialization() {
        let mut deps = mock_dependencies();

        instantiate(
            deps.as_mut(),
            mock_env(),
            mock_info("larry", &[]),
            InstantiateMsg {
                name: "".into(),
                symbol: "".into(),
                minter: CREATOR.to_string(),
            },
        )
        .unwrap();

        let version = cw2::get_contract_version(deps.as_ref().storage).unwrap();
        assert_eq!(version.contract, CONTRACT_NAME);
        assert_ne!(version.contract, cw721_base::CONTRACT_NAME);
    }

    #[test]
    fn use_metadata_extension() {
        let mut deps = mock_dependencies();
        let contract = Cw721MetadataContract::default();

        let info = mock_info(CREATOR, &[]);
        let init_msg = InstantiateMsg {
            name: "Cat".to_string(),
            symbol: "CAT".to_string(),
            minter: CREATOR.to_string(),
        };
        contract
            .instantiate(deps.as_mut(), mock_env(), info.clone(), init_msg)
            .unwrap();

        let token_id = "1";
        let extension = Some(CatMetadata {
            name: "Kitty".to_string(),
            ..CatMetadata::default()
        });
        let exec_msg = ExecuteMsg::Mint {
            token_id: token_id.to_string(),
            owner: "john".to_string(),
            token_uri: None,
            extension: extension.clone(),
        };
        contract
            .execute(deps.as_mut(), mock_env(), info, exec_msg)
            .unwrap();

        let res = contract.nft_info(deps.as_ref(), token_id.into()).unwrap();
        assert_eq!(res.token_uri, None);
        assert_eq!(res.extension, extension);
    }
}
