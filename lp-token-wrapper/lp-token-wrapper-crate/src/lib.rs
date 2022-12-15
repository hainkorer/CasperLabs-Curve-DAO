#![no_std]

extern crate alloc;

pub mod data;
mod lp_token_wrapper;

pub use curve_erc20_crate::{Address, CURVEERC20};
pub use lp_token_wrapper::LPTOKENWRAPPER;
