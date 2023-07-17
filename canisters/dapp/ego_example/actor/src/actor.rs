// ------------------
//
// **Here are ego dependencies, needed for ego injections**
//
// ------------------
// BTreeMap
use std::collections::BTreeMap;

// ego_macros
use ego_macros::{inject_app_info_api, inject_ego_api};

// ic_cdk
use candid::candid_method;
use ic_cdk::caller;
use ic_cdk::export::Principal;
use ic_cdk_macros::*;

// ------------------
//
// **Project dependencies
//
// ------------------
// injected macros
use ego_example_mod::state::*;
use ego_example_mod::types::{Example, UserProfile, UserWallet};

// ------------------
//
// ** injections
//
// ------------------
// injection ego apis
inject_ego_api!();
inject_app_info_api!();

#[warn(unused_must_use)]
#[init]
#[candid_method(init, rename = "init")]
fn canister_init() {
    let caller = caller();
    info_log_add(format!("ego_example: init, caller is {}", caller.clone()).as_str());
    owner_add(caller);
}

#[pre_upgrade]
pub fn pre_upgrade() {
    ego_example_mod::state::pre_upgrade()
}

#[post_upgrade]
pub fn post_upgrade() {
    ego_example_mod::state::post_upgrade();
}

#[update(name = "whoAmI", guard = "owner_guard")]
#[candid_method(update, rename = "whoAmI")]
pub fn who_am_i() -> Principal {
    ic_cdk::api::caller()
}

#[update(name = "insert_user", guard = "owner_guard")]
#[candid_method(update, rename = "insert_user")]
pub fn insert_user(user_id: u16, user_name: String) -> UserProfile{
    Example::add_user(user_id, user_name)
}

#[query(name = "get_user", guard = "owner_guard")]
#[candid_method(query, rename = "get_user")]
pub fn get_user(user_id: u16) -> Option<UserProfile> {
    Example::get_user(user_id)
}

#[query(name = "get_all_users", guard = "owner_guard")]
#[candid_method(query, rename = "get_all_users")]
pub fn get_all_users() -> Vec<UserProfile> {
    info_log_add("ego_example: get_all_users");
    Example::get_all_users()
}

#[update(name = "insert_wallet", guard = "owner_guard")]
#[candid_method(update, rename = "insert_wallet")]
pub fn insert_wallet(user_id: u16, balance: u32) -> UserWallet{
    Example::add_wallet(user_id, balance)
}

#[query(name = "get_wallet", guard = "owner_guard")]
#[candid_method(query, rename = "get_wallet")]
pub fn get_wallet(user_id: u16) -> Option<UserWallet> {
    Example::get_wallet(user_id)
}

#[query(name = "get_all_wallets", guard = "owner_guard")]
#[candid_method(query, rename = "get_all_wallets")]
pub fn get_all_wallets() -> Vec<UserWallet> {
    info_log_add("ego_example: get_all_wallets");
    Example::get_all_wallets()
}
