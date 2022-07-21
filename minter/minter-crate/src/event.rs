use casper_types::{Key, U256};

pub enum MINTEREvent {
    Minted {
        recipient: Key,
        gauge: Key,
        minted: U256,
    },
}
