use crate::types::*;
use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    DefaultMemoryImpl, Memory as MemoryTrait, StableBTreeMap,
};
use std::cell::RefCell;

const WASM_PAGE_SIZE: u64 = 65536;
const UPGRADES: MemoryId = MemoryId::new(0);
const BTREE_ID: MemoryId = MemoryId::new(1);

type InnerMemory = DefaultMemoryImpl;

pub type Memory = VirtualMemory<InnerMemory>;

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));
    /// We don't want everything store in one big state, so we decouple memory for each static global vars.
    static BTREES: RefCell<StableBTreeMap<BtreeKey, BtreeValue, Memory>> = RefCell::new(StableBTreeMap::init(get_btree_memory()));
}

/// Get the memory for upgrades.
/// This memory is used to store the wasm module of the canister.
pub fn get_btree_memory() -> Memory {
    MEMORY_MANAGER.with(|m| m.borrow().get(BTREE_ID))
}
pub fn insert_btree(key: String, value: BtreeValue) {
    BTREES.with(|m| m.borrow_mut().insert(BtreeKey(key), value));
}
pub fn get_btree(address_string: String) -> Option<BtreeValue> {
    BTREES.with(|m| m.borrow().get(&BtreeKey(address_string)))
}
pub fn get_all_btree() -> Vec<BtreeValue> {
    BTREES.with(|m| {
        m.borrow()
            .iter()
            .map(|f| f.1.clone())
            .collect::<Vec<BtreeValue>>()
    })
}

/// Get the memory for upgrades.
/// This memory is used to store the wasm module of the canister.
pub fn get_upgrades_memory() -> Memory {
    MEMORY_MANAGER.with(|m| m.borrow().get(UPGRADES))
}

/// Write memory and increase memory size if necessary.
/// use page size 64kb
pub fn write<M: MemoryTrait>(memory: &M, offset: u64, bytes: &[u8]) {
    let last_byte = offset
        .checked_add(bytes.len() as u64)
        .expect("Address space overflow");

    let size_pages = memory.size();
    let size_bytes = size_pages
        .checked_mul(WASM_PAGE_SIZE)
        .expect("Address space overflow");

    if size_bytes < last_byte {
        let diff_bytes = last_byte - size_bytes;
        let diff_pages = diff_bytes
            .checked_add(WASM_PAGE_SIZE - 1)
            .expect("Address space overflow")
            / WASM_PAGE_SIZE;
        if memory.grow(diff_pages) == -1 {
            panic!(
                "Failed to grow memory from {} pages to {} pages (delta = {} pages).",
                size_pages,
                size_pages + diff_pages,
                diff_pages
            );
        }
    }
    memory.write(offset, bytes);
}
