use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    Count { number: u32 },
    CountInSubMsg { number: u32 },
}

#[cw_serde]
pub enum QueryMsg {}
