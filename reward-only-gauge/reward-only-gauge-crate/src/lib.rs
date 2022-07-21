#![no_std]
extern crate alloc;

pub mod data;
mod reward_only_gauge;

pub use casperlabs_contract_utils;
pub use reward_only_gauge::REWARDONLYGAUGE;
