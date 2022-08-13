use alloc::{string::ToString, vec::Vec};
use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_types::{
    bytesrepr::ToBytes, CLTyped, ContractHash, ContractPackageHash, Key, U128, U256,
};
use casper_types_derive::{CLTyped, FromBytes, ToBytes};
use casperlabs_contract_utils::{get_key, set_key, Dict};
use common::{keys::*, utils::*};

pub const TOKENLESS_PRODUCTION: U256 = U256([40, 0, 0, 0]);
pub const BOOST_WARMUP: U256 = U256([1209600000, 0, 0, 0]);
pub const WEEK: U256 = U256([604800000, 0, 0, 0]);

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

    pub fn get(&self, owner: &Key, spender: &Key) -> bool {
        self.dict.get_by_keys((owner, spender)).unwrap_or_default()
    }

    pub fn set(&self, owner: &Key, spender: &Key, value: bool) {
        self.dict.set_by_keys((owner, spender), value);
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
pub struct WorkingBalances {
    dict: Dict,
}

impl WorkingBalances {
    pub fn instance() -> WorkingBalances {
        WorkingBalances {
            dict: Dict::instance(WORKING_BALANCES),
        }
    }

    pub fn init() {
        Dict::init(WORKING_BALANCES)
    }

    pub fn get(&self, key: &Key) -> U256 {
        self.dict.get_by_key(key).unwrap_or_default()
    }

    pub fn set(&self, key: &Key, value: U256) {
        self.dict.set_by_key(key, value);
    }
}

/// The goal is to be able to calculate ∫(rate * balance / totalSupply dt) from 0 till checkpoint
/// All values are kept in units of being multiplied by 1e9
#[derive(CLTyped, ToBytes, FromBytes)]
pub struct PeriodTimestamp {
    dict: Dict,
    length: U256,
}

impl PeriodTimestamp {
    pub fn instance() -> PeriodTimestamp {
        PeriodTimestamp {
            dict: Dict::instance(PERIOD_TIMESTAMP),
            length: 0.into(),
        }
    }

    pub fn init() {
        Dict::init(PERIOD_TIMESTAMP)
    }

    pub fn get(&self, key: &U256) -> U256 {
        self.dict.get(key.to_string().as_str()).unwrap_or_default()
    }

    pub fn set(&self, key: &U256, value: U256) {
        self.dict.set(key.to_string().as_str(), value);
    }

    pub fn push(&mut self, value: U256) {
        self.dict.set(self.length.to_string().as_str(), value);
        self.length = self.length.checked_add(1.into()).unwrap_or_revert();
    }
}

/// 1e9 * ∫(rate(t) / totalSupply(t) dt) from 0 till checkpoint
/// bump epoch when rate() changes

#[derive(CLTyped, ToBytes, FromBytes)]
pub struct IntegrateInvSupply {
    dict: Dict,
    length: U256,
}

impl IntegrateInvSupply {
    pub fn instance() -> IntegrateInvSupply {
        IntegrateInvSupply {
            dict: Dict::instance(INTEGRATE_INV_SUPPLY),
            length: 0.into(),
        }
    }

    pub fn init() {
        Dict::init(INTEGRATE_INV_SUPPLY)
    }

    pub fn get(&self, key: &U256) -> U256 {
        self.dict.get(key.to_string().as_str()).unwrap_or_default()
    }

    pub fn set(&self, key: &U256, value: U256) {
        self.dict.set(key.to_string().as_str(), value);
    }

    pub fn push(&mut self, value: U256) {
        self.dict.set(self.length.to_string().as_str(), value);
        self.length = self.length.checked_add(1.into()).unwrap_or_revert();
    }
}

/// 1e9 * ∫(rate(t) / totalSupply(t) dt) from (last_action) till checkpoint
#[derive(Clone, Copy, CLTyped, ToBytes, FromBytes, Default)]
pub struct IntegrateInvSupplyOf {
    dict: Dict,
}

impl IntegrateInvSupplyOf {
    pub fn instance() -> IntegrateInvSupplyOf {
        IntegrateInvSupplyOf {
            dict: Dict::instance(INTEGRATE_INV_SUPPLY_OF),
        }
    }

    pub fn init() {
        Dict::init(INTEGRATE_INV_SUPPLY_OF)
    }

    pub fn get(&self, key: &Key) -> U256 {
        self.dict.get_by_key(key).unwrap_or_default()
    }

    pub fn set(&self, key: &Key, value: U256) {
        self.dict.set_by_key(key, value);
    }
}

#[derive(Clone, Copy, CLTyped, ToBytes, FromBytes, Default)]
pub struct IntegrateCheckpointOf {
    dict: Dict,
}

impl IntegrateCheckpointOf {
    pub fn instance() -> IntegrateCheckpointOf {
        IntegrateCheckpointOf {
            dict: Dict::instance(INTEGRATE_CHECKPOINT_OF),
        }
    }

    pub fn init() {
        Dict::init(INTEGRATE_CHECKPOINT_OF)
    }

    pub fn get(&self, key: &Key) -> U256 {
        self.dict.get_by_key(key).unwrap_or_default()
    }

    pub fn set(&self, key: &Key, value: U256) {
        self.dict.set_by_key(key, value);
    }
}

/// ∫(balance * rate(t) / totalSupply(t) dt) from 0 till checkpoint
/// Units: rate * t = already number of coins per address to issue
#[derive(Clone, Copy, CLTyped, ToBytes, FromBytes, Default)]
pub struct IntegrateFraction {
    dict: Dict,
}

impl IntegrateFraction {
    pub fn instance() -> IntegrateFraction {
        IntegrateFraction {
            dict: Dict::instance(INTEGRATE_FRACTION),
        }
    }

    pub fn init() {
        Dict::init(INTEGRATE_FRACTION)
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
pub struct RewardsFor {
    dict: Dict,
}

impl RewardsFor {
    pub fn instance() -> RewardsFor {
        RewardsFor {
            dict: Dict::instance(REWARDS_FOR),
        }
    }

    pub fn init() {
        Dict::init(REWARDS_FOR)
    }

    pub fn get(&self, key: &Key) -> U256 {
        self.dict.get_by_key(key).unwrap_or_default()
    }

    pub fn set(&self, key: &Key, value: U256) {
        self.dict.set_by_key(key, value);
    }
}

#[derive(Clone, Copy, CLTyped, ToBytes, FromBytes, Default)]
pub struct ClaimedRewardsFor {
    dict: Dict,
}

impl ClaimedRewardsFor {
    pub fn instance() -> ClaimedRewardsFor {
        ClaimedRewardsFor {
            dict: Dict::instance(CLAIMED_REWARDS_FOR),
        }
    }

    pub fn init() {
        Dict::init(CLAIMED_REWARDS_FOR)
    }

    pub fn get(&self, key: &Key) -> U256 {
        self.dict.get_by_key(key).unwrap_or_default()
    }

    pub fn set(&self, key: &Key, value: U256) {
        self.dict.set_by_key(key, value);
    }
}

pub fn get_period() -> U128 {
    get_key(PERIOD).unwrap_or_default()
}

pub fn set_period(period: U128) {
    set_key(PERIOD, period);
}

pub fn get_minter() -> Key {
    get_key(MINTER).unwrap_or_else(zero_address)
}

pub fn set_minter(minter: Key) {
    set_key(MINTER, minter);
}

pub fn get_crv_token() -> Key {
    get_key(CRV_TOKEN).unwrap_or_else(zero_address)
}

pub fn set_crv_token(crv_token: Key) {
    set_key(CRV_TOKEN, crv_token);
}

pub fn get_lp_token() -> Key {
    get_key(LP_TOKEN).unwrap_or_else(zero_address)
}

pub fn set_lp_token(lp_token: Key) {
    set_key(LP_TOKEN, lp_token);
}
pub fn get_controller() -> Key {
    get_key(CONTROLLER).unwrap_or_else(zero_address)
}

pub fn set_controller(controller: Key) {
    set_key(CONTROLLER, controller);
}
pub fn get_voting_escrow() -> Key {
    get_key(VOTING_ESCROW).unwrap_or_else(zero_address)
}

pub fn set_voting_escrow(voting_escrow: Key) {
    set_key(VOTING_ESCROW, voting_escrow);
}

pub fn get_total_supply() -> U256 {
    get_key(TOTAL_SUPPLY).unwrap_or_default()
}

pub fn set_total_supply(total_supply: U256) {
    set_key(TOTAL_SUPPLY, total_supply);
}

pub fn get_future_epoch_time() -> U256 {
    get_key(FUTURE_EPOCH_TIME).unwrap_or_default()
}

pub fn set_future_epoch_time(future_epoch_time: U256) {
    set_key(FUTURE_EPOCH_TIME, future_epoch_time);
}

pub fn get_working_supply() -> U256 {
    get_key(WORKING_SUPPLY).unwrap_or_default()
}

pub fn set_working_supply(working_supply: U256) {
    set_key(WORKING_SUPPLY, working_supply);
}

pub fn get_inflation_rate() -> U256 {
    get_key(INFLATION_RATE).unwrap_or_default()
}

pub fn set_inflation_rate(inflation_rate: U256) {
    set_key(INFLATION_RATE, inflation_rate);
}

pub fn get_reward_contract() -> Key {
    get_key(REWARD_CONTRACT).unwrap_or_else(zero_address)
}

pub fn set_reward_contract(reward_contract: Key) {
    set_key(REWARD_CONTRACT, reward_contract);
}

pub fn get_rewarded_token() -> Key {
    get_key(REWARDED_TOKEN).unwrap_or_else(zero_address)
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

pub fn get_admin() -> Key {
    get_key(ADMIN).unwrap_or_else(zero_address)
}

pub fn set_admin(admin: Key) {
    set_key(ADMIN, admin);
}

pub fn get_future_admin() -> Key {
    get_key(FUTURE_ADMIN).unwrap_or_else(zero_address)
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

pub fn get_is_claiming_rewards() -> bool {
    get_key(IS_CLAIMING_REWARDS).unwrap_or_default()
}

pub fn set_is_claiming_rewards(is_claiming_rewards: bool) {
    set_key(IS_CLAIMING_REWARDS, is_claiming_rewards);
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
