#![no_std]
extern crate alloc;

pub mod data;
mod child_streamer;
pub mod event;

pub use contract_utils;
pub use child_streamer::{Error, CHILDSTREAMER};

use alloc::{collections::BTreeMap, string::String};
use casper_types::U256;
pub type TokenId = U256;
pub type Meta = BTreeMap<String, String>;
