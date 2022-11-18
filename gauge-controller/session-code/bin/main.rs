#![no_std]
#![no_main]

// We need to explicitly import the std alloc crate and `alloc::string::String` as we're in a
// `no_std` environment.
extern crate alloc;
use alloc::string::String;
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    bytesrepr::ToBytes, runtime_args, ApiError, CLTyped, Key, RuntimeArgs, URef, U128, U256,
};
use common::keys::*;

// Key is the same a destination
fn store<T: CLTyped + ToBytes>(key: &str, value: T) {
    // Store `value` under a new unforgeable reference.
    let value_ref: URef = storage::new_uref(value);

    // Wrap the unforgeable reference in a value of type `Key`.
    let value_key: Key = value_ref.into();

    // Store this key under the name "special_value" in context-local storage.
    runtime::put_key(key, value_key);
}

#[no_mangle]
pub extern "C" fn call() {
    let entrypoint: String = runtime::get_named_arg("entrypoint");
    let package_hash: Key = runtime::get_named_arg("package_hash");

    match entrypoint.as_str() {
        GAUGE_TYPES => {
            let addr: Key = runtime::get_named_arg("addr");
            let ret: (bool, U128) = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                GAUGE_TYPES,
                runtime_args! {
                    "addr"=>addr,
                },
            );
            store(GAUGE_TYPES, ret);
        }
        GAUGE_RELATIVE_WEIGHT => {
            let addr: Key = runtime::get_named_arg("addr");
            let ret: U256 = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                GAUGE_RELATIVE_WEIGHT,
                runtime_args! {
                    "addr"=>addr,
                },
            );
            store(GAUGE_RELATIVE_WEIGHT, ret);
        }
        GAUGE_RELATIVE_WEIGHT_WRITE => {
            let addr: Key = runtime::get_named_arg("addr");
            let ret: U256 = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                GAUGE_RELATIVE_WEIGHT_WRITE,
                runtime_args! {
                    "addr"=>addr,
                },
            );
            store(GAUGE_RELATIVE_WEIGHT_WRITE, ret);
        }

        
        GET_TOTAL_WEIGHT => {
            let ret: U256 = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                GET_TOTAL_WEIGHT,
                runtime_args! {},
            );
            store(GET_TOTAL_WEIGHT, ret);
        }
        GET_WEIGHTS_SUM_PER_TYPE => {
            let type_id: (bool, U128) = runtime::get_named_arg("type_id");
            let ret: U256 = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                GET_WEIGHTS_SUM_PER_TYPE,
                runtime_args! {
                    "type_id"=>type_id,
                },
            );
            store(GET_WEIGHTS_SUM_PER_TYPE, ret);
        }
        _ => runtime::revert(ApiError::UnexpectedKeyVariant),
    };
}
