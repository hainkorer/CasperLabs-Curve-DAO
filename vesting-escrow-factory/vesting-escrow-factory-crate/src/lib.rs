#![no_std]
extern crate alloc;

pub mod data;
mod vesting_escrow_factory;
pub mod event;

pub use contract_utils;
pub use vesting_escrow_factory::{Error, VESTINGESCROWFACTORY};

use alloc::{collections::BTreeMap, string::String};
use casper_types::U256;
pub type TokenId = U256;
pub type Meta = BTreeMap<String, String>;
