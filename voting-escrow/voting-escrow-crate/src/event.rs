use alloc::string::{String, ToString};
use casper_types::{Key, U128, U256};

pub enum VotingEscrowEvent {
    CommitOwnership {
        admin: Key,
    },
    ApplyOwnership {
        admin: Key,
    },
    Deposit {
        provider: Key,
        value: U256,
        locktime: U256,
        _type: U128,
        ts: U256,
    },
    Withdraw {
        provider: Key,
        value: U256,
        ts: U256,
    },
    Supply {
        prev_supply: U256,
        supply: U256,
    },
}

impl VotingEscrowEvent {
    pub fn type_name(&self) -> String {
        match self {
            VotingEscrowEvent::CommitOwnership { admin: _ } => "commitOwnership",
            VotingEscrowEvent::ApplyOwnership { admin: _ } => "applyOwnership",
            VotingEscrowEvent::Deposit {
                provider: _,
                value: _,
                locktime: _,
                _type: _,
                ts: _,
            } => "deposit",
            VotingEscrowEvent::Withdraw {
                provider: _,
                value: _,
                ts: _,
            } => "withdraw",
            VotingEscrowEvent::Supply {
                prev_supply: _,
                supply: _,
            } => "supply",
        }
        .to_string()
    }
}
