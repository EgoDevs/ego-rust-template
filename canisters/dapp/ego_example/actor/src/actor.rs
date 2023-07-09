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
// types
use ego_example_mod::types::*;

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

#[update(name = "insert_btree", guard = "owner_guard")]
#[candid_method(update, rename = "insert_btree")]
pub fn insert_btree(key: String, value: BtreeValue) {
    ego_example_mod::memory::insert_btree(key, value)
}

#[query(name = "get_btree", guard = "owner_guard")]
#[candid_method(query, rename = "get_btree")]
pub fn get_btree(key: String) -> Option<BtreeValue> {
    ego_example_mod::memory::get_btree(key)
}

#[query(name = "get_all_btree", guard = "owner_guard")]
#[candid_method(query, rename = "get_all_btree")]
pub fn get_all_btree() -> Vec<BtreeValue> {
    ego_example_mod::memory::get_all_btree()
}
