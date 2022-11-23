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

#[no_mangle]
pub extern "C" fn call() {
    let entrypoint: String = runtime::get_named_arg("entrypoint");
    let package_hash: Key = runtime::get_named_arg("package_hash");

    match entrypoint.as_str() {
        // Voting Escrow
        GET_LAST_USER_SLOPE => {
            let addr: Key = runtime::get_named_arg("addr");
            let ret: (bool, U128) = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                GET_LAST_USER_SLOPE,
                runtime_args! {
                    "addr" => addr
                },
            );
            store(GET_LAST_USER_SLOPE, ret);
        }
        USER_POINT_HISTORY_TS => {
            let addr: Key = runtime::get_named_arg("addr");
            let idx: U256 = runtime::get_named_arg("idx");
            let ret: U256 = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                USER_POINT_HISTORY_TS,
                runtime_args! {
                    "addr" => addr,
                    "idx" => idx
                },
            );
            store(USER_POINT_HISTORY_TS, ret);
        }
        LOCKED_END => {
            let addr: Key = runtime::get_named_arg("addr");
            let ret: U256 = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                LOCKED_END,
                runtime_args! {
                    "addr" => addr
                },
            );
            store(LOCKED_END, ret);
        }
        BALANCE_OF => {
            let addr: Key = runtime::get_named_arg("addr");
            let t: U256 = runtime::get_named_arg("t");
            let ret: U256 = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                BALANCE_OF,
                runtime_args! {
                    "addr" => addr,
                    "t" => Some(t)
                },
            );
            store(BALANCE_OF, ret);
        }
        BALANCE_OF_AT => {
            let addr: Key = runtime::get_named_arg("addr");
            let block: U256 = runtime::get_named_arg("block");
            let ret: U256 = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                BALANCE_OF_AT,
                runtime_args! {
                    "addr" => addr,
                    "block" => block
                },
            );
            store(BALANCE_OF_AT, ret);
        }
        TOTAL_SUPPLY => {
            let t: U256 = runtime::get_named_arg("t");
            let ret: U256 = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                TOTAL_SUPPLY,
                runtime_args! {
                    "t" => Some(t),
                },
            );
            store(TOTAL_SUPPLY, ret);
        }
        TOTAL_SUPPLY_AT => {
            let block: U256 = runtime::get_named_arg("block");
            let ret: U256 = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                TOTAL_SUPPLY_AT,
                runtime_args! {
                    "block" => block,
                },
            );
            store(TOTAL_SUPPLY_AT, ret);
        }
        // Fee Distributor
        VE_FOR_AT => {
            let user: Key = runtime::get_named_arg("user");
            let timestamp: U256 = runtime::get_named_arg("timestamp");
            let ret: U256 = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                VE_FOR_AT,
                runtime_args! {
                    "user" => user,
                    "timestamp" => timestamp
                },
            );
            store(VE_FOR_AT, ret);
        }
        CLAIM => {
            let addr: Option<Key> = runtime::get_named_arg("addr");
            let ret: U256 = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                CLAIM,
                runtime_args! {
                    "addr" => addr
                },
            );
            store(CLAIM, ret);
        }
        CLAIM_MANY => {
            let receivers: Vec<String> = runtime::get_named_arg("receivers");
            let ret: bool = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                CLAIM_MANY,
                runtime_args! {
                    "receivers" => receivers
                },
            );
            store(CLAIM_MANY, ret);
        }
        BURN => {
            let coin: Key = runtime::get_named_arg("coin");
            let ret: bool = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                BURN,
                runtime_args! {
                    "coin" => coin
                },
            );
            store(BURN, ret);
        }
        RECOVER_BALANCE => {
            let coin: Key = runtime::get_named_arg("coin");
            let ret: bool = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                RECOVER_BALANCE,
                runtime_args! {
                    "coin" => coin
                },
            );
            store(RECOVER_BALANCE, ret);
        }
        //IRewardDistributionRecipient
        IS_OWNER => {
            let ret: bool = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                IS_OWNER,
                runtime_args! {},
            );
            store(IS_OWNER, ret);
        }
        _ => runtime::revert(ApiError::UnexpectedKeyVariant),
    };
}
