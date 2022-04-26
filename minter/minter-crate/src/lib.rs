#![no_std]
extern crate alloc;

pub mod data;
mod minter;
pub mod event;

pub use contract_utils;
pub use minter::{Error, MINTER};

use alloc::{collections::BTreeMap, string::String};
use casper_types::U256;
pub type TokenId = U256;
pub type Meta = BTreeMap<String, String>;
