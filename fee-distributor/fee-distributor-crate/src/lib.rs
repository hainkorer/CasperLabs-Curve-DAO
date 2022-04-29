#![no_std]

extern crate alloc;

pub mod data;
pub mod event;
mod fee_distributor;

pub use fee_distributor::FEEDISTRIBUTOR;
