use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: String,
    pub cat_code_id: u64,
    pub gear_code_id: u64,
}
