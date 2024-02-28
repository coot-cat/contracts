use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Coin;

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: String,
    pub token_id_prefix: String,
    pub mint_price: Coin,
    pub cat_code_id: u64,
    pub gear_code_id: u64,
    pub checkin_pubkey: String,
}

#[cw_serde]
pub struct MintParams {
    pub name: String,
    pub gender: u8,
    pub color: u8,
    pub variety: u8,
}

#[cw_serde]
pub struct CheckinParams {
    pub kind: u8,
    pub msg_hash: String,
    pub msg_signature: String,
}

#[cw_serde]
pub enum ExecMsg {
    Mint(MintParams),
    Checkin(CheckinParams),
    Withdraw,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(u64)]
    CheckinCount(String),
}
