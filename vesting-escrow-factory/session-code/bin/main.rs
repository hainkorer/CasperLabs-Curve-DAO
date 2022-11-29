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
    bytesrepr::ToBytes, runtime_args, ApiError, CLTyped, ContractHash, Key, RuntimeArgs, URef, U256,
};
use common::keys::*;

const APPLY_TRANSFER_OWNERSHIP_VEF: &str = "apply_transfer_ownership_vef";
const COMMIT_TRANSFER_OWNERSHIP_VEF: &str = "commit_transfer_ownership_vef";
const DEPLOY_VESTING_CONTRACT: &str = "deploy_vesting_contract";
const INITIALIZE: &str = "initialize";

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
        APPLY_TRANSFER_OWNERSHIP_VEF => {
            let ret: bool = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                APPLY_TRANSFER_OWNERSHIP_VEF,
                runtime_args! {},
            );
            store(APPLY_TRANSFER_OWNERSHIP, ret);
        }
        COMMIT_TRANSFER_OWNERSHIP_VEF => {
            let addr: Key = runtime::get_named_arg("addr");
            let ret: bool = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                COMMIT_TRANSFER_OWNERSHIP_VEF,
                runtime_args! {
                    "addr"=>addr,
                },
            );
            store(COMMIT_TRANSFER_OWNERSHIP, ret);
        }
        DEPLOY_VESTING_CONTRACT => {
            let token: Key = runtime::get_named_arg("token");
            let recipient: Key = runtime::get_named_arg("recipient");
            let amount: U256 = runtime::get_named_arg("amount");
            let can_disable: bool = runtime::get_named_arg("can_disable");
            let vesting_duration: U256 = runtime::get_named_arg("vesting_duration");
            let vesting_start: Option<U256> = runtime::get_named_arg("vesting_start");
            let ret: Key = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                DEPLOY_VESTING_CONTRACT,
                runtime_args! {
                    "token"=>token,
                    "recipient"=>recipient,
                    "amount"=>amount,
                    "can_disable"=>can_disable,
                    "vesting_duration"=>vesting_duration,
                    "vesting_start"=>vesting_start
                },
            );
            store(DEPLOY_VESTING_CONTRACT, ret);
        }
        INITIALIZE => {
            let admin: Key = runtime::get_named_arg("admin");
            let token: Key = runtime::get_named_arg("token");
            let recipient: Key = runtime::get_named_arg("recipient");
            let amount: U256 = runtime::get_named_arg("amount");
            let start_time: U256 = runtime::get_named_arg("start_time");
            let end_time: U256 = runtime::get_named_arg("end_time");
            let can_disable: bool = runtime::get_named_arg("can_disable");
            let contract_hash: ContractHash = runtime::get_named_arg("contract_hash");
            let ret: bool = runtime::call_versioned_contract(
                package_hash.into_hash().unwrap_or_revert().into(),
                None,
                INITIALIZE,
                runtime_args! {
                    "admin"=>admin,
                    "token"=>token,
                    "recipient"=>recipient,
                    "amount"=>amount,
                    "start_time"=>start_time,
                    "end_time"=>end_time,
                    "can_disable"=>can_disable,
                    "contract_hash"=>contract_hash,
                    "package_hash"=>package_hash,
                },
            );
            store(INITIALIZE, ret);
        }
        _ => runtime::revert(ApiError::UnexpectedKeyVariant),
    };
}
