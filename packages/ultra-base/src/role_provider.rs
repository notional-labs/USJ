
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    ActivePool,
    TroveManager,
    Owner,
    StabilityPool,
    BorrowerOperations,
}

impl ToString for Role {
    fn to_string(&self) -> String {
        match &self {
            Role::ActivePool => "active_pool",
            Role::TroveManager => "trove_manager",
            Role::Owner => "owner",
            Role::StabilityPool => "stability_pool",
            Role::BorrowerOperations => "borrower_operations",
        }
        .into()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct InstantiateMsg {
    pub active_pool: String,
    pub trove_manager: String,
    pub owner: String,
    pub stability_pool: String,
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
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct HasAnyRoleResponse {
    pub has_role: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct RoleAddressResponse {
    pub address: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum MigrateMsg {}
