#![no_std]
#![no_main]

// We need to explicitly import the std alloc crate and `alloc::string::String` as we're in a
// `no_std` environment.
extern crate alloc;
use alloc::{string::String, vec::Vec};
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

// pub const APPLY_TRANSFER_OWNERSHIP: &str = "apply_transfer_ownership";
// pub const COMMIT_TRANSFER_OWNERSHIP: &str = "commit_transfer_ownership";
// pub const VESTED_SUPPLY: &str = "vested_supply";
// pub const LOCKED_SUPPLY: &str = "locked_supply";
// pub const VESTED_OF: &str = "vested_of";
// pub const BALANCE_OF: &str = "balance_of";
// pub const LOCKED_OF: &str = "locked_of";

#[no_mangle]
pub extern "C" fn call() {
    let entrypoint: String = runtime::get_named_arg("entrypoint");
    let package_hash: Key = runtime::get_named_arg("package_hash");

    match entrypoint.as_str() {
        APPLY_TRANSFER_OWNERSHIP => {
            let ret: bool = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                APPLY_TRANSFER_OWNERSHIP,
                runtime_args! {
                },
            );
            store(APPLY_TRANSFER_OWNERSHIP, ret);
        }
        COMMIT_TRANSFER_OWNERSHIP => {
            let addr: Key = runtime::get_named_arg("addr");
            let ret: bool = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                COMMIT_TRANSFER_OWNERSHIP,
                runtime_args! {
                    "addr"=>addr,
                },
            );
            store(COMMIT_TRANSFER_OWNERSHIP, ret);
        }
        VESTED_SUPPLY => {
            let ret: U256 = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                VESTED_SUPPLY,
                runtime_args! {
                },
            );
            store(VESTED_SUPPLY, ret);
        }
        LOCKED_SUPPLY => {
            let ret: U256 = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                LOCKED_SUPPLY,
                runtime_args! {
                },
            );
            store(LOCKED_SUPPLY, ret);
        }
        VESTED_OF => {
            let _recipient: Key = runtime::get_named_arg("_recipient");
            let ret: U256 = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                VESTED_OF,
                runtime_args! {
                    "_recipient"=>_recipient,
                },
            );
            store(VESTED_OF, ret);
        }
        BALANCE_OF => {
            let _recipient: Key = runtime::get_named_arg("_recipient");
            let ret: U256 = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                BALANCE_OF,
                runtime_args! {
                    "_recipient"=>_recipient,
                },
            );
            store(BALANCE_OF, ret);
        }
        LOCKED_OF => {
            let _recipient: Key = runtime::get_named_arg("_recipient");
            let ret: U256 = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                LOCKED_OF,
                runtime_args! {
                    "_recipient"=>_recipient,
                },
            );
            store(LOCKED_OF, ret);
        }
        _ => runtime::revert(ApiError::UnexpectedKeyVariant),
    };
}
