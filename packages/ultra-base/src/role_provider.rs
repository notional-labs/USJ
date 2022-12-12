use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::slice::Iter;

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    ActivePool,
    TroveManager,
    Owner,
    StabilityPool,
    BorrowerOperations,
    DefaultPool,
    CollateralSurplusPool,
    UltraToken,
    PriceFeed,
    SortedTroves,
    RewardPool,
}

impl ToString for Role {
    fn to_string(&self) -> String {
        match &self {
            Role::ActivePool => "active_pool",
            Role::TroveManager => "trove_manager",
            Role::Owner => "owner",
            Role::StabilityPool => "stability_pool",
            Role::BorrowerOperations => "borrower_operations",
            Role::DefaultPool => "default_pool",
            Role::CollateralSurplusPool => "collateral_surplus_pool",
            Role::UltraToken => "ultra_token",
            Role::PriceFeed => "price_feed",
            Role::SortedTroves => "sorted_troves",
            Role::RewardPool => "reward_pool",
        }
        .into()
    }
}

impl Role {
    pub fn iterator() -> Iter<'static, Role> {
        static ROLES: [Role; 11] = [Role::ActivePool , Role::TroveManager, Role::Owner, 
            Role::StabilityPool, Role::BorrowerOperations, Role::DefaultPool, 
            Role::CollateralSurplusPool, Role::UltraToken, Role::PriceFeed,
            Role::SortedTroves, Role::RewardPool];
        ROLES.iter()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct InstantiateMsg {
    pub active_pool: String,
    pub trove_manager: String,
    pub owner: String,
    pub stability_pool: String,
    pub borrower_operations: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    UpdateRole { role: Role, address: String },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg<Role> {
    HasAnyRole { address: String, roles: Vec<Role> },
    RoleAddress { role: Role },
    AllRoles {}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct HasAnyRoleResponse {
    pub has_role: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct RoleAddressResponse {
    pub address: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct AllRolesResponse {
    pub roles: Vec<(Role, Option<String>)>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum MigrateMsg {}
