use alloc::string::{String, ToString};
use alloc::vec::Vec;
use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_types::bytesrepr::Bytes;
use casper_types::{ContractHash, ContractPackageHash, Key, U128, U256};
use casper_types_derive::{CLTyped, FromBytes, ToBytes};
use common::keys::*;
use contract_utils::{get_key, key_to_str, set_key, Dict};

pub const MAX_REWARDS: U256 = U256([8, 0, 0, 0]);
pub const TOKENLESS_PRODUCTION: U256 = U256([40, 0, 0, 0]);
pub const CLAIM_FREQUENCY: U256 = U256([3600, 0, 0, 0]);
pub const WEEK: U256 = U256([604800, 0, 0, 0]);

#[allow(non_snake_case)]

pub fn zero_address() -> Key {
    Key::from_formatted_str(
        "hash-0000000000000000000000000000000000000000000000000000000000000000".into(),
    )
    .unwrap()
}

// const REWARD_DATA: &str = "reward_data";

// const MINTER: &str = "minter";
// const CRV_TOKEN: &str = "crv_token";
// const LP_TOKEN: &str = "lp_token";
// const CONTROLLER: &str = "controller";
// const VOTING_ESCROW: &str = "voting_escrow";
// const FUTURE_EPOCH_TIME: &str = "future_epoch_time";
// const TOTAL_SUPPLY: &str = "total_supply";
// const DECIMALS: &str = "decimals";

// const NAME: &str = "name";
// const SYMBOL: &str = "symbol";

// const WORKING_SUPPLY: &str = "working_supply";
// const PERIOD: &str = "period";
// const INFLATION_RATE: &str = "inflation_rate";
// const ADMIN: &str = "admin";
// const FUTURE_ADMIN: &str = "future_admin";
// const IS_KILLED: &str = "is_killed";
// const CLAIM_SIG: &str = "claim_sig";

// const CONTRACT_HASH: &str = "contract_hash";
// const PACKAGE_HASH: &str = "package_hash";
// const LOCK: &str = "lock";
// const MYVEC: &str = "myvec";

#[derive(Clone, Copy, CLTyped, ToBytes, FromBytes)]
pub struct RewardData {
    pub address: Key,
    pub time_stamp: U256,
}

#[derive(Clone, Copy, CLTyped, ToBytes, FromBytes)]
pub struct ClaimDataStruct {
    pub claimable_amount: U256,
    pub claimed_amount: U256,
}

const CLAIM_DATA_DICT: &str = "claim_data";
pub struct ClaimData {
    dict: Dict,
}

impl ClaimData {
    pub fn instance() -> ClaimData {
        ClaimData {
            dict: Dict::instance(CLAIM_DATA_DICT),
        }
    }

    pub fn init() {
        Dict::init(CLAIM_DATA_DICT)
    }

    pub fn get(&self, user: &Key, claiming_address: &Key) -> ClaimDataStruct {
        self.dict
            .get_by_keys((user, claiming_address))
            .unwrap_or_revert()
    }

    pub fn set(&self, user: &Key, claiming_address: &Key, claimed_amount: ClaimDataStruct) {
        self.dict
            .set_by_keys((user, claiming_address), claimed_amount);
    }
}
pub const REWARD_TOKENS: &str = "reward_tokens";
pub struct RewardTokens {
    dict: Dict,
    length: U256,
}

impl RewardTokens {
    pub fn instance() -> RewardTokens {
        RewardTokens {
            dict: Dict::instance(REWARD_TOKENS),
            length: 0.into(),
        }
    }

    pub fn init() {
        Dict::init(REWARD_TOKENS)
    }

    pub fn get(&self, indx: &U256) -> Key {
        self.dict.get(indx.to_string().as_str()).unwrap_or_revert()
    }

    pub fn set(&self, indx: &U256, value: Key) {
        self.dict.set(indx.to_string().as_str(), value);
    }

    pub fn push(&mut self, value: U256) {
        self.dict.set(self.length.to_string().as_str(), value);
        self.length = self.length.checked_add(1.into()).unwrap_or_revert();
    }
}

pub const REWARDS_RECEIVER: &str = "reward_reciever";

pub struct RewardsReceiver {
    dict: Dict,
}

impl RewardsReceiver {
    pub fn instance() -> RewardsReceiver {
        RewardsReceiver {
            dict: Dict::instance(REWARDS_RECEIVER),
        }
    }

    pub fn init() {
        Dict::init(REWARDS_RECEIVER)
    }

    pub fn get(&self, owner: &Key) -> Key {
        self.dict.get(&key_to_str(owner)).unwrap_or_revert()
    }

    pub fn set(&self, owner: &Key, value: Key) {
        self.dict.set(&key_to_str(owner), value);
    }
}

const BALANCE_OF: &str = "balance_of";
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

const ALLOWANCE: &str = "allowance";
pub struct Allowance {
    dict: Dict,
}

impl Allowance {
    pub fn instance() -> Allowance {
        Allowance {
            dict: Dict::instance(ALLOWANCE),
        }
    }

    pub fn init() {
        Dict::init(ALLOWANCE)
    }

    pub fn get(&self, key1: &Key, key2: &Key) -> U256 {
        self.dict.get_by_keys((key1, key2)).unwrap_or_default()
    }

    pub fn set(&self, key1: &Key, key2: &Key, value: U256) {
        self.dict.set_by_keys((key1, key2), value);
    }
}

const WORKING_BALANCES: &str = "working_balances";
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

const PERIOD_TIMESTAMP: &str = "period_timestamp";
pub struct PeriodTimestamp {
    dict: Dict,
}

impl PeriodTimestamp {
    pub fn instance() -> PeriodTimestamp {
        PeriodTimestamp {
            dict: Dict::instance(PERIOD_TIMESTAMP),
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
}

const INTEGRATE_INV_SUPPLY: &str = "integrate_inv_supply";
pub struct IntegrateInvSupply {
    dict: Dict,
}

impl IntegrateInvSupply {
    pub fn instance() -> IntegrateInvSupply {
        IntegrateInvSupply {
            dict: Dict::instance(INTEGRATE_INV_SUPPLY),
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
}

const INTEGRATE_INV_SUPPLY_OF: &str = "integrate_inv_supply_of";
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

const INTEGRATE_CHECKPOINT_OF: &str = "integrate_checkpoint_of";
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

const INTEGRATE_FRACTION: &str = "integrate_fraction";
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

// const REWARDS_RECIEVER: &str = "rewards_receiver";
// pub struct RewardsReciever {
//     dict: Dict,
// }

// impl RewardsReciever {
//     pub fn instance() -> RewardsReciever {
//         RewardsReciever {
//             dict: Dict::instance(REWARDS_RECIEVER),
//         }
//     }

//     pub fn init() {
//         Dict::init(REWARDS_RECIEVER)
//     }

//     pub fn get(&self, key: &Key) -> &Key {
//         self.dict.get_by_key(key).unwrap_or_d
//     }

//     pub fn set(&self, key1: &Key, key2: &Key) {
//         self.dict.set_by_key(key1, key2);
//     }
// }

const REWARD_INTEGRAL: &str = "reward_integral";
pub struct RewardIntegral {
    dict: Dict,
}

impl RewardIntegral {
    pub fn instance() -> RewardIntegral {
        RewardIntegral {
            dict: Dict::instance(REWARD_INTEGRAL),
        }
    }

    pub fn init() {
        Dict::init(REWARD_INTEGRAL)
    }

    pub fn get(&self, key: &Key) -> U256 {
        self.dict.get_by_key(key).unwrap_or_default()
    }

    pub fn set(&self, key: &Key, value: U256) {
        self.dict.set_by_key(key, value);
    }
}
const REWARD_INTEGRAL_FOR: &str = "reward_integral_for";
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

    pub fn get(&self, key1: &Key, key2: &Key) -> U256 {
        self.dict.get_by_keys((key1, key2)).unwrap_or_default()
    }

    pub fn set(&self, key1: &Key, key2: &Key, value: U256) {
        self.dict.set_by_keys((key1, key2), value);
    }
}
pub fn get_decimals() -> u8 {
    get_key(DECIMALS).unwrap_or_revert()
}

pub fn set_decimals(decimals: u8) {
    set_key(DECIMALS, decimals);
}

// const CLAIM_DATA: &str = "claim_data";
// pub struct ClaimData {
//     dict: Dict,
// }

// impl ClaimData {
//     pub fn instance() -> ClaimData {
//         ClaimData {
//             dict: Dict::instance(CLAIM_DATA),
//         }
//     }

//     pub fn init() {
//         Dict::init(CLAIM_DATA)
//     }

//     pub fn get(&self, key1: &Key,key2: &Key) -> U256 {
//         self.dict.get_by_keys((key1, key2)).unwrap_or_default()
//     }

//     pub fn set(&self, key1: &Key,key2: &Key ,value: U256) {
//         self.dict.set_by_keys((key1, key2), value);
//     }
// }
pub fn claim_sig() -> Bytes {
    get_key(CLAIM_SIG).unwrap_or_revert()
}

pub fn set_claim_sig(claim_sig: Bytes) {
    set_key(CLAIM_SIG, claim_sig);
}
pub fn myvec() -> Vec<Key> {
    get_key(MYVEC).unwrap_or_revert()
}
pub fn set_myvec(myvec: Vec<Key>) {
    set_key(MYVEC, myvec);
}
pub fn reward_data() -> RewardData {
    get_key(REWARD_DATA).unwrap_or_revert()
}

pub fn set_reward_data(reward_data: RewardData) {
    set_key(REWARD_DATA, reward_data);
}
pub fn set_minter(minter: Key) {
    set_key(MINTER, minter);
}

pub fn get_minter() -> Key {
    get_key(MINTER).unwrap_or_revert()
}

pub fn set_crv_token(crv_token: Key) {
    set_key(CRV_TOKEN, crv_token);
}

pub fn get_crv_token() -> Key {
    get_key(CRV_TOKEN).unwrap_or_revert()
}

pub fn set_lp_token(lp_token: Key) {
    set_key(LP_TOKEN, lp_token);
}

pub fn get_lp_token() -> Key {
    get_key(LP_TOKEN).unwrap_or_revert()
}

pub fn set_controller(controller: Key) {
    set_key(CONTROLLER, controller);
}

pub fn get_controller() -> Key {
    get_key(CONTROLLER).unwrap_or_revert()
}

pub fn set_voting_escrow(voting_escrow: Key) {
    set_key(VOTING_ESCROW, voting_escrow);
}

pub fn get_voting_escrow() -> Key {
    get_key(VOTING_ESCROW).unwrap_or_revert()
}

pub fn set_future_epoch_time(future_epoch_time: U256) {
    set_key(FUTURE_EPOCH_TIME, future_epoch_time);
}

pub fn get_future_epoch_time() -> U256 {
    get_key(FUTURE_EPOCH_TIME).unwrap_or_revert()
}
pub fn set_total_supply(total_supply: U256) {
    set_key(TOTAL_SUPPLY, total_supply);
}

pub fn get_total_supply() -> U256 {
    get_key(TOTAL_SUPPLY).unwrap_or_revert()
}

pub fn get_name() -> String {
    get_key(NAME).unwrap_or_revert()
}

pub fn set_name(name: String) {
    set_key(NAME, name);
}

pub fn set_symbol(symbol: String) {
    set_key(SYMBOL, symbol);
}

pub fn get_symbol() -> String {
    get_key(SYMBOL).unwrap_or_revert()
}
pub fn set_working_supply(working_supply: U256) {
    set_key(WORKING_SUPPLY, working_supply);
}

pub fn get_working_supply() -> U256 {
    get_key(WORKING_SUPPLY).unwrap_or_revert()
}

pub fn set_period(period: U128) {
    set_key(PERIOD, period);
}

pub fn get_period() -> U128 {
    get_key(PERIOD).unwrap_or_revert()
}

pub fn set_inflation_rate(inflation_rate: U256) {
    set_key(INFLATION_RATE, inflation_rate);
}

pub fn get_inflation_rate() -> U256 {
    get_key(INFLATION_RATE).unwrap_or_revert()
}

pub fn set_admin(admin: Key) {
    set_key(ADMIN, admin);
}

pub fn get_admin() -> Key {
    get_key(ADMIN).unwrap_or_revert()
}

pub fn set_future_admin(future_admin: Key) {
    set_key(FUTURE_ADMIN, future_admin);
}

pub fn get_future_admin() -> Key {
    get_key(FUTURE_ADMIN).unwrap_or_revert()
}

pub fn set_is_killed(is_killed: bool) {
    set_key(IS_KILLED, is_killed);
}

pub fn get_is_killed() -> bool {
    get_key(IS_KILLED).unwrap_or_revert()
}

pub fn set_contract_hash(contract_hash: ContractHash) {
    set_key(SELF_CONTRACT_HASH, contract_hash);
}

pub fn get_contract_hash() -> ContractHash {
    get_key(SELF_CONTRACT_HASH).unwrap_or_revert()
}

pub fn set_package_hash(package_hash: ContractPackageHash) {
    set_key(SELF_CONTRACT_PACKAGE_HASH, package_hash);
}

pub fn get_package_hash() -> ContractPackageHash {
    get_key(SELF_CONTRACT_PACKAGE_HASH).unwrap_or_revert()
}

pub fn set_lock(lock: bool) {
    set_key(LOCK, lock);
}

pub fn get_lock() -> bool {
    get_key(LOCK).unwrap_or_revert()
}
