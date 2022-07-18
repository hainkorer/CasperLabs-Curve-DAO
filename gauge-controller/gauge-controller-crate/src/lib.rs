#![no_std]
extern crate alloc;

pub mod data;
mod gauge_controller;

pub use casperlabs_contract_utils;
pub use gauge_controller::GAUGECONLTROLLER;
