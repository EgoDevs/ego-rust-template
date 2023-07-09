use candid::CandidType;
use ic_stable_structures::{BoundedStorable, Storable};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// We define a example key with String
/// because String is expandable, cannot store in stable structure directly
/// so we use a struct to wrap it.
#[derive(CandidType, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq, Clone)]
pub struct BtreeKey(pub String);

impl Storable for BtreeKey {
    fn to_bytes(&self) -> Cow<[u8]> {
        self.0.to_bytes()
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Self(String::from_bytes(bytes))
    }
}

impl BoundedStorable for BtreeKey {
    // 128 for example
    const MAX_SIZE: u32 = 128;
    const IS_FIXED_SIZE: bool = false;
}

#[derive(CandidType, Serialize, Deserialize, Clone)]
pub struct BtreeValue {
    /// key is expandable
    /// but we have to give it a boundary
    /// say 128 bytes
    pub key: String,
    /// value is expandable
    /// but we have to give it a boundary
    /// say 896 bytes
    pub value: Vec<u8>,
}

impl Storable for BtreeValue {
    // serialize the struct to bytes
    fn to_bytes(&self) -> Cow<[u8]> {
        candid::encode_one::<&BtreeValue>(self)
            .expect("Error: Candid Serializing BtreeValue")
            .into()
    }

    // deserialize the btyes to struct
    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        candid::decode_one::<BtreeValue>(bytes.as_ref())
            .expect("Error: Candid DeSerializing BtreeValue")
    }
}

impl BoundedStorable for BtreeValue {
    /// 128 + 896 = 640
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}
