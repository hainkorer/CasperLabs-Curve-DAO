#![no_std]
extern crate alloc;

pub mod data;
pub mod event;
mod reward_only_gauge;

pub use contract_utils;
pub use reward_only_gauge::{Error, REWARDONLYGAUGE};

use alloc::{collections::BTreeMap, string::String};
use casper_types::U256;
pub type TokenId = U256;
pub type Meta = BTreeMap<String, String>;
