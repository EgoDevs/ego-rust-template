

mod actor;

#[allow(dead_code)]
#[cfg(any(target_arch = "wasm32", test))]
fn main() {}

#[allow(dead_code)]
#[cfg(not(any(target_arch = "wasm32", test)))]
fn main() {
    use ego_types::app::{AppId, Version};
    use ego_types::app_info::AppInfo;
    use ego_example_mod::types::*;
    use ic_cdk::export::Principal;
    use std::collections::BTreeMap;

    candid::export_service!();
    std::print!("{}", __export_service());
}
