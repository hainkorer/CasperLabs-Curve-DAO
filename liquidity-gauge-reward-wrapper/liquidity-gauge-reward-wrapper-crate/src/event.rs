use alloc::string::{String, ToString};
use casper_types::{Key, U256};

pub enum LiquidityGaugeRewardWrapperEvent {
    Deposit {
        provider: Key,
        value: U256,
    },
    Withdraw {
        provider: Key,
        value: U256,
    },
    CommitOwnership {
        admin: Key,
    },
    ApplyOwnership {
        admin: Key,
    },
    Approval {
        owner: Key,
        spender: Key,
        value: U256,
    },
    Transfer {
        from: Key,
        to: Key,
        value: U256,
    },
}

impl LiquidityGaugeRewardWrapperEvent {
    pub fn type_name(&self) -> String {
        match self {
            LiquidityGaugeRewardWrapperEvent::Deposit {
                provider: _,
                value: _,
            } => "deposit",
            LiquidityGaugeRewardWrapperEvent::Withdraw {
                provider: _,
                value: _,
            } => "withdraw",
            LiquidityGaugeRewardWrapperEvent::CommitOwnership { admin: _ } => "commitOwnership",
            LiquidityGaugeRewardWrapperEvent::ApplyOwnership { admin: _ } => "applyOwnership",
            LiquidityGaugeRewardWrapperEvent::Transfer {
                from: _,
                to: _,
                value: _,
            } => "transfer",
            LiquidityGaugeRewardWrapperEvent::Approval {
                owner: _,
                spender: _,
                value: _,
            } => "approval",
        }
        .to_string()
    }
}
