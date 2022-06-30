use core::convert::TryInto;

use crate::alloc::string::ToString;
use alloc::string::String;

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};

use casper_types::{
    bytesrepr::{FromBytes, ToBytes},
    CLTyped,
};

pub fn get_key<T: FromBytes + CLTyped + Default>(name: &str) -> T {
    match runtime::get_key(name) {
        None => Default::default(),
        Some(value) => {
            let key = value.try_into().unwrap_or_revert();
            storage::read(key).unwrap_or_revert().unwrap_or_revert()
        }
    }
}

pub fn set_key<T: ToBytes + CLTyped>(name: &str, value: T) {
    match runtime::get_key(name) {
        Some(key) => {
            let key_ref = key.try_into().unwrap_or_revert();
            storage::write(key_ref, value);
        }
        None => {
            let key = storage::new_uref(value).into();
            runtime::put_key(name, key);
        }
    }
}

pub fn self_hash_key() -> String {
    "self_hash".to_string()
}

pub fn self_package_key() -> String {
    "package_hash".to_string()
}

pub fn gauge_controller_key() -> String {
    "gauge_controller".to_string()
}

pub fn transfer_key() -> String {
    "transfer_result".to_string()
}

pub fn transfer_from_key() -> String {
    "transfer_from_result".to_string()
}

pub fn allowance() -> String {
    "allowance".to_string()
}

pub fn increase_allowance_key() -> String {
    "increase_allowance_result".to_string()
}

pub fn decrease_allowance_key() -> String {
    "decrease_allowance_result".to_string()
}
