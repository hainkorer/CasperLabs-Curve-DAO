use alloc::string::{String, ToString};
use casper_types::{Key, U256};

pub enum CurveTokenV3Event {
    Transfer {
        from: Key,
        to: Key,
        value: U256,
    },
    Approval {
        owner: Key,
        spender: Key,
        value: U256,
    },
}

impl CurveTokenV3Event {
    pub fn type_name(&self) -> String {
        match self {
            CurveTokenV3Event::Transfer {
                from: _,
                to: _,
                value: _,
            } => "Transfer",
            CurveTokenV3Event::Approval {
                owner: _,
                spender: _,
                value: _,
            } => "Approval",
        }
        .to_string()
    }
}
