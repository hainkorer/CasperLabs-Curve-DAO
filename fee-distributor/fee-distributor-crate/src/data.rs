use alloc::{string::ToString, vec::Vec};
use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_types::{
    bytesrepr::ToBytes, CLTyped, ContractHash, ContractPackageHash, Key, U128, U256,
};
use casper_types_derive::{CLTyped, FromBytes, ToBytes};
use common::keys::*;
use contract_utils::{get_key, set_key, Dict};

pub const WEEK: U256 = U256([604800, 0, 0, 0]);
pub const TOKEN_CHECKPOINT_DEADLINE: U256 = U256([86400, 0, 0, 0]);

pub fn zero_address() -> Key {
    Key::from_formatted_str(
        "hash-0000000000000000000000000000000000000000000000000000000000000000".into(),
    )
    .unwrap()
}

// We cannot really do block numbers per se b/c slope is per time, not per block
// and per block could be fairly bad b/c Ethereum changes blocktimes.
// What we can do is to extrapolate ***At functions
#[derive(Clone, Copy, CLTyped, ToBytes, FromBytes, Default)]
pub struct Point {
    pub bias: U128,
    pub slope: U128, // - dweight / dt
    pub ts: U256,
    pub blk: U256, // block
}

pub const FEE_DISTRIBUTOR_TIME_CURSOR_OF: &str = "fee_distributor_time_cursor_of";
pub struct TimeCursorOf {
    dict: Dict,
}

impl TimeCursorOf {
    pub fn instance() -> TimeCursorOf {
        TimeCursorOf {
            dict: Dict::instance(FEE_DISTRIBUTOR_TIME_CURSOR_OF),
        }
    }

    pub fn init() {
        Dict::init(FEE_DISTRIBUTOR_TIME_CURSOR_OF)
    }

    pub fn get(&self, key: &Key) -> U256 {
        self.dict.get_by_key(key).unwrap_or_default()
    }

    pub fn set(&self, key: &Key, value: U256) {
        self.dict.set_by_key(key, value);
    }
}

pub const FEE_DISTRIBUTOR_USER_EPOCH_OF: &str = "fee_distributor_user_epoch_of";
pub struct UserEpochOf {
    dict: Dict,
}

impl UserEpochOf {
    pub fn instance() -> UserEpochOf {
        UserEpochOf {
            dict: Dict::instance(FEE_DISTRIBUTOR_USER_EPOCH_OF),
        }
    }

    pub fn init() {
        Dict::init(FEE_DISTRIBUTOR_USER_EPOCH_OF)
    }

    pub fn get(&self, key: &Key) -> U256 {
        self.dict.get_by_key(key).unwrap_or_default()
    }

    pub fn set(&self, key: &Key, value: U256) {
        self.dict.set_by_key(key, value);
    }
}

pub const FEE_DISTRIBUTOR_TOKENS_PER_WEEK: &str = "fee_distributor_tokens_per_week";
pub struct TokensPerWeek {
    dict: Dict,
    length: U256,
}

impl TokensPerWeek {
    pub fn instance() -> TokensPerWeek {
        TokensPerWeek {
            dict: Dict::instance(FEE_DISTRIBUTOR_TOKENS_PER_WEEK),
            length: 0.into(),
        }
    }

    pub fn init() {
        Dict::init(FEE_DISTRIBUTOR_TOKENS_PER_WEEK)
    }

    pub fn get(&self, indx: &U256) -> U256 {
        self.dict.get(indx.to_string().as_str()).unwrap_or_default()
    }

    pub fn set(&self, indx: &U256, value: U256) {
        self.dict.set(indx.to_string().as_str(), value);
    }

    pub fn push(&mut self, value: U256) {
        self.dict.set(self.length.to_string().as_str(), value);
        self.length = self.length.checked_add(1.into()).unwrap_or_revert();
    }
}

pub const FEE_DISTRIBUTOR_VE_SUPPLY: &str = "fee_distributor_ve_supply";
pub struct VeSupply {
    dict: Dict,
    length: U256,
}

impl VeSupply {
    pub fn instance() -> VeSupply {
        VeSupply {
            dict: Dict::instance(FEE_DISTRIBUTOR_VE_SUPPLY),
            length: 0.into(),
        }
    }

    pub fn init() {
        Dict::init(FEE_DISTRIBUTOR_VE_SUPPLY)
    }

    pub fn get(&self, indx: &U256) -> U256 {
        self.dict.get(indx.to_string().as_str()).unwrap_or_default()
    }

    pub fn set(&self, indx: &U256, value: U256) {
        self.dict.set(indx.to_string().as_str(), value);
    }

    pub fn push(&mut self, value: U256) {
        self.dict.set(self.length.to_string().as_str(), value);
        self.length = self.length.checked_add(1.into()).unwrap_or_revert();
    }
}

pub fn get_start_time() -> U256 {
    get_key(FEE_DISTRIBUTOR_START_TIME).unwrap_or_default()
}

pub fn set_start_time(start_time: U256) {
    set_key(FEE_DISTRIBUTOR_START_TIME, start_time);
}

pub fn get_time_cursor() -> U256 {
    get_key(FEE_DISTRIBUTOR_TIME_CURSOR).unwrap_or_default()
}

pub fn set_time_cursor(time_cursor: U256) {
    set_key(FEE_DISTRIBUTOR_TIME_CURSOR, time_cursor);
}

pub fn get_last_token_time() -> U256 {
    get_key(FEE_DISTRIBUTOR_LAST_TOKEN_TIME).unwrap_or_default()
}

pub fn set_last_token_time(last_token_time: U256) {
    set_key(FEE_DISTRIBUTOR_LAST_TOKEN_TIME, last_token_time);
}

pub fn get_voting_escrow() -> Key {
    get_key(FEE_DISTRIBUTOR_VOTING_ESCROW).unwrap_or(zero_address())
}

pub fn set_voting_escrow(voting_escrow: Key) {
    set_key(FEE_DISTRIBUTOR_VOTING_ESCROW, voting_escrow);
}

pub fn get_token() -> Key {
    get_key(FEE_DISTRIBUTOR_TOKEN).unwrap_or(zero_address())
}

pub fn set_token(token: Key) {
    set_key(FEE_DISTRIBUTOR_TOKEN, token);
}

pub fn get_total_received() -> U256 {
    get_key(FEE_DISTRIBUTOR_TOTAL_RECEIVED).unwrap_or_default()
}

pub fn set_total_received(total_received: U256) {
    set_key(FEE_DISTRIBUTOR_TOTAL_RECEIVED, total_received);
}

pub fn get_token_last_balance() -> U256 {
    get_key(FEE_DISTRIBUTOR_TOKEN_LAST_BALANCE).unwrap_or_default()
}

pub fn set_token_last_balance(token_last_balance: U256) {
    set_key(FEE_DISTRIBUTOR_TOKEN_LAST_BALANCE, token_last_balance);
}

pub fn get_admin() -> Key {
    get_key(FEE_DISTRIBUTOR_ADMIN).unwrap_or(zero_address())
}

pub fn set_admin(admin: Key) {
    set_key(FEE_DISTRIBUTOR_ADMIN, admin);
}

pub fn get_future_admin() -> Key {
    get_key(FEE_DISTRIBUTOR_FUTURE_ADMIN).unwrap_or(zero_address())
}

pub fn set_future_admin(future_admin: Key) {
    set_key(FEE_DISTRIBUTOR_FUTURE_ADMIN, future_admin);
}

pub fn get_can_checkpoint_token() -> bool {
    get_key(FEE_DISTRIBUTOR_CAN_CHECKPOINT_TOKEN).unwrap_or_default()
}

pub fn set_can_checkpoint_token(can_checkpoint_token: bool) {
    set_key(FEE_DISTRIBUTOR_CAN_CHECKPOINT_TOKEN, can_checkpoint_token);
}

pub fn get_emergency_return() -> Key {
    get_key(FEE_DISTRIBUTOR_EMERGENCY_RETURN).unwrap_or(zero_address())
}

pub fn set_emergency_return(emergency_return: Key) {
    set_key(FEE_DISTRIBUTOR_EMERGENCY_RETURN, emergency_return);
}

pub fn get_is_killed() -> bool {
    get_key(FEE_DISTRIBUTOR_IS_KILLED).unwrap_or_default()
}

pub fn set_is_killed(is_killed: bool) {
    set_key(FEE_DISTRIBUTOR_IS_KILLED, is_killed);
}

pub fn get_lock() -> bool {
    get_key(FEE_DISTRIBUTOR_LOCK).unwrap_or_default()
}

pub fn set_lock(lock: bool) {
    set_key(FEE_DISTRIBUTOR_LOCK, lock);
}

pub fn get_contract_hash() -> ContractHash {
    get_key(FEE_DISTRIBUTOR_CONTRACT_HASH).unwrap_or_default()
}

pub fn set_contract_hash(contract_hash: ContractHash) {
    set_key(FEE_DISTRIBUTOR_CONTRACT_HASH, contract_hash);
}

pub fn get_package_hash() -> ContractPackageHash {
    get_key(FEE_DISTRIBUTOR_PACKAGE_HASH).unwrap_or_default()
}

pub fn set_package_hash(package_hash: ContractPackageHash) {
    set_key(FEE_DISTRIBUTOR_PACKAGE_HASH, package_hash);
}

pub fn js_ret<T: CLTyped + ToBytes>(ret: T) {
    set_key(FEE_DISTRIBUTOR_RESULT, ret);
}
