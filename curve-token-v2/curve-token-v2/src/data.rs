use core::{convert::TryInto, f32::MIN};

use alloc::string::String;
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{bytesrepr::ToBytes, CLTyped, ContractPackageHash, Key, U256};
use common::keys::*;
use contract_utils::{get_key, set_key};
pub fn set_result<T: ToBytes + CLTyped>(value: T) {
    match runtime::get_key(CURVE_TOKEN_V2_RESULT) {
        Some(key) => {
            let key_ref = key.try_into().unwrap_or_revert();
            storage::write(key_ref, value);
        }
        None => {
            let key = storage::new_uref(value).into();
            runtime::put_key(CURVE_TOKEN_V2_RESULT, key);
        }
    }
}
pub fn zero_address() -> Key {
    Key::from_formatted_str(
        "hash-0000000000000000000000000000000000000000000000000000000000000000".into(),
    )
    .unwrap()
}
pub fn get_init_supply() -> U256 {
    get_key(CURVE_TOKEN_V2_INIT_SUPPLY).unwrap_or_revert()
}

pub fn set_init_supply(init_supply: U256) {
    set_key(CURVE_TOKEN_V2_INIT_SUPPLY, init_supply);
}
pub fn get_minter() -> Key {
    get_key(CURVE_TOKEN_V2_MINTER).unwrap_or_revert()
}

pub fn set_minter(minter: Key) {
    set_key(CURVE_TOKEN_V2_MINTER, minter);
}

pub fn set_hash(contract_hash: Key) {
    set_key(CURVE_TOKEN_V2_SELF_CONTRACT_HASH, contract_hash);
}

pub fn get_hash() -> Key {
    get_key(CURVE_TOKEN_V2_SELF_CONTRACT_HASH).unwrap_or_revert()
}

pub fn set_package_hash(package_hash: ContractPackageHash) {
    set_key(CURVE_TOKEN_V2_SELF_PACKAGE_HASH, package_hash);
}

pub fn get_package_hash() -> ContractPackageHash {
    get_key(CURVE_TOKEN_V2_SELF_PACKAGE_HASH).unwrap_or_revert()
}
