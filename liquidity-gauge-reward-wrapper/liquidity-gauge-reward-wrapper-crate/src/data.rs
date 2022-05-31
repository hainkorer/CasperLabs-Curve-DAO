use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_types::{
    bytesrepr::ToBytes, CLTyped, ContractHash, ContractPackageHash, Key, U128, U256,
};
use casper_types_derive::{CLTyped, FromBytes, ToBytes};
use common::keys::*;
use contract_utils::{get_key, set_key, Dict};

pub const TEN_E_NINE: u128 = 1000000000;

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
            dict: Dict::instance(ALLOWANCES),
        }
    }

    pub fn init() {
        Dict::init(ALLOWANCES)
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
            dict: Dict::instance(APPROVED_TO_DEPOSIT),
        }
    }

    pub fn init() {
        Dict::init(APPROVED_TO_DEPOSIT)
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
            dict: Dict::instance(BALANCE_OF),
        }
    }

    pub fn init() {
        Dict::init(BALANCE_OF)
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
            dict: Dict::instance(CLAIMABLE_CRV),
        }
    }

    pub fn init() {
        Dict::init(CLAIMABLE_CRV)
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
            dict: Dict::instance(REWARD_INTEGRAL_FOR),
        }
    }

    pub fn init() {
        Dict::init(REWARD_INTEGRAL_FOR)
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
            dict: Dict::instance(CRV_INTEGRAL_FOR),
        }
    }

    pub fn init() {
        Dict::init(CRV_INTEGRAL_FOR)
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
            dict: Dict::instance(CLAIMABLE_REWARDS),
        }
    }

    pub fn init() {
        Dict::init(CLAIMABLE_REWARDS)
    }

    pub fn get(&self, key: &Key) -> U256 {
        self.dict.get_by_key(key).unwrap_or_default()
    }

    pub fn set(&self, key: &Key, value: U256) {
        self.dict.set_by_key(key, value);
    }
}
pub fn name() -> String {
    get_key(NAME).unwrap_or_revert()
}

pub fn set_name(name: String) {
    set_key(NAME, name);
}

pub fn symbol() -> String {
    get_key(SYMBOL).unwrap_or_revert()
}

pub fn set_symbol(symbol: String) {
    set_key(SYMBOL, symbol);
}

pub fn decimals() -> u8 {
    get_key(DECIMALS).unwrap_or_revert()
}

pub fn set_decimals(decimals: u8) {
    set_key(DECIMALS, decimals);
}

pub fn get_minter() -> Key {
    get_key(MINTER).unwrap_or(zero_address())
}

pub fn set_minter(minter: Key) {
    set_key(MINTER, minter);
}

pub fn get_crv_token() -> Key {
    get_key(CRV_TOKEN).unwrap_or(zero_address())
}

pub fn set_crv_token(crv_token: Key) {
    set_key(CRV_TOKEN, crv_token);
}

pub fn get_lp_token() -> Key {
    get_key(LP_TOKEN).unwrap_or(zero_address())
}

pub fn set_lp_token(lp_token: Key) {
    set_key(LP_TOKEN, lp_token);
}
pub fn get_gauge() -> Key {
    get_key(GAUGE).unwrap_or(zero_address())
}

pub fn set_gauge(gauge: Key) {
    set_key(GAUGE, gauge);
}

pub fn get_total_supply() -> U256 {
    get_key(TOTAL_SUPPLY).unwrap_or_default()
}

pub fn set_total_supply(total_supply: U256) {
    set_key(TOTAL_SUPPLY, total_supply);
}
pub fn get_rewarded_token() -> Key {
    get_key(REWARDED_TOKEN).unwrap_or(zero_address())
}

pub fn set_rewarded_token(rewarded_token: Key) {
    set_key(REWARDED_TOKEN, rewarded_token);
}

pub fn get_reward_integral() -> U256 {
    get_key(REWARD_INTEGRAL).unwrap_or_default()
}

pub fn set_reward_integral(reward_integral: U256) {
    set_key(REWARD_INTEGRAL, reward_integral);
}
pub fn get_crv_integral() -> U256 {
    get_key(CRV_INTEGRAL).unwrap_or_default()
}

pub fn set_crv_integral(crv_integral: U256) {
    set_key(CRV_INTEGRAL, crv_integral);
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

pub fn get_is_killed() -> bool {
    get_key(IS_KILLED).unwrap_or_default()
}

pub fn set_is_killed(is_killed: bool) {
    set_key(IS_KILLED, is_killed);
}

pub fn get_lock() -> bool {
    get_key(LOCK).unwrap_or_default()
}

pub fn set_lock(lock: bool) {
    set_key(LOCK, lock);
}

pub fn get_contract_hash() -> ContractHash {
    get_key(SELF_CONTRACT_HASH).unwrap_or_default()
}

pub fn set_contract_hash(contract_hash: ContractHash) {
    set_key(SELF_CONTRACT_HASH, contract_hash);
}

pub fn get_package_hash() -> ContractPackageHash {
    get_key(SELF_CONTRACT_PACKAGE_HASH).unwrap_or_default()
}

pub fn set_package_hash(package_hash: ContractPackageHash) {
    set_key(SELF_CONTRACT_PACKAGE_HASH, package_hash);
}

pub fn js_ret<T: CLTyped + ToBytes>(ret: T) {
    set_key(RESULT, ret);
}
