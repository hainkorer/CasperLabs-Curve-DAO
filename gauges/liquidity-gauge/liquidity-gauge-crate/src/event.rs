use alloc::string::{String, ToString};
use casper_types::{Key, U256};

pub enum LiquidityGaugeEvent {
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

impl LiquidityGaugeEvent {
    pub fn type_name(&self) -> String {
        match self {
            LiquidityGaugeEvent::Deposit {
                provider: _,
                value: _,
            } => "Deposit",
            LiquidityGaugeEvent::Withdraw {
                provider: _,
                value: _,
            } => "Withdraw",
            LiquidityGaugeEvent::UpdateLiquidityLimit {
                user: _,
                original_balance: _,
                original_supply: _,
                working_balance: _,
                working_supply: _,
            } => "UpdateLiquidityLimit",
            LiquidityGaugeEvent::CommitOwnership { admin: _ } => "CommitOwnership",
            LiquidityGaugeEvent::ApplyOwnership { admin: _ } => "ApplyOwnership",
        }
        .to_string()
    }
}
