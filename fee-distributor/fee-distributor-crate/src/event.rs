use alloc::string::{String, ToString};
use casper_types::{Key, U256};

pub enum FeeDistributorEvent {
    CommitAdmin {
        admin: Key,
    },
    ApplyAdmin {
        admin: Key,
    },
    ToggleAllowCheckpointToken {
        toggle_flag: bool,
    },
    CheckpointToken {
        time: U256,
        tokens: U256,
    },
    Claimed {
        recipient: Key,
        amount: U256,
        claim_epoch: U256,
        max_epoch: U256,
    },
}

impl FeeDistributorEvent {
    pub fn type_name(&self) -> String {
        match self {
            FeeDistributorEvent::CommitAdmin { admin: _ } => "commitAdmin",
            FeeDistributorEvent::ApplyAdmin { admin: _ } => "applyAdmin",
            FeeDistributorEvent::ToggleAllowCheckpointToken { toggle_flag: _ } => {
                "toggleAllowCheckpointToken"
            }
            FeeDistributorEvent::CheckpointToken { time: _, tokens: _ } => "checkpointToken",
            FeeDistributorEvent::Claimed {
                recipient: _,
                amount: _,
                claim_epoch: _,
                max_epoch: _,
            } => "claimed",
        }
        .to_string()
    }
}
