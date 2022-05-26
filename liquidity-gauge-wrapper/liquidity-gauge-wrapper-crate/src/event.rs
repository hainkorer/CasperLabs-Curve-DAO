use alloc::string::{String, ToString};
use casper_types::{Key, U256};

pub enum LiquidityGaugeWrapperEvent {
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

impl LiquidityGaugeWrapperEvent {
    pub fn type_name(&self) -> String {
        match self {
            LiquidityGaugeWrapperEvent::Deposit {
                provider: _,
                value: _,
            } => "deposit",
            LiquidityGaugeWrapperEvent::Withdraw {
                provider: _,
                value: _,
            } => "withdraw",
            LiquidityGaugeWrapperEvent::CommitOwnership { admin: _ } => "commitOwnership",
            LiquidityGaugeWrapperEvent::ApplyOwnership { admin: _ } => "applyOwnership",
            LiquidityGaugeWrapperEvent::Transfer {
                from: _,
                to: _,
                value: _,
            } => "transfer",
            LiquidityGaugeWrapperEvent::Approval {
                owner: _,
                spender: _,
                value: _,
            } => "approval",
        }
        .to_string()
    }
}
