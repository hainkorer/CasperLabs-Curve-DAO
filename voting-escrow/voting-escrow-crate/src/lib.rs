#![no_std]

extern crate alloc;

pub mod data;
pub mod event;
mod voting_escrow;

pub use voting_escrow::VOTINGESCROW;
