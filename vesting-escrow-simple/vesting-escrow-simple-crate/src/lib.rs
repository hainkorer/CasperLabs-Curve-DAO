#![no_std]

extern crate alloc;

pub mod data;
pub mod entry_points;
mod vesting_escrow_simple;

pub use vesting_escrow_simple::VESTINGESCROWSIMPLE;
