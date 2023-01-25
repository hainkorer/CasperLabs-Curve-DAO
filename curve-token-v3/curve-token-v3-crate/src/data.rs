use core::convert::TryInto;

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::CLTyped;
use casper_types::{bytesrepr::ToBytes, ContractHash};
use casper_types::{ContractPackageHash, Key};
use casperlabs_contract_utils::{get_key, set_key};
use common::{keys::*, utils::*};

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
pub fn get_minter() -> Key {
    get_key(CURVE_TOKEN_V3_MINTER).unwrap_or_else(zero_address)
}

pub fn set_minter(minter: Key) {
    set_key(CURVE_TOKEN_V3_MINTER, minter);
}
pub fn get_token() -> Key {
    get_key(CURVE_TOKEN_V3_CURVE).unwrap_or_else(zero_address)
}

pub fn set_token(token: Key) {
    set_key(CURVE_TOKEN_V3_CURVE, token);
}

pub fn set_hash(contract_hash: ContractHash) {
    set_key(CURVE_TOKEN_V3_SELF_CONTRACT_HASH, contract_hash);
}
pub fn get_hash() -> ContractHash {
    get_key(CURVE_TOKEN_V3_SELF_CONTRACT_HASH).unwrap_or_default()
}
pub fn set_package_hash(package_hash: ContractPackageHash) {
    set_key(CURVE_TOKEN_V3_SELF_PACKAGE_HASH, package_hash);
}
pub fn get_package_hash() -> ContractPackageHash {
    get_key(CURVE_TOKEN_V3_SELF_PACKAGE_HASH).unwrap_or_default()
}
