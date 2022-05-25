#![no_std]

extern crate alloc;

pub mod data;
pub mod event;
mod liquidity_gauge_wrapper;

pub use liquidity_gauge_wrapper::LIQUIDITYGAUGEWRAPPER;
