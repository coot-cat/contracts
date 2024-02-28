use cosmwasm_std::{Addr, Coin};
use cw_multi_test::{error::AnyResult, App, ContractWrapper, Executor};

use crate::msg::{CheckinParams, ExecMsg, InstantiateMsg, MintParams, QueryMsg};
use crate::{execute, instantiate, query, reply};

pub struct CodeId(u64);

impl CodeId {
    pub fn store_code(app: &mut App) -> Self {
        let contract = ContractWrapper::new(execute, instantiate, query).with_reply(reply);

        CodeId(app.store_code(Box::new(contract)))
    }

    pub fn id(&self) -> u64 {
        self.0
    }
}

#[derive(Debug)]
pub struct Contract(Addr);

impl Contract {
    #[allow(clippy::too_many_arguments)]
    #[track_caller]
    pub fn instantiate(
        app: &mut App,
        code_id: u64,
        owner: String,
        token_id_prefix: String,
        mint_price: Coin,
        cat_code_id: u64,
        gear_code_id: u64,
        checkin_pubkey: String,
        sender: Addr,
        admin: impl Into<Option<String>>,
    ) -> AnyResult<Self> {
        let msg = InstantiateMsg {
            token_id_prefix,
            owner,
            mint_price,
            cat_code_id,
            gear_code_id,
            checkin_pubkey,
        };

        app.instantiate_contract(
            code_id,
            sender,
            &msg,
            &[],
            "CoolCat game contract",
            admin.into(),
        )
        .map(Self)
    }

    #[track_caller]
    pub fn addr(&self) -> Addr {
        self.0.clone()
    }

    #[allow(clippy::too_many_arguments)]
    #[track_caller]
    pub fn checkin(
        &self,
        app: &mut App,
        sender: &str,
        funds: &[Coin],
        gear_kind: u8,
        msg_hash: String,
        signature: String,
    ) -> AnyResult<()> {
        app.execute_contract(
            Addr::unchecked(sender),
            self.0.clone(),
            &ExecMsg::Checkin(CheckinParams {
                kind: gear_kind,
                msg_hash,
                msg_signature: signature,
            }),
            funds,
        )?;
        Ok(())
    }

    #[track_caller]
    pub fn query_checkin_count(&self, app: &App, addr: String) -> AnyResult<u64> {
        let result: u64 = app
            .wrap()
            .query_wasm_smart(self.0.clone(), &QueryMsg::CheckinCount(addr))?;

        Ok(result)
    }

    #[allow(clippy::too_many_arguments)]
    #[track_caller]
    pub fn mint(
        &self,
        app: &mut App,
        sender: &str,
        funds: &[Coin],
        name: String,
        gender: u8,
        color: u8,
        variety: u8,
    ) -> AnyResult<()> {
        app.execute_contract(
            Addr::unchecked(sender),
            self.0.clone(),
            &ExecMsg::Mint(MintParams {
                name,
                gender,
                color,
                variety,
            }),
            funds,
        )?;
        Ok(())
    }

    pub fn withdraw(&self, app: &mut App, sender: &str) -> AnyResult<()> {
        app.execute_contract(
            Addr::unchecked(sender),
            self.0.clone(),
            &ExecMsg::Withdraw,
            &[],
        )?;
        Ok(())
    }
}
