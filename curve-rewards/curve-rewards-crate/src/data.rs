use casper_types::{bytesrepr::ToBytes, CLTyped, ContractHash, ContractPackageHash, Key, U256};
use casperlabs_contract_utils::{get_key, set_key, Dict};
use common::keys::*;

pub const TEN_E_NINE: u128 = 1000000000;
pub const DURATION: U256 = U256([604800, 0, 0, 0]);
//Zero Address
pub fn zero_address() -> Key {
    Key::from_formatted_str("hash-0000000000000000000000000000000000000000000000000000000000000000")
        .unwrap()
}
//Dict
pub struct UserRewardPerTokenPaid {
    dict: Dict,
}

impl UserRewardPerTokenPaid {
    pub fn instance() -> UserRewardPerTokenPaid {
        UserRewardPerTokenPaid {
            dict: Dict::instance(USER_REWARD_PER_TOKEN_PAID_DICT),
        }
    }

    pub fn init() {
        Dict::init(USER_REWARD_PER_TOKEN_PAID_DICT)
    }

    pub fn get(&self, owner: &Key) -> U256 {
        self.dict.get_by_key(owner).unwrap_or_default()
    }

    pub fn set(&self, owner: &Key, value: U256) {
        self.dict.set_by_key(owner, value);
    }
}

pub struct Rewards {
    dict: Dict,
}

impl Rewards {
    pub fn instance() -> Rewards {
        Rewards {
            dict: Dict::instance(REWARDS_DICT),
        }
    }

    pub fn init() {
        Dict::init(REWARDS_DICT)
    }

    pub fn get(&self, owner: &Key) -> U256 {
        self.dict.get_by_key(owner).unwrap_or_default()
    }

    pub fn set(&self, owner: &Key, value: U256) {
        self.dict.set_by_key(owner, value);
    }
}

pub fn set_reward_rate(reward_rate: U256) {
    set_key(REWARD_RATE, reward_rate);
}
pub fn get_reward_rate() -> U256 {
    get_key(REWARD_RATE).unwrap_or_default()
}
// Period Finish
pub fn set_period_finish(period_finish: U256) {
    set_key(PERIOD_FINISH, period_finish);
}
pub fn get_period_finish() -> U256 {
    get_key(PERIOD_FINISH).unwrap_or_default()
}

pub fn set_hash(contract_hash: ContractHash) {
    set_key(SELF_CONTRACT_HASH, contract_hash);
}

pub fn set_last_update_time(last_update_time: U256) {
    set_key(LAST_UPDATE_TIME, last_update_time);
}
pub fn get_last_update_time() -> U256 {
    get_key(LAST_UPDATE_TIME).unwrap_or_default()
}

pub fn set_snx(snx: Key) {
    set_key(SNX, snx);
}
pub fn get_snx() -> Key {
    get_key(SNX).unwrap_or_else(zero_address)
}

pub fn set_reward_per_token_stored(reward_per_token_stored: U256) {
    set_key(REWARD_PER_TOKEN_STORED, reward_per_token_stored);
}
pub fn get_reward_per_token_stored() -> U256 {
    get_key(REWARD_PER_TOKEN_STORED).unwrap_or_default()
}

pub fn get_hash() -> ContractHash {
    get_key(SELF_CONTRACT_HASH).unwrap_or_default()
}
pub fn set_package_hash(package_hash: ContractPackageHash) {
    set_key(SELF_CONTRACT_PACKAGE_HASH, package_hash);
}
pub fn get_package_hash() -> ContractPackageHash {
    get_key(SELF_CONTRACT_PACKAGE_HASH).unwrap_or_default()
}
pub fn js_ret<T: CLTyped + ToBytes>(ret: T) {
    set_key(RESULT, ret);
}
