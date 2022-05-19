use alloc::string::{String, ToString};
use casper_types::{Key, U256};

pub enum LiquidityGaugeRewardEvent {
    Deposit {
        provider: Key,
        value: U256,
    },
    Withdraw {
        provider: Key,
        value: U256,
    },
    UpdateLiquidityLimit {
        user: Key,
        original_balance: U256,
        original_supply: U256,
        working_balance: U256,
        working_supply: U256,
    },
    CommitOwnership {
        admin: Key,
    },
    ApplyOwnership {
        admin: Key,
    },
}

impl LiquidityGaugeRewardEvent {
    pub fn type_name(&self) -> String {
        match self {
            LiquidityGaugeRewardEvent::Deposit {
                provider: _,
                value: _,
            } => "deposit",
            LiquidityGaugeRewardEvent::Withdraw {
                provider: _,
                value: _,
            } => "withdraw",
            LiquidityGaugeRewardEvent::UpdateLiquidityLimit {
                user: _,
                original_balance: _,
                original_supply: _,
                working_balance: _,
                working_supply: _,
            } => "updateLiquidityLimit",
            LiquidityGaugeRewardEvent::CommitOwnership { admin: _ } => "commitOwnership",
            LiquidityGaugeRewardEvent::ApplyOwnership { admin: _ } => "applyOwnership",
        }
        .to_string()
    }
}
