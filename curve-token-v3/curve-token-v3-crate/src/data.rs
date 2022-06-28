use core::{convert::TryInto, f32::MIN};

use alloc::string::String;
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::bytesrepr::ToBytes;
use casper_types::CLTyped;
use casper_types::{ContractPackageHash, Key, U256};
use casper_types_derive::{CLTyped, FromBytes, ToBytes};
use common::keys::*;
use casperlabs_contract_utils::{get_key, key_to_str, set_key, Dict};

pub fn set_result<T: ToBytes + CLTyped>(value: T) {
    match runtime::get_key(CURVE_TOKEN_V3_RESULT) {
        Some(key) => {
            let key_ref = key.try_into().unwrap_or_revert();
            storage::write(key_ref, value);
        }
        None => {
            let key = storage::new_uref(value).into();
            runtime::put_key(CURVE_TOKEN_V3_RESULT, key);
        }
    }
}
pub struct Balances {
    dict: Dict,
}

impl Balances {
    pub fn instance() -> Balances {
        Balances {
            dict: Dict::instance(CURVE_TOKEN_V3_BALANCE_OF_DICT),
        }
    }

    pub fn init() {
        Dict::init(CURVE_TOKEN_V3_BALANCE_OF_DICT)
    }

    pub fn get(&self, owner: &Key) -> U256 {
        self.dict.get(&key_to_str(owner)).unwrap_or_default()
    }

    pub fn set(&self, owner: &Key, value: U256) {
        self.dict.set(&key_to_str(owner), value);
    }
}
pub struct Allowances {
    dict: Dict,
}

impl Allowances {
    pub fn instance() -> Allowances {
        Allowances {
            dict: Dict::instance(CURVE_TOKEN_V3_ALLOWANCES_DICT),
        }
    }

    pub fn init() {
        Dict::init(CURVE_TOKEN_V3_ALLOWANCES_DICT)
    }

    pub fn get(&self, owner: &Key, spender: &Key) -> U256 {
        self.dict.get_by_keys((owner, spender)).unwrap_or_default()
    }

    pub fn set(&self, owner: &Key, spender: &Key, value: U256) {
        self.dict.set_by_keys((owner, spender), value);
    }
}

pub fn zero_address() -> Key {
    Key::from_formatted_str(
        "hash-0000000000000000000000000000000000000000000000000000000000000000".into(),
    )
    .unwrap()
}
pub fn get_name() -> String {
    get_key(CURVE_TOKEN_V3_NAME).unwrap_or_revert()
}

pub fn set_name(name: String) {
    set_key(CURVE_TOKEN_V3_NAME, name);
}
pub fn get_symbol() -> String {
    get_key(CURVE_TOKEN_V3_SYMBOL).unwrap_or_revert()
}

pub fn set_symbol(symbol: String) {
    set_key(CURVE_TOKEN_V3_SYMBOL, symbol);
}
pub fn get_decimals() -> u8 {
    get_key(CURVE_TOKEN_V3_DECIMALS).unwrap_or_revert()
}

pub fn set_decimals(decimals: u8) {
    set_key(CURVE_TOKEN_V3_DECIMALS, decimals);
}
pub fn get_total_supply() -> U256 {
    get_key(CURVE_TOKEN_V3_TOTAL_SUPPLY).unwrap_or_revert()
}

pub fn set_total_supply(init_supply: U256) {
    set_key(CURVE_TOKEN_V3_TOTAL_SUPPLY, init_supply);
}
pub fn get_minter() -> Key {
    get_key(CURVE_TOKEN_V3_MINTER).unwrap_or_revert()
}

pub fn set_minter(minter: Key) {
    set_key(CURVE_TOKEN_V3_MINTER, minter);
}
pub fn get_token() -> Key {
    get_key(CURVE_TOKEN_V3_CURVE).unwrap_or(zero_address())
}

pub fn set_token(token: Key) {
    set_key(CURVE_TOKEN_V3_CURVE, token);
}

pub fn set_hash(contract_hash: Key) {
    set_key(CURVE_TOKEN_V3_SELF_CONTRACT_HASH, contract_hash);
}

pub fn get_hash() -> Key {
    get_key(CURVE_TOKEN_V3_SELF_CONTRACT_HASH).unwrap_or_revert()
}

pub fn set_package_hash(package_hash: ContractPackageHash) {
    set_key(CURVE_TOKEN_V3_SELF_PACKAGE_HASH, package_hash);
}

pub fn get_package_hash() -> ContractPackageHash {
    get_key(CURVE_TOKEN_V3_SELF_PACKAGE_HASH).unwrap_or_revert()
}
