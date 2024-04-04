use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    DefaultMemoryImpl, RestrictedMemory, StableBTreeMap, StableCell,
};

use crate::types::*;
use std::cell::RefCell;

const USER_PROFILE_MEM_ID: MemoryId = MemoryId::new(0);
const USER_WALLET_MEM_ID: MemoryId = MemoryId::new(1);
const METADATA_PAGES: u64 = 16;

type VM = VirtualMemory<DefaultMemoryImpl>;

thread_local! {
   pub static CONFIG:RefCell<StableState> = RefCell::new(StableState::default());

    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    pub static USERS: RefCell<StableBTreeMap<u16, UserProfile, VM>> = MEMORY_MANAGER.with(|mm| {
        RefCell::new(StableBTreeMap::init(mm.borrow().get(USER_PROFILE_MEM_ID)))
    });

    pub static WALLETS: RefCell<StableBTreeMap<u16, UserWallet, VM>> = MEMORY_MANAGER.with(|mm| {
        RefCell::new(StableBTreeMap::init(mm.borrow().get(USER_WALLET_MEM_ID)))
    });
}
