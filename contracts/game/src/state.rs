use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin};
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct Config {
    pub owner: Addr,
    pub checkin_pubkey: String,
    pub token_id_prefix: String,
    pub mint_price: Coin,
    pub cat_contract: Addr,
    pub gear_contract: Addr,
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const NETX_CAT_ID: Item<u64> = Item::new("next_cat_id");
pub const NETX_GEAR_ID: Item<u64> = Item::new("next_gear_id");
pub const CHECKIN_COUNTS: Map<String, u64> = Map::new("checkin_counts");
