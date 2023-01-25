use alloc::{
    format,
    string::{String, ToString},
    vec::Vec,
};
use casper_contract::{contract_api::runtime::get_call_stack, unwrap_or_revert::UnwrapOrRevert};
use casper_types::{system::CallStackElement, ContractHash, ContractPackageHash, Key, U256};
use casper_types_derive::{CLTyped, FromBytes, ToBytes};
use casperlabs_contract_utils::{get_key, key_to_str, set_key, Dict};
use common::{errors::*, keys::*, utils::*};

pub const MAX_REWARDS: U256 = U256([8, 0, 0, 0]);
pub const CLAIM_FREQUENCY: U256 = U256([3600000, 0, 0, 0]);

#[derive(Clone, Copy, CLTyped, ToBytes, FromBytes)]
pub struct RewardData {
    pub address: Key,
    pub time_stamp: U256,
}

#[derive(Clone, Copy, CLTyped, ToBytes, FromBytes, Default)]
pub struct ClaimDataStruct {
    pub claimable_amount: U256,
    pub claimed_amount: U256,
}

pub struct RewardTokens {
    dict: Dict,
    length: U256,
}

impl RewardTokens {
    pub fn instance() -> RewardTokens {
        RewardTokens {
            dict: Dict::instance(REWARD_TOKENS_DICT),
            length: 0.into(),
        }
    }

    pub fn init() {
        Dict::init(REWARD_TOKENS_DICT)
    }

    pub fn get(&self, indx: &U256) -> Key {
        self.dict
            .get(indx.to_string().as_str())
            .unwrap_or_else(zero_address)
    }

    pub fn set(&self, indx: &U256, value: Key) {
        self.dict.set(indx.to_string().as_str(), value);
    }

    pub fn push(&mut self, value: U256) {
        self.dict.set(self.length.to_string().as_str(), value);
        self.length = self
            .length
            .checked_add(1.into())
            .unwrap_or_revert_with(Error::RewardOnlyGaugeOverFlow5);
    }
}

pub struct RewardBalances {
    dict: Dict,
}

impl RewardBalances {
    pub fn instance() -> RewardBalances {
        RewardBalances {
            dict: Dict::instance(REWARD_BALANCES_DICT),
        }
    }

    pub fn init() {
        Dict::init(REWARD_BALANCES_DICT)
    }

    pub fn get(&self, owner: &Key) -> U256 {
        self.dict.get(&key_to_str(owner)).unwrap_or_default()
    }

    pub fn set(&self, owner: &Key, value: U256) {
        self.dict.set(&key_to_str(owner), value);
    }
}

pub struct RewardsReceiver {
    dict: Dict,
}

impl RewardsReceiver {
    pub fn instance() -> RewardsReceiver {
        RewardsReceiver {
            dict: Dict::instance(REWARDS_RECEIVER_DICT),
        }
    }

    pub fn init() {
        Dict::init(REWARDS_RECEIVER_DICT)
    }

    pub fn get(&self, owner: &Key) -> Key {
        self.dict
            .get(&key_to_str(owner))
            .unwrap_or_else(zero_address)
    }

    pub fn set(&self, owner: &Key, value: Key) {
        self.dict.set(&key_to_str(owner), value);
    }
}

pub struct RewardIntegral {
    dict: Dict,
}

impl RewardIntegral {
    pub fn instance() -> RewardIntegral {
        RewardIntegral {
            dict: Dict::instance(REWARD_INTEGRAL_DICT),
        }
    }

    pub fn init() {
        Dict::init(REWARD_INTEGRAL_DICT)
    }

    pub fn get(&self, owner: &Key) -> U256 {
        self.dict.get(&key_to_str(owner)).unwrap_or_default()
    }

    pub fn set(&self, owner: &Key, value: U256) {
        self.dict.set(&key_to_str(owner), value);
    }
}

pub struct RewardIntegralFor {
    dict: Dict,
}

impl RewardIntegralFor {
    pub fn instance() -> RewardIntegralFor {
        RewardIntegralFor {
            dict: Dict::instance(REWARD_INTEGRAL_FOR_DICT),
        }
    }

    pub fn init() {
        Dict::init(REWARD_INTEGRAL_FOR_DICT)
    }

    pub fn get(&self, reward_token: &Key, claiming_address: &Key) -> U256 {
        self.dict
            .get_by_keys((reward_token, claiming_address))
            .unwrap_or_default()
    }

    pub fn set(&self, reward_token: &Key, claiming_address: &Key, integral: U256) {
        self.dict
            .set_by_keys((reward_token, claiming_address), integral);
    }
}

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
        ClaimDataStruct {
            claimable_amount: self
                .dict
                .get(
                    hash(format!(
                        "{}{}{}{}{}",
                        CLAIM_DATA_DICT,
                        "_claimable_amount_",
                        user.to_formatted_string(),
                        "_",
                        claiming_address.to_formatted_string()
                    ))
                    .as_str(),
                )
                .unwrap_or_default(),

            claimed_amount: self
                .dict
                .get(
                    hash(format!(
                        "{}{}{}{}{}",
                        CLAIM_DATA_DICT,
                        "_claimed_amount_",
                        user.to_formatted_string(),
                        "_",
                        claiming_address.to_formatted_string()
                    ))
                    .as_str(),
                )
                .unwrap_or_default(),
        }
    }

    pub fn set(&self, user: &Key, claiming_address: &Key, value: ClaimDataStruct) {
        self.dict.set(
            hash(format!(
                "{}{}{}{}{}",
                CLAIM_DATA_DICT,
                "_claimable_amount_",
                user.to_formatted_string(),
                "_",
                claiming_address.to_formatted_string()
            ))
            .as_str(),
            value.claimable_amount,
        );

        self.dict.set(
            hash(format!(
                "{}{}{}{}{}",
                CLAIM_DATA_DICT,
                "_claimed_amount_",
                user.to_formatted_string(),
                "_",
                claiming_address.to_formatted_string()
            ))
            .as_str(),
            value.claimed_amount,
        );
    }
}

pub fn set_lock(lock: u64) {
    set_key(LOCK, lock);
}

pub fn get_lock() -> u64 {
    get_key(LOCK).unwrap_or_revert()
}

pub fn claim_sig() -> String {
    get_key(CLAIM_SIG).unwrap_or_revert()
}

pub fn set_claim_sig(claim_sig: String) {
    set_key(CLAIM_SIG, claim_sig);
}

pub fn reward_data() -> RewardData {
    RewardData {
        address: get_key(hash(format!("{}{}", REWARD_DATA, "_address")).as_str())
            .unwrap_or_else(zero_address),
        time_stamp: get_key(hash(format!("{}{}", REWARD_DATA, "_time_stamp")).as_str())
            .unwrap_or_default(),
    }
}

pub fn set_reward_data(reward_data: RewardData) {
    set_key(
        hash(format!("{}{}", REWARD_DATA, "_address")).as_str(),
        reward_data.address,
    );
    set_key(
        hash(format!("{}{}", REWARD_DATA, "_time_stamp")).as_str(),
        reward_data.time_stamp,
    );
}

pub fn admin() -> Key {
    get_key(ADMIN).unwrap_or_revert()
}

pub fn set_admin(admin: Key) {
    set_key(ADMIN, admin);
}
pub fn lp_token() -> Key {
    get_key(LP_TOKEN).unwrap_or_revert()
}

pub fn set_lp_token(admin: Key) {
    set_key(LP_TOKEN, admin);
}

pub fn future_admin() -> Key {
    get_key(FUTURE_ADMIN).unwrap_or_revert()
}

pub fn set_future_admin(future_admin: Key) {
    set_key(FUTURE_ADMIN, future_admin);
}

pub fn set_hash(contract_hash: ContractHash) {
    set_key(SELF_CONTRACT_HASH, contract_hash);
}

pub fn get_hash() -> ContractHash {
    get_key(SELF_CONTRACT_HASH).unwrap_or_revert()
}
pub fn set_package_hash(package_hash: ContractPackageHash) {
    set_key(SELF_CONTRACT_PACKAGE_HASH, package_hash);
}

pub fn get_package_hash() -> ContractPackageHash {
    get_key(SELF_CONTRACT_PACKAGE_HASH).unwrap_or_revert()
}

pub fn contract_package_hash() -> ContractPackageHash {
    let call_stacks = get_call_stack();
    let last_entry = call_stacks.last().unwrap_or_revert();
    let package_hash: Option<ContractPackageHash> = match last_entry {
        CallStackElement::StoredContract {
            contract_package_hash,
            contract_hash: _,
        } => Some(*contract_package_hash),
        _ => None,
    };
    package_hash.unwrap_or_revert()
}
