#![no_std]

extern crate alloc;

pub mod data;
pub mod event;
mod gauge_proxy;

pub use gauge_proxy::GAUGEPROXY;
