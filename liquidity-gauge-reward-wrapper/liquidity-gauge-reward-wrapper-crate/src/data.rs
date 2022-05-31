use alloc::{string::String, vec::Vec};
use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_types::{bytesrepr::ToBytes, CLTyped, ContractHash, ContractPackageHash, Key, U256};
use casper_types_derive::{CLTyped, FromBytes, ToBytes};
use common::keys::*;
use contract_utils::{get_key, set_key, Dict};

pub const REWARD_WRAPPER_TEN_E_NINE: u128 = 1000000000;

pub fn zero_address() -> Key {
    Key::from_formatted_str(
        "hash-0000000000000000000000000000000000000000000000000000000000000000".into(),
    )
    .unwrap()
}

pub struct Allowances {
    dict: Dict,
}

impl Allowances {
    pub fn instance() -> Allowances {
        Allowances {
            dict: Dict::instance(REWARD_WRAPPER_ALLOWANCES),
        }
    }

    pub fn init() {
        Dict::init(REWARD_WRAPPER_ALLOWANCES)
    }

    pub fn get(&self, owner: &Key, spender: &Key) -> U256 {
        self.dict.get_by_keys((owner, spender)).unwrap_or_default()
    }

    pub fn set(&self, owner: &Key, spender: &Key, value: U256) {
        self.dict.set_by_keys((owner, spender), value);
    }
}

// caller -> recipient -> can deposit?

#[derive(Clone, Copy, CLTyped, ToBytes, FromBytes, Default)]
pub struct ApprovedToDeposit {
    dict: Dict,
}

impl ApprovedToDeposit {
    pub fn instance() -> ApprovedToDeposit {
        ApprovedToDeposit {
            dict: Dict::instance(REWARD_WRAPPER_APPROVED_TO_DEPOSIT),
        }
    }

    pub fn init() {
        Dict::init(REWARD_WRAPPER_APPROVED_TO_DEPOSIT)
    }

    pub fn get(&self, key_1: &Key, key_2: &Key) -> bool {
        self.dict.get_by_keys((key_1, key_2)).unwrap_or_default()
    }

    pub fn set(&self, key_1: &Key, key_2: &Key, value: bool) {
        self.dict.set_by_keys((key_1, key_2), value);
    }
}

#[derive(Clone, Copy, CLTyped, ToBytes, FromBytes, Default)]
pub struct BalanceOf {
    dict: Dict,
}

impl BalanceOf {
    pub fn instance() -> BalanceOf {
        BalanceOf {
            dict: Dict::instance(REWARD_WRAPPER_BALANCE_OF),
        }
    }

    pub fn init() {
        Dict::init(REWARD_WRAPPER_BALANCE_OF)
    }

    pub fn get(&self, key: &Key) -> U256 {
        self.dict.get_by_key(key).unwrap_or_default()
    }

    pub fn set(&self, key: &Key, value: U256) {
        self.dict.set_by_key(key, value);
    }
}

#[derive(Clone, Copy, CLTyped, ToBytes, FromBytes, Default)]
pub struct ClaimableCrv {
    dict: Dict,
}

impl ClaimableCrv {
    pub fn instance() -> ClaimableCrv {
        ClaimableCrv {
            dict: Dict::instance(REWARD_WRAPPER_CLAIMABLE_CRV),
        }
    }

    pub fn init() {
        Dict::init(REWARD_WRAPPER_CLAIMABLE_CRV)
    }

    pub fn get(&self, key: &Key) -> U256 {
        self.dict.get_by_key(key).unwrap_or_default()
    }

    pub fn set(&self, key: &Key, value: U256) {
        self.dict.set_by_key(key, value);
    }
}
#[derive(Clone, Copy, CLTyped, ToBytes, FromBytes, Default)]
pub struct RewardIntegralFor {
    dict: Dict,
}

impl RewardIntegralFor {
    pub fn instance() -> RewardIntegralFor {
        RewardIntegralFor {
            dict: Dict::instance(REWARD_WRAPPER_REWARD_INTEGRAL_FOR),
        }
    }

    pub fn init() {
        Dict::init(REWARD_WRAPPER_REWARD_INTEGRAL_FOR)
    }

    pub fn get(&self, key: &Key) -> U256 {
        self.dict.get_by_key(key).unwrap_or_default()
    }

    pub fn set(&self, key: &Key, value: U256) {
        self.dict.set_by_key(key, value);
    }
}

#[derive(Clone, Copy, CLTyped, ToBytes, FromBytes, Default)]
pub struct CrvIntegralFor {
    dict: Dict,
}

impl CrvIntegralFor {
    pub fn instance() -> CrvIntegralFor {
        CrvIntegralFor {
            dict: Dict::instance(REWARD_WRAPPER_CRV_INTEGRAL_FOR),
        }
    }

    pub fn init() {
        Dict::init(REWARD_WRAPPER_CRV_INTEGRAL_FOR)
    }

    pub fn get(&self, key: &Key) -> U256 {
        self.dict.get_by_key(key).unwrap_or_default()
    }

    pub fn set(&self, key: &Key, value: U256) {
        self.dict.set_by_key(key, value);
    }
}

#[derive(Clone, Copy, CLTyped, ToBytes, FromBytes, Default)]
pub struct ClaimableRewards {
    dict: Dict,
}

impl ClaimableRewards {
    pub fn instance() -> ClaimableRewards {
        ClaimableRewards {
            dict: Dict::instance(REWARD_WRAPPER_CLAIMABLE_REWARDS),
        }
    }

    pub fn init() {
        Dict::init(REWARD_WRAPPER_CLAIMABLE_REWARDS)
    }

    pub fn get(&self, key: &Key) -> U256 {
        self.dict.get_by_key(key).unwrap_or_default()
    }

    pub fn set(&self, key: &Key, value: U256) {
        self.dict.set_by_key(key, value);
    }
}
pub fn name() -> String {
    get_key(REWARD_WRAPPER_NAME).unwrap_or_revert()
}

pub fn set_name(name: String) {
    set_key(REWARD_WRAPPER_NAME, name);
}

pub fn symbol() -> String {
    get_key(REWARD_WRAPPER_SYMBOL).unwrap_or_revert()
}

pub fn set_symbol(symbol: String) {
    set_key(REWARD_WRAPPER_SYMBOL, symbol);
}

pub fn decimals() -> u8 {
    get_key(REWARD_WRAPPER_DECIMALS).unwrap_or_revert()
}

pub fn set_decimals(decimals: u8) {
    set_key(REWARD_WRAPPER_DECIMALS, decimals);
}

pub fn get_minter() -> Key {
    get_key(REWARD_WRAPPER_MINTER).unwrap_or(zero_address())
}

pub fn set_minter(minter: Key) {
    set_key(REWARD_WRAPPER_MINTER, minter);
}

pub fn get_crv_token() -> Key {
    get_key(REWARD_WRAPPER_CRV_TOKEN).unwrap_or(zero_address())
}

pub fn set_crv_token(crv_token: Key) {
    set_key(REWARD_WRAPPER_CRV_TOKEN, crv_token);
}

pub fn get_lp_token() -> Key {
    get_key(REWARD_WRAPPER_LP_TOKEN).unwrap_or(zero_address())
}

pub fn set_lp_token(lp_token: Key) {
    set_key(REWARD_WRAPPER_LP_TOKEN, lp_token);
}
pub fn get_gauge() -> Key {
    get_key(REWARD_WRAPPER_GAUGE).unwrap_or(zero_address())
}

pub fn set_gauge(gauge: Key) {
    set_key(REWARD_WRAPPER_GAUGE, gauge);
}

pub fn get_total_supply() -> U256 {
    get_key(REWARD_WRAPPER_TOTAL_SUPPLY).unwrap_or_default()
}

pub fn set_total_supply(total_supply: U256) {
    set_key(REWARD_WRAPPER_TOTAL_SUPPLY, total_supply);
}
pub fn get_rewarded_token() -> Key {
    get_key(REWARD_WRAPPER_REWARDED_TOKEN).unwrap_or(zero_address())
}

pub fn set_rewarded_token(rewarded_token: Key) {
    set_key(REWARD_WRAPPER_REWARDED_TOKEN, rewarded_token);
}

pub fn get_reward_integral() -> U256 {
    get_key(REWARD_WRAPPER_REWARD_INTEGRAL).unwrap_or_default()
}

pub fn set_reward_integral(reward_integral: U256) {
    set_key(REWARD_WRAPPER_REWARD_INTEGRAL, reward_integral);
}
pub fn get_crv_integral() -> U256 {
    get_key(REWARD_WRAPPER_CRV_INTEGRAL).unwrap_or_default()
}

pub fn set_crv_integral(crv_integral: U256) {
    set_key(REWARD_WRAPPER_CRV_INTEGRAL, crv_integral);
}
pub fn get_admin() -> Key {
    get_key(REWARD_WRAPPER_ADMIN).unwrap_or(zero_address())
}

pub fn set_admin(admin: Key) {
    set_key(REWARD_WRAPPER_ADMIN, admin);
}

pub fn get_future_admin() -> Key {
    get_key(REWARD_WRAPPER_FUTURE_ADMIN).unwrap_or(zero_address())
}

pub fn set_future_admin(future_admin: Key) {
    set_key(REWARD_WRAPPER_FUTURE_ADMIN, future_admin);
}

pub fn get_is_killed() -> bool {
    get_key(REWARD_WRAPPER_IS_KILLED).unwrap_or_default()
}

pub fn set_is_killed(is_killed: bool) {
    set_key(REWARD_WRAPPER_IS_KILLED, is_killed);
}

pub fn get_lock() -> bool {
    get_key(REWARD_WRAPPER_LOCK).unwrap_or_default()
}

pub fn set_lock(lock: bool) {
    set_key(REWARD_WRAPPER_LOCK, lock);
}

pub fn get_contract_hash() -> ContractHash {
    get_key(REWARD_WRAPPER_CONTRACT_HASH).unwrap_or_default()
}

pub fn set_contract_hash(contract_hash: ContractHash) {
    set_key(REWARD_WRAPPER_CONTRACT_HASH, contract_hash);
}

pub fn get_package_hash() -> ContractPackageHash {
    get_key(REWARD_WRAPPER_PACKAGE_HASH).unwrap_or_default()
}

pub fn set_package_hash(package_hash: ContractPackageHash) {
    set_key(REWARD_WRAPPER_PACKAGE_HASH, package_hash);
}

pub fn js_ret<T: CLTyped + ToBytes>(ret: T) {
    set_key(REWARD_WRAPPER_RESULT, ret);
}
