use alloc::string::{String, ToString};
use casper_types::{Key, U256};

pub enum CurveRewardsEvent {
    RewardAdded { reward: U256 },
    Staked { user: Key, amount: U256 },
    Withdrawn { user: Key, amount: U256 },
    RewardPaid { user: Key, reward: U256 },
}

impl CurveRewardsEvent {
    pub fn type_name(&self) -> String {
        match self {
            CurveRewardsEvent::RewardAdded { reward: _ } => "RewardAdded",
            CurveRewardsEvent::Staked { user: _, amount: _ } => "Staked",
            CurveRewardsEvent::Withdrawn { user: _, amount: _ } => "Withdrawn",
            CurveRewardsEvent::RewardPaid { user: _, reward: _ } => "RewardPaid",
        }
        .to_string()
    }
}
