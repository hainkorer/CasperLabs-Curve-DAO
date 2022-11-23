use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{bytesrepr::ToBytes, CLTyped, ContractPackageHash, Key, U128, U256};
use core::convert::TryInto;

use casperlabs_contract_utils::{get_key, set_key};
use common::{keys::*, utils::*};
pub const YEAR: U256 = U256([31536000000, 0, 0, 0]);
pub const INITIAL_SUPPLY: U256 = U256([1_303_030_303, 0, 0, 0]);
pub const INITIAL_RATE: U256 = U256([8714335457889396, 0, 0, 0]);
pub const RATE_REDUCTION_TIME: U256 = YEAR;
pub const RATE_REDUCTION_COEFFICIENT: U256 = U256([1189207115002721024, 0, 0, 0]);

pub const RATE_DENOMINATOR: U256 = U256([10000000000000000000, 0, 0, 0]); //10^18
pub const INFLATION_DELAY: U256 = U256([86400000, 0, 0, 0]);

pub fn set_result<T: ToBytes + CLTyped>(value: T) {
    match runtime::get_key(RESULT) {
        Some(key) => {
            let key_ref = key.try_into().unwrap_or_revert();
            storage::write(key_ref, value);
        }
        None => {
            let key = storage::new_uref(value).into();
            runtime::put_key(RESULT, key);
        }
    }
}

pub fn set_start_epoch_time(start_epoch_time: U256) {
    set_key(START_EPOCH_TIME, start_epoch_time);
}
pub fn get_start_epoch_time() -> U256 {
    get_key(START_EPOCH_TIME).unwrap_or_default()
}
pub fn set_rate(rate: U256) {
    set_key(RATE, rate);
}
pub fn get_rate() -> U256 {
    get_key(RATE).unwrap_or_default()
}
pub fn set_start_epoch_supply(start_epoch_supply: U256) {
    set_key(START_EPOCH_SUPPLY, start_epoch_supply);
}
pub fn get_start_epoch_supply() -> U256 {
    get_key(START_EPOCH_SUPPLY).unwrap_or_default()
}
pub fn get_init_supply() -> U256 {
    get_key(INIT_SUPPLY).unwrap_or_default()
}

pub fn set_init_supply(init_supply: U256) {
    set_key(INIT_SUPPLY, init_supply);
}
pub fn get_minter() -> Key {
    get_key(MINTER).unwrap_or_else(zero_address)
}
pub fn set_minter(minter: Key) {
    set_key(MINTER, minter);
}
pub fn get_admin() -> Key {
    get_key(ADMIN).unwrap_or_else(zero_address)
}
pub fn set_admin(admin: Key) {
    set_key(ADMIN, admin);
}
pub fn get_mining_epoch() -> U128 {
    get_key(MINING_EPOCH).unwrap_or_default()
}
pub fn set_mining_epoch(mining_epoch: U128) {
    set_key(MINING_EPOCH, mining_epoch);
}
pub fn get_is_updated() -> bool {
    get_key(IS_UPDATED).unwrap_or_default()
}
pub fn set_is_updated(is_updated: bool) {
    set_key(IS_UPDATED, is_updated);
}
pub fn set_hash(contract_hash: Key) {
    set_key(SELF_CONTRACT_HASH, contract_hash);
}
pub fn get_hash() -> Key {
    get_key(SELF_CONTRACT_HASH).unwrap_or_else(zero_address)
}
pub fn set_package_hash(package_hash: ContractPackageHash) {
    set_key(SELF_CONTRACT_PACKAGE_HASH, package_hash);
}
pub fn get_package_hash() -> ContractPackageHash {
    get_key(SELF_CONTRACT_PACKAGE_HASH).unwrap_or_default()
}
