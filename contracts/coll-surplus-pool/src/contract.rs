use std::vec;

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    coin, to_binary, Addr, BankMsg, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError,
    StdResult, Storage, Uint128,
};

use cw2::set_contract_version;
use ultra_base::role_provider::Role;
use ultra_base::coll_surplus_pool::{ExecuteMsg, InstantiateMsg, ParamsResponse, QueryMsg};

use crate::error::ContractError;
use crate::state::{
    SudoParams, TotalCollsInPool, COLL_OF_ACCOUNT, SUDO_PARAMS, TOTAL_COLLS_IN_POOL, ADMIN, ROLE_CONSUMER,
};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:active-pool";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub const NATIVE_JUNO_DENOM: &str = "ujuno";

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    mut deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    // set admin so that only admin can access to update role function
    ADMIN.set(deps.branch(), Some(info.sender))?;
    // store sudo params
    let sudo_params = SudoParams {
        name: msg.name,
        owner: deps.api.addr_validate(&msg.owner)?,
    };

    // initial assets in pool
    let assets_in_pool = TotalCollsInPool {
        juno: Uint128::zero(),
    };

    SUDO_PARAMS.save(deps.storage, &sudo_params)?;
    TOTAL_COLLS_IN_POOL.save(deps.storage, &assets_in_pool)?;

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::UpdateAdmin { admin } => {
            Ok(ADMIN.execute_update_admin(deps, info, Some(admin))?)
        }
        ExecuteMsg::UpdateRole { role_provider } => {
            execute_update_role(deps, env, info, role_provider)
        }
        ExecuteMsg::AccountSurplus { account, amount } => {
            execute_account_surplus(deps, env, info, account, amount)
        }
        ExecuteMsg::ClaimColl { account } => execute_claim_coll(deps, env, info, account),
    }
}

pub fn execute_update_role(
    deps: DepsMut, 
    _env: Env,
    info: MessageInfo,
    role_provider: Addr
) -> Result<Response, ContractError> {
    ADMIN.assert_admin(deps.as_ref(), &info.sender)?;
    ROLE_CONSUMER.add_role_provider(deps.storage, role_provider.clone())?;

    let res = Response::new()
        .add_attribute("action", "update_role")
        .add_attribute("role_provider_addr", role_provider);
    Ok(res)
}

pub fn execute_account_surplus(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    account: Addr,
    amount: Uint128,
) -> Result<Response, ContractError> {
    ROLE_CONSUMER
        .assert_role(deps.as_ref(), &info.sender, vec![Role::TroveManager])?;

    let mut coll_of_account = COLL_OF_ACCOUNT.load(deps.storage, account.clone())?;
    coll_of_account += amount;
    COLL_OF_ACCOUNT.save(deps.storage, account.clone(), &coll_of_account)?;

    let res = Response::new()
        .add_attribute("action", "account_surplus")
        .add_attribute("account", account);
    Ok(res)
}

pub fn execute_claim_coll(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    account: Addr,
) -> Result<Response, ContractError> {
    ROLE_CONSUMER
        .assert_role(deps.as_ref(), &info.sender, vec![Role::BorrowerOperations])?;

    let mut coll_of_account = COLL_OF_ACCOUNT.load(deps.storage, account.clone())?;
    let mut total_colls_in_pool = TOTAL_COLLS_IN_POOL.load(deps.storage)?;

    if coll_of_account.is_zero() {
        return Err(ContractError::NoCollAvailableToClaim {});
    }
    let send_msg = BankMsg::Send {
        to_address: account.to_string(),
        amount: vec![coin(coll_of_account.u128(), NATIVE_JUNO_DENOM.to_string())],
    };

    total_colls_in_pool.juno = total_colls_in_pool
        .juno
        .checked_sub(coll_of_account)
        .map_err(StdError::overflow)?;
    coll_of_account = Uint128::zero();

    COLL_OF_ACCOUNT.save(deps.storage, account.clone(), &coll_of_account)?;
    TOTAL_COLLS_IN_POOL.save(deps.storage, &total_colls_in_pool)?;

    let res = Response::new()
        .add_message(send_msg)
        .add_attribute("action", "claim_coll")
        .add_attribute("account", account);
    Ok(res)
}

/// Checks to enfore only owner can call
fn only_owner(store: &dyn Storage, info: &MessageInfo) -> Result<Addr, ContractError> {
    let params = SUDO_PARAMS.load(store)?;
    if params.owner != info.sender.as_ref() {
        return Err(ContractError::UnauthorizedOwner {});
    }
    Ok(info.sender.clone())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetParams {} => to_binary(&query_params(deps)?),
        QueryMsg::GetJUNO {} => to_binary(&query_juno_state(deps)?),
        QueryMsg::GetCollateral { account } => to_binary(&query_coll_of_account(deps, account)?),
    }
}

pub fn query_juno_state(deps: Deps) -> StdResult<Uint128> {
    let info = TOTAL_COLLS_IN_POOL.load(deps.storage)?;
    let res = info.juno;
    Ok(res)
}

pub fn query_coll_of_account(deps: Deps, account: Addr) -> StdResult<Uint128> {
    let info = COLL_OF_ACCOUNT.load(deps.storage, account)?;
    Ok(info)
}

pub fn query_params(deps: Deps) -> StdResult<ParamsResponse> {
    let info = SUDO_PARAMS.load(deps.storage)?;
    let res = ParamsResponse {
        name: info.name,
        owner: info.owner,
    };
    Ok(res)
}
