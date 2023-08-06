pub mod actor;
pub mod memory;
pub mod state;
pub mod types;

use crate::types::*;
use candid::Principal;
use ego_types::app::{AppId, Version};
use ego_types::app_info::AppInfo;
use std::collections::BTreeMap;

candid::export_service!();

#[no_mangle]
pub fn get_candid_pointer() -> *mut std::os::raw::c_char {
    let c_string = std::ffi::CString::new(__export_service()).unwrap();

    c_string.into_raw()
}
