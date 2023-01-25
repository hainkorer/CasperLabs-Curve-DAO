#![no_std]

extern crate alloc;

mod curve_rewards;
pub mod data;
pub mod event;

pub use casperlabs_i_reward_distribution_recipient::{IREWARDDISTRIBUTIONRECIPIENT, OWNABLE};
pub use casperlabs_lp_token_wrapper::{data::get_uni, Address, CURVEERC20, LPTOKENWRAPPER};
pub use curve_rewards::CURVEREWARDS;
