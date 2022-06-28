use casper_types::{bytesrepr::ToBytes, CLTyped, ContractHash, ContractPackageHash, Key};
use common::keys::*;
use casperlabs_contract_utils::{get_key, set_key};

pub fn zero_address() -> Key {
    Key::from_formatted_str(
        "hash-0000000000000000000000000000000000000000000000000000000000000000".into(),
    )
    .unwrap()
}

pub fn get_ownership_admin() -> Key {
    get_key(OWNERSHIP_ADMIN).unwrap_or(zero_address())
}

pub fn set_ownership_admin(ownership_admin: Key) {
    set_key(OWNERSHIP_ADMIN, ownership_admin);
}

pub fn get_emergency_admin() -> Key {
    get_key(EMERGENCY_ADMIN).unwrap_or(zero_address())
}

pub fn set_emergency_admin(emergency_admin: Key) {
    set_key(EMERGENCY_ADMIN, emergency_admin);
}

pub fn get_future_ownership_admin() -> Key {
    get_key(FUTURE_OWNERSHIP_ADMIN).unwrap_or(zero_address())
}

pub fn set_future_ownership_admin(future_ownership_admin: Key) {
    set_key(FUTURE_OWNERSHIP_ADMIN, future_ownership_admin);
}

pub fn get_future_emergency_admin() -> Key {
    get_key(FUTURE_EMERGENCY_ADMIN).unwrap_or(zero_address())
}

pub fn set_future_emergency_admin(future_emergency_admin: Key) {
    set_key(FUTURE_EMERGENCY_ADMIN, future_emergency_admin);
}

pub fn get_lock() -> bool {
    get_key(LOCK).unwrap_or_default()
}

pub fn set_lock(lock: bool) {
    set_key(LOCK, lock);
}

pub fn get_contract_hash() -> ContractHash {
    get_key(SELF_CONTRACT_HASH).unwrap_or_default()
}

pub fn set_contract_hash(contract_hash: ContractHash) {
    set_key(SELF_CONTRACT_HASH, contract_hash);
}

pub fn get_package_hash() -> ContractPackageHash {
    get_key(SELF_CONTRACT_PACKAGE_HASH).unwrap_or_default()
}

pub fn set_package_hash(package_hash: ContractPackageHash) {
    set_key(SELF_CONTRACT_PACKAGE_HASH, package_hash);
}

pub fn js_ret<T: CLTyped + ToBytes>(ret: T) {
    set_key(RESULT, ret);
}
