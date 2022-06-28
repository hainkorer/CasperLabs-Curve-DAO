#![no_std]
extern crate alloc;

pub mod data;
pub mod event;
mod vesting_escrow;

pub use casperlabs_contract_utils;
pub use vesting_escrow::VESTINGESCROW;

use alloc::{collections::BTreeMap, string::String};
use casper_types::U256;
pub type TokenId = U256;
pub type Meta = BTreeMap<String, String>;
