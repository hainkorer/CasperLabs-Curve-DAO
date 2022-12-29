#![no_std]

extern crate alloc;

pub mod data;
mod lp_token_wrapper;

pub use crv20::{Address, CURVEERC20};
pub use lp_token_wrapper::LPTOKENWRAPPER;
