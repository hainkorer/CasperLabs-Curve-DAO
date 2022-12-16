use casper_types::Key;
use casperlabs_contract_utils::{get_key, set_key};
use common::{keys::*, utils::*};

pub fn set_uni(uni: Key) {
    set_key(UNI, uni);
}
pub fn get_uni() -> Key {
    get_key(UNI).unwrap_or_else(zero_address)
}
