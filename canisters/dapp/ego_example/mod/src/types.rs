use candid::CandidType;
use ic_cdk::export::Principal;
use serde::{Deserialize, Serialize};


#[derive(CandidType, Serialize, Deserialize, Clone)]
pub struct ExampleState {}
