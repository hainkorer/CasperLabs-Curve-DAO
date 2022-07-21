#![no_std]
extern crate alloc;

pub mod data;
mod vesting_escrow;

pub use casperlabs_contract_utils;
pub use vesting_escrow::VESTINGESCROW;
