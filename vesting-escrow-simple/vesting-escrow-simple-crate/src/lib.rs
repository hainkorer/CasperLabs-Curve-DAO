#![no_std]

extern crate alloc;

pub mod data;
mod vesting_escrow_simple;
pub mod entry_points;

pub use vesting_escrow_simple::VESTINGESCROWSIMPLE;
