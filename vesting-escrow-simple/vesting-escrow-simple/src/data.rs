use alloc::string::String;
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{bytesrepr::ToBytes, CLTyped, ContractPackageHash, Key, U256};
use common::keys::*;
use contract_utils::{get_key, set_key, Dict};
use core::{convert::TryInto, f32::MIN};

pub fn set_result<T: ToBytes + CLTyped>(value: T) {
    match runtime::get_key(VESTING_ESCROW_SIMPLE_RESULT) {
        Some(key) => {
            let key_ref = key.try_into().unwrap_or_revert();
            storage::write(key_ref, value);
        }
        None => {
            let key = storage::new_uref(value).into();
            runtime::put_key(VESTING_ESCROW_SIMPLE_RESULT, key);
        }
    }
}

pub fn set_token(token: Key) {
    set_key(VESTING_ESCROW_SIMPLE_TOKEN, token);
}

pub fn get_token() -> Key {
    get_key(VESTING_ESCROW_SIMPLE_TOKEN).unwrap_or_revert()
}
pub fn set_start_time(start_time: U256) {
    set_key(VESTING_ESCROW_SIMPLE_START_TIME, start_time);
}

pub fn get_start_time() -> U256 {
    get_key(VESTING_ESCROW_SIMPLE_START_TIME).unwrap_or_revert()
}
pub fn set_end_time(end_time: U256) {
    set_key(VESTING_ESCROW_SIMPLE_END_TIME, end_time);
}

pub fn get_end_time() -> U256 {
    get_key(VESTING_ESCROW_SIMPLE_END_TIME).unwrap_or_revert()
}
pub struct InitialLocked {
    dict: Dict,
}

impl InitialLocked {
    pub fn instance() -> InitialLocked {
        InitialLocked {
            dict: Dict::instance(VESTING_ESCROW_SIMPLE_INITIAL_LOCKED_DICT),
        }
    }

    pub fn init() {
        Dict::init(VESTING_ESCROW_SIMPLE_INITIAL_LOCKED_DICT)
    }

    pub fn get(&self, address: &Key) -> U256 {
        self.dict.get_by_key(address).unwrap_or_default()
    }

    pub fn set(&self, address: &Key, value: U256) {
        self.dict.set_by_key(address, value)
    }
}
pub struct TotalClaimed {
    dict: Dict,
}

impl TotalClaimed {
    pub fn instance() -> TotalClaimed {
        TotalClaimed {
            dict: Dict::instance(VESTING_ESCROW_SIMPLE_TOTAL_CLAIMED_DICT),
        }
    }

    pub fn init() {
        Dict::init(VESTING_ESCROW_SIMPLE_TOTAL_CLAIMED_DICT)
    }

    pub fn get(&self, address: &Key) -> U256 {
        self.dict.get_by_key(address).unwrap_or_default()
    }

    pub fn set(&self, address: &Key, value: U256) {
        self.dict.set_by_key(address, value)
    }
}

pub struct DisableddAt {
    dict: Dict,
}

impl DisableddAt {
    pub fn instance() -> DisableddAt {
        DisableddAt {
            dict: Dict::instance(VESTING_ESCROW_SIMPLE_DISABLED_AT_DICT),
        }
    }

    pub fn init() {
        Dict::init(VESTING_ESCROW_SIMPLE_DISABLED_AT_DICT)
    }

    pub fn get(&self, address: &Key) -> U256 {
        self.dict.get_by_key(address).unwrap_or_default()
    }

    pub fn set(&self, address: &Key, value: U256) {
        self.dict.set_by_key(address, value)
    }
}

pub fn get_lock() -> bool {
    get_key(VESTING_ESCROW_SIMPLE_LOCK).unwrap_or_default()
}

pub fn set_lock(lock: bool) {
    set_key(VESTING_ESCROW_SIMPLE_LOCK, lock);
}
pub fn set_initial_locked_supply(initial_locked_supply: U256) {
    set_key(
        VESTING_ESCROW_SIMPLE_INITIAL_LOCKED_DICT_SUPPLY,
        initial_locked_supply,
    );
}

pub fn get_initial_locked_supply() -> U256 {
    get_key(VESTING_ESCROW_SIMPLE_INITIAL_LOCKED_DICT_SUPPLY).unwrap_or_revert()
}
pub fn set_can_disable(can_disable: bool) {
    set_key(VESTING_ESCROW_SIMPLE_CAN_DISABLE, can_disable);
}

pub fn get_can_disable() -> bool {
    get_key(VESTING_ESCROW_SIMPLE_CAN_DISABLE).unwrap_or_revert()
}
pub fn set_admin(admin: Key) {
    set_key(VESTING_ESCROW_SIMPLE_ADMIN, admin);
}

pub fn get_admin() -> Key {
    get_key(VESTING_ESCROW_SIMPLE_ADMIN).unwrap_or_revert()
}
pub fn set_future_admin(future_admin: Key) {
    set_key(VESTING_ESCROW_SIMPLE_FUTURE_ADMIN, future_admin);
}
pub fn get_future_admin() -> Key {
    get_key(VESTING_ESCROW_SIMPLE_FUTURE_ADMIN).unwrap_or_revert()
}
pub fn zero_address() -> Key {
    Key::from_formatted_str(
        "hash-0000000000000000000000000000000000000000000000000000000000000000".into(),
    )
    .unwrap()
}

pub fn set_hash(contract_hash: Key) {
    set_key(VESTING_ESCROW_SIMPLE_SELF_CONTRACT_HASH, contract_hash);
}

pub fn get_hash() -> Key {
    get_key(VESTING_ESCROW_SIMPLE_SELF_CONTRACT_HASH).unwrap_or_revert()
}

pub fn set_package_hash(package_hash: ContractPackageHash) {
    set_key(VESTING_ESCROW_SIMPLE_SELF_PACKAGE_HASH, package_hash);
}

pub fn get_package_hash() -> ContractPackageHash {
    get_key(VESTING_ESCROW_SIMPLE_SELF_PACKAGE_HASH).unwrap_or_revert()
}
