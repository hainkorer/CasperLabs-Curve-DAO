use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_types::{
    bytesrepr::ToBytes, CLTyped, ContractHash, ContractPackageHash, Key, U128, U256,
};
use casper_types_derive::{CLTyped, FromBytes, ToBytes};
use contract_utils::{get_key, key_and_value_to_str, set_key, Dict};

pub const DEPOSIT_FOR_TYPE: U128 = U128([0, 0]);
pub const CREATE_LOCK_TYPE: U128 = U128([1, 0]);
pub const INCREASE_LOCK_AMOUNT: U128 = U128([2, 0]);
pub const INCREASE_UNLOCK_TIME: U128 = U128([3, 0]);
pub const WEEK: U256 = U256([604800, 0, 0, 0]); // all future times are rounded by week
pub const MAXTIME: U256 = U256([126144000, 0, 0, 0]); // 4 years
pub const MULTIPLIER: U256 = U256([1000000000000000000, 0, 0, 0]);

pub const TOKEN: &str = "token";
pub const SUPPLY: &str = "supply";
pub const FUTURE_SMART_WALLET_CHECKER: &str = "future_smart_wallet_checker";
pub const SMART_WALLET_CHECKER: &str = "smart_wallet_checker";
pub const ADMIN: &str = "admin";
pub const FUTURE_ADMIN: &str = "future_admin";
pub const CONTROLLER: &str = "controller";
pub const TRANSFERS_ENABLED: &str = "transfers_enabled";
pub const NAME: &str = "name";
pub const SYMBOL: &str = "symbol";
pub const VERSION: &str = "version";
pub const DECIMALS: &str = "decimals";
pub const EPOCH: &str = "epoch";

pub const LOCK: &str = "lock";
pub const CONTRACT_HASH: &str = "contract_hash";
pub const PACKAGE_HASH: &str = "package_hash";
pub const RESULT: &str = "result";

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

#[derive(Clone, Copy, CLTyped, ToBytes, FromBytes, Default)]
pub struct LockedBalance {
    pub amount: U128,
    pub end: U256,
}

pub const LOCKED: &str = "locked";
pub struct Locked {
    dict: Dict,
}

impl Locked {
    pub fn instance() -> Locked {
        Locked {
            dict: Dict::instance(LOCKED),
        }
    }

    pub fn init() {
        Dict::init(LOCKED)
    }

    pub fn get(&self, key: &Key) -> LockedBalance {
        self.dict.get_by_key(key).unwrap_or_default()
    }

    pub fn set(&self, key: &Key, value: LockedBalance) {
        self.dict.set_by_key(key, value);
    }
}

pub const USER_POINT_HISTORY: &str = "user_point_history";
pub struct UserPointHistory {
    dict: Dict,
}

impl UserPointHistory {
    pub fn instance() -> UserPointHistory {
        UserPointHistory {
            dict: Dict::instance(USER_POINT_HISTORY),
        }
    }

    pub fn init() {
        Dict::init(USER_POINT_HISTORY)
    }

    pub fn get(&self, key: &Key, _key: &U256) -> Point {
        let key_: String = key_and_value_to_str(key, _key);
        self.dict.get(key_.as_str()).unwrap_or_default()
    }

    pub fn set(&self, key: &Key, _key: &U256, value: Point) {
        let key_: String = key_and_value_to_str(key, _key);
        self.dict.set(key_.as_str(), value);
    }
}

pub const USER_POINT_EPOCH: &str = "user_point_epoch";
pub struct UserPointEpoch {
    dict: Dict,
}

impl UserPointEpoch {
    pub fn instance() -> UserPointEpoch {
        UserPointEpoch {
            dict: Dict::instance(USER_POINT_EPOCH),
        }
    }

    pub fn init() {
        Dict::init(USER_POINT_EPOCH)
    }

    pub fn get(&self, key: &Key) -> U256 {
        self.dict.get_by_key(key).unwrap_or_default()
    }

    pub fn set(&self, key: &Key, value: U256) {
        self.dict.set_by_key(key, value);
    }
}

pub const SLOPE_CHANGES: &str = "slope_changes";
pub struct SlopeChanges {
    dict: Dict,
}

impl SlopeChanges {
    pub fn instance() -> SlopeChanges {
        SlopeChanges {
            dict: Dict::instance(SLOPE_CHANGES),
        }
    }

    pub fn init() {
        Dict::init(SLOPE_CHANGES)
    }

    pub fn get(&self, key: &U256) -> U128 {
        self.dict.get(key.to_string().as_str()).unwrap_or_default()
    }

    pub fn set(&self, key: &U256, value: U128) {
        self.dict.set(key.to_string().as_str(), value);
    }
}

pub const POINT_HISTORY: &str = "point_history";
pub struct PointHistory {
    dict: Dict,
    length: U256,
}

impl PointHistory {
    pub fn instance() -> PointHistory {
        PointHistory {
            dict: Dict::instance(POINT_HISTORY),
            length: 0.into(),
        }
    }

    pub fn init() {
        Dict::init(POINT_HISTORY)
    }

    pub fn get(&self, indx: &U256) -> Point {
        self.dict.get(indx.to_string().as_str()).unwrap_or_default()
    }

    pub fn set(&self, indx: &U256, value: Point) {
        self.dict.set(indx.to_string().as_str(), value);
    }

    pub fn push(&mut self, value: Point) {
        self.dict.set(self.length.to_string().as_str(), value);
        self.length = self.length.checked_add(1.into()).unwrap_or_revert();
    }
}

pub fn get_token() -> Key {
    get_key(TOKEN).unwrap_or(zero_address())
}

pub fn set_token(token: Key) {
    set_key(TOKEN, token);
}

pub fn get_supply() -> U256 {
    get_key(SUPPLY).unwrap_or_default()
}

pub fn set_supply(supply: U256) {
    set_key(SUPPLY, supply);
}

pub fn get_future_smart_wallet_checker() -> Key {
    get_key(FUTURE_SMART_WALLET_CHECKER).unwrap_or(zero_address())
}

pub fn set_future_smart_wallet_checker(future_smart_wallet_checker: Key) {
    set_key(FUTURE_SMART_WALLET_CHECKER, future_smart_wallet_checker);
}

pub fn get_smart_wallet_checker() -> Key {
    get_key(SMART_WALLET_CHECKER).unwrap_or(zero_address())
}

pub fn set_smart_wallet_checker(smart_wallet_checker: Key) {
    set_key(SMART_WALLET_CHECKER, smart_wallet_checker);
}

pub fn get_admin() -> Key {
    get_key(ADMIN).unwrap_or(zero_address())
}

pub fn set_admin(admin: Key) {
    set_key(ADMIN, admin);
}

pub fn get_future_admin() -> Key {
    get_key(FUTURE_ADMIN).unwrap_or(zero_address())
}

pub fn set_future_admin(future_admin: Key) {
    set_key(FUTURE_ADMIN, future_admin);
}

pub fn get_controller() -> Key {
    get_key(CONTROLLER).unwrap_or(zero_address())
}

pub fn set_controller(controller: Key) {
    set_key(CONTROLLER, controller);
}

pub fn get_transfers_enabled() -> bool {
    get_key(TRANSFERS_ENABLED).unwrap_or_default()
}

pub fn set_transfers_enabled(transfers_enabled: bool) {
    set_key(TRANSFERS_ENABLED, transfers_enabled);
}

pub fn get_name() -> String {
    get_key(NAME).unwrap_or_default()
}

pub fn set_name(name: String) {
    set_key(NAME, name);
}

pub fn get_symbol() -> String {
    get_key(SYMBOL).unwrap_or_default()
}

pub fn set_symbol(symbol: String) {
    set_key(SYMBOL, symbol);
}

pub fn get_version() -> String {
    get_key(VERSION).unwrap_or_default()
}

pub fn set_version(version: String) {
    set_key(VERSION, version);
}

pub fn get_decimals() -> U256 {
    get_key(DECIMALS).unwrap_or_default()
}

pub fn set_decimals(decimals: U256) {
    set_key(DECIMALS, decimals);
}

pub fn get_epoch() -> U256 {
    get_key(EPOCH).unwrap_or_default()
}

pub fn set_epoch(epoch: U256) {
    set_key(EPOCH, epoch);
}

pub fn get_lock() -> bool {
    get_key(LOCK).unwrap_or_default()
}

pub fn set_lock(lock: bool) {
    set_key(LOCK, lock);
}

pub fn get_contract_hash() -> ContractHash {
    get_key(CONTRACT_HASH).unwrap_or_default()
}

pub fn set_contract_hash(contract_hash: ContractHash) {
    set_key(CONTRACT_HASH, contract_hash);
}

pub fn get_package_hash() -> ContractPackageHash {
    get_key(PACKAGE_HASH).unwrap_or_default()
}

pub fn set_package_hash(package_hash: ContractPackageHash) {
    set_key(PACKAGE_HASH, package_hash);
}

pub fn js_ret<T: CLTyped + ToBytes>(ret: T) {
    set_key(RESULT, ret);
}
