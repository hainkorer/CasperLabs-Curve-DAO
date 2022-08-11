use alloc::vec::Vec;
use casper_types::Key;

use crate::TokenId;

pub enum ERC20Event {
    Approve {
        owner: Key,
        spender: Key,
        token_ids: Vec<TokenId>,
    },
    Transfer {
        sender: Key,
        recipient: Key,
        token_ids: Vec<TokenId>,
    },
}
