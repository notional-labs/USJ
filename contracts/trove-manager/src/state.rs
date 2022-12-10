use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use ultra_base::trove_manager::Status;
use ultra_controllers::roles::RoleConsumer;

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct Trove {
    pub juno: Uint128,
    pub ultra_debt: Uint128,
    pub stake: Uint128,
    pub status: Status,
}


#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct SudoParams {
    pub name: String,
    pub owner: Addr,
}

pub struct State<'a> {
    pub roles: RoleConsumer<'a>,
}

impl<'a> Default for State<'a> {
    fn default() -> Self {
        State {
            roles: RoleConsumer::new("role_provider_address"),
        }
    }
}

pub const SUDO_PARAMS: Item<SudoParams> = Item::new("sudo-params");
pub const TROVES: Map<Addr, Trove> = Map::new("troves");