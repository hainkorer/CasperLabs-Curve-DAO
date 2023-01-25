#![no_std]

extern crate alloc;

pub mod data;
mod i_reward_distribution_recipient;

pub use casperlabs_ownable::OWNABLE;
pub use i_reward_distribution_recipient::IREWARDDISTRIBUTIONRECIPIENT;
