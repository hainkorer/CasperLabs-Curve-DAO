#![no_std]
extern crate alloc;

pub mod data;
pub mod event;
mod gauge_controller;

pub use casperlabs_contract_utils;
pub use gauge_controller::GAUGECONLTROLLER;

use alloc::{collections::BTreeMap, string::String};
use casper_types::U256;
pub type TokenId = U256;
pub type Meta = BTreeMap<String, String>;
