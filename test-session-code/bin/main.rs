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
        CLAIMABLE_REWARD => {
            let addr: Key = runtime::get_named_arg("addr");
            let ret: U256 = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                CLAIMABLE_REWARD,
                runtime_args! {
                    "addr" => addr,
                },
            );
            store(CLAIMABLE_REWARD, ret);
        }
        ALLOWANCE => {
            let owner: Key = runtime::get_named_arg("owner");
            let spender: Key = runtime::get_named_arg("spender");
            let ret: U256 = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                ALLOWANCE,
                runtime_args! {
                    "owner" => owner,
                    "spender" => spender,
                },
            );
            store(ALLOWANCE, ret);
        }
        BALANCE_OF => {
            let owner: Key = runtime::get_named_arg("owner");
            let ret: U256 = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                BALANCE_OF,
                runtime_args! {
                    "owner" => owner,
                },
            );
            store(BALANCE_OF, ret);
        }
        APPROVED_TO_DEPOSIT => {
            let owner: Key = runtime::get_named_arg("owner");
            let spender: Key = runtime::get_named_arg("spender");
            let ret: bool = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                APPROVED_TO_DEPOSIT,
                runtime_args! {
                    "owner" => owner,
                    "spender" => spender,
                },
            );
            store(APPROVED_TO_DEPOSIT, ret);
        }
        ADMIN => {
            let ret: Key = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                ADMIN,
                runtime_args! {},
            );
            store(ADMIN, ret);
        }
        FUTURE_ADMIN => {
            let ret: Key = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                FUTURE_ADMIN,
                runtime_args! {},
            );
            store(FUTURE_ADMIN, ret);
        }
        IS_KILLED => {
            let ret: bool = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                IS_KILLED,
                runtime_args! {},
            );
            store(IS_KILLED, ret);
        }
        IS_OWNER => {
            let ret: bool = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                IS_OWNER,
                runtime_args! {},
            );
            store(IS_OWNER, ret);
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
        LAST_TIME_REWARD_APPLICABLE => {
            let ret: U256 = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                LAST_TIME_REWARD_APPLICABLE,
                runtime_args! {},
            );
            store(LAST_TIME_REWARD_APPLICABLE, ret);
        }
        REWARD_PER_TOKEN => {
            let ret: U256 = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                REWARD_PER_TOKEN,
                runtime_args! {},
            );
            store(REWARD_PER_TOKEN, ret);
        }
        EARNED => {
            let account: Key = runtime::get_named_arg("account");
            let ret: U256 = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                EARNED,
                runtime_args! {
                    "account" => account,
                },
            );
            store(EARNED, ret);
        }
        DECIMALS => {
            let ret: U256 = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                DECIMALS,
                runtime_args! {},
            );
            store(DECIMALS, ret);
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
        INTEGRATE_CHECKPOINT => {
            let ret: U256 = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                INTEGRATE_CHECKPOINT,
                runtime_args! {},
            );
            store(INTEGRATE_CHECKPOINT, ret);
        }
        REWARD_CONTRACT => {
            let ret: Key = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                REWARD_CONTRACT,
                runtime_args! {},
            );
            store(REWARD_CONTRACT, ret);
        }
        LAST_CLAIM => {
            let ret: U256 = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                LAST_CLAIM,
                runtime_args! {},
            );
            store(LAST_CLAIM, ret);
        }
        CLAIMED_REWARD => {
            let addr: Key = runtime::get_named_arg("addr");
            let token: Key = runtime::get_named_arg("token");
            let ret: U256 = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                CLAIMED_REWARD,
                runtime_args! {
                    "addr"=>addr,
                    "token"=>token,
                },
            );
            store(CLAIMED_REWARD, ret);
        }
        CLAIMABLE_V3_REWARD => {
            let addr: Key = runtime::get_named_arg("addr");
            let token: Key = runtime::get_named_arg("token");
            let ret: U256 = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                CLAIMABLE_REWARD,
                runtime_args! {
                    "addr"=>addr,
                    "token"=>token,
                },
            );
            store(CLAIMABLE_REWARD, ret);
        }
        U8_DECIMALS => {
            let ret: u8 = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                DECIMALS,
                runtime_args! {},
            );
            store(DECIMALS, ret);
        }
        VESTED_SUPPLY => {
            let ret: U256 = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                VESTED_SUPPLY,
                runtime_args! {},
            );
            store(VESTED_SUPPLY, ret);
        }
        LOCKED_SUPPLY => {
            let ret: U256 = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                LOCKED_SUPPLY,
                runtime_args! {},
            );
            store(LOCKED_SUPPLY, ret);
        }
        VESTED_OF => {
            let recipient: Key = runtime::get_named_arg("recipient");
            let ret: U256 = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                VESTED_OF,
                runtime_args! {
                    "recipient"=>recipient,
                },
            );
            store(VESTED_OF, ret);
        }
        LOCKED_OF => {
            let recipient: Key = runtime::get_named_arg("recipient");
            let ret: U256 = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                LOCKED_OF,
                runtime_args! {
                    "recipient"=>recipient,
                },
            );
            store(LOCKED_OF, ret);
        }
         _ => runtime::revert(ApiError::UnexpectedKeyVariant),
    };
}
