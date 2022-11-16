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
    bytesrepr::ToBytes, runtime_args, ApiError, CLTyped, Key, RuntimeArgs, URef, U256,
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
        FUTURE_EPOCH_TIME_WRITE => {
            let ret: U256 = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                FUTURE_EPOCH_TIME_WRITE,
                runtime_args! {},
            );
            store(FUTURE_EPOCH_TIME_WRITE, ret);
        }
        START_EPOCH_TIME_WRITE => {
            let ret: U256 = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                START_EPOCH_TIME_WRITE,
                runtime_args! {},
            );
            store(START_EPOCH_TIME_WRITE, ret);
        }
        AVAILABLE_SUPPLY => {
            let ret: U256 = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                AVAILABLE_SUPPLY,
                runtime_args! {},
            );
            store(AVAILABLE_SUPPLY, ret);
        }
        MINT => {
            let to: Key = runtime::get_named_arg("to");
            let amount: U256 = runtime::get_named_arg("amount");
            let ret: bool = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                MINT,
                runtime_args! {
                    "to"=>to,
                    "amount"=>amount
                },
            );
            store(MINT, ret);
        }
        MINTABLE_IN_TIMEFRAME => {
            let start: U256 = runtime::get_named_arg("start");
            let end: U256 = runtime::get_named_arg("end");
            let ret: U256 = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                MINTABLE_IN_TIMEFRAME,
                runtime_args! {
                    "start"=>start,
                    "end"=>end
                },
            );
            store(MINTABLE_IN_TIMEFRAME, ret);
        }
        BALANCE_OF => {
            let owner: Key = runtime::get_named_arg("owner");
            let ret: U256 = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                BALANCE_OF,
                runtime_args! {
                    "owner"=>owner
                },
            );
            store(BALANCE_OF, ret);
        }
        TOTAL_SUPPLY => {
            let ret: U256 = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                TOTAL_SUPPLY,
                runtime_args! {},
            );
            store(TOTAL_SUPPLY, ret);
        }
        INCREASE_ALLOWANCE => {
            let spender: Key = runtime::get_named_arg("spender");
            let amount: U256 = runtime::get_named_arg("amount");
            let ret: Result<(), u32> = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                INCREASE_ALLOWANCE,
                runtime_args! {
                    "spender"=>spender,
                    "amount"=>amount
                },
            );
            store(INCREASE_ALLOWANCE, ret);
        }
        DECREASE_ALLOWANCE => {
            let spender: Key = runtime::get_named_arg("spender");
            let amount: U256 = runtime::get_named_arg("amount");
            let ret: Result<(), u32> = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                DECREASE_ALLOWANCE,
                runtime_args! {
                    "spender"=>spender,
                    "amount"=>amount
                },
            );
            store(DECREASE_ALLOWANCE, ret);
        }
        TRANSFER => {
            let recipient: Key = runtime::get_named_arg("recipient");
            let amount: U256 = runtime::get_named_arg("amount");
            let ret: Result<(), u32> = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                TRANSFER,
                runtime_args! {
                    "recipient"=>recipient,
                    "amount"=>amount
                },
            );
            store(TRANSFER, ret);
        }
        TRANSFER_FROM => {
            let owner: Key = runtime::get_named_arg("owner");
            let recipient: Key = runtime::get_named_arg("recipient");
            let amount: U256 = runtime::get_named_arg("amount");
            let ret: Result<(), u32> = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                TRANSFER_FROM,
                runtime_args! {
                    "owner"=>owner,
                    "recipient"=>recipient,
                    "amount"=>amount
                },
            );
            store(TRANSFER_FROM, ret);
        }
        ALLOWANCE => {
            let owner: Key = runtime::get_named_arg("owner");
            let spender: Key = runtime::get_named_arg("spender");
            let ret: U256 = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                ALLOWANCE,
                runtime_args! {
                    "owner"=>owner,
                    "spender"=>spender
                },
            );
            store(ALLOWANCE, ret);
        }
        _ => runtime::revert(ApiError::UnexpectedKeyVariant),
    };
}
