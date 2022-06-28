use crate::event::REWARDONLYGAUGEEvent;
use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};
use casper_contract::{
    contract_api::{runtime::get_call_stack, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    bytesrepr::Bytes, system::CallStackElement, ContractPackageHash, Key, URef, U256,
};
use casper_types_derive::{CLTyped, FromBytes, ToBytes};
use common::keys::*;
use casperlabs_contract_utils::{get_key, key_to_str, set_key, Dict};

pub const MAX_REWARDS: U256 = U256([8, 0, 0, 0]);
pub const CLAIM_FREQUENCY: U256 = U256([3600, 0, 0, 0]);

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
pub struct Balances {
    dict: Dict,
}

impl Balances {
    pub fn instance() -> Balances {
        Balances {
            dict: Dict::instance(BALANCES_DICT),
        }
    }

    pub fn init() {
        Dict::init(BALANCES_DICT)
    }

    pub fn get(&self, owner: &Key) -> U256 {
        self.dict.get(&key_to_str(owner)).unwrap_or_default()
    }

    pub fn set(&self, owner: &Key, value: U256) {
        self.dict.set(&key_to_str(owner), value);
    }
}

pub struct Allowances {
    dict: Dict,
}

impl Allowances {
    pub fn instance() -> Allowances {
        Allowances {
            dict: Dict::instance(ALLOWANCES_DICT),
        }
    }

    pub fn init() {
        Dict::init(ALLOWANCES_DICT)
    }

    pub fn get(&self, owner: &Key, spender: &Key) -> U256 {
        self.dict.get_by_keys((owner, spender)).unwrap_or_default()
    }

    pub fn set(&self, owner: &Key, spender: &Key, value: U256) {
        self.dict.set_by_keys((owner, spender), value);
    }
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
        self.dict.get(indx.to_string().as_str()).unwrap_or(zero_address())
    }

    pub fn set(&self, indx: &U256, value: Key) {
        self.dict.set(indx.to_string().as_str(), value);
    }

    pub fn push(&mut self, value: U256) {
        self.dict.set(self.length.to_string().as_str(), value);
        self.length = self.length.checked_add(1.into()).unwrap_or_revert();
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
        self.dict.get(&key_to_str(owner)).unwrap_or(zero_address())
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
        let data = ClaimDataStruct {
            claimable_amount: 0.into(),
            claimed_amount: 0.into(),
        };
        self.dict
            .get_by_keys((user, claiming_address))
            .unwrap_or(data)
    }

    pub fn set(&self, user: &Key, claiming_address: &Key, claimed_amount: ClaimDataStruct) {
        self.dict
            .set_by_keys((user, claiming_address), claimed_amount);
    }
}

pub fn set_lock(lock: u64) {
    set_key(LOCK, lock);
}

pub fn get_lock() -> u64 {
    get_key(LOCK).unwrap_or_revert()
}

pub fn name() -> String {
    get_key(NAME).unwrap_or_revert()
}

pub fn set_name(name: String) {
    set_key(NAME, name);
}

pub fn claim_sig() -> Bytes {
    get_key(CLAIM_SIG).unwrap_or_revert()
}

pub fn set_claim_sig(claim_sig: Bytes) {
    set_key(CLAIM_SIG, claim_sig);
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

pub fn total_supply() -> U256 {
    get_key(TOTAL_SUPPLY).unwrap_or_default()
}

pub fn set_total_supply(total_supply: U256) {
    set_key(TOTAL_SUPPLY, total_supply);
}
pub fn reward_data() -> RewardData {
    let data = RewardData {
        address: zero_address(),
        time_stamp: 0.into(),
    };
    // data.address = zero_address();
    // data.time_stamp = 0.into();
    get_key(REWARD_DATA).unwrap_or(data)
}

pub fn set_reward_data(reward_data: RewardData) {
    set_key(REWARD_DATA, reward_data);
}

pub fn zero_address() -> Key {
    Key::from_formatted_str(
        "hash-0000000000000000000000000000000000000000000000000000000000000000".into(),
    )
    .unwrap()
}

pub fn account_zero_address() -> Key {
    Key::from_formatted_str(
        "account-hash-0000000000000000000000000000000000000000000000000000000000000000".into(),
    )
    .unwrap()
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

pub fn set_hash(contract_hash: Key) {
    set_key(SELF_CONTRACT_HASH, contract_hash);
}

pub fn get_hash() -> Key {
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

pub fn emit(event: &REWARDONLYGAUGEEvent) {
    let mut events = Vec::new();
    let package = contract_package_hash();
    match event {
        REWARDONLYGAUGEEvent::Mint {
            recipient,
            token_ids,
        } => {
            for token_id in token_ids {
                let mut param = BTreeMap::new();
                param.insert(SELF_CONTRACT_PACKAGE_HASH, package.to_string());
                param.insert(EVENT_TYPE, "mint_remove_one".to_string());
                param.insert("recipient", recipient.to_string());
                param.insert("token_id", token_id.to_string());
                events.push(param);
            }
        }
        REWARDONLYGAUGEEvent::Burn { owner, token_ids } => {
            for token_id in token_ids {
                let mut param = BTreeMap::new();
                param.insert(SELF_CONTRACT_PACKAGE_HASH, package.to_string());
                param.insert(EVENT_TYPE, "burn_remove_one".to_string());
                param.insert("owner", owner.to_string());
                param.insert("token_id", token_id.to_string());
                events.push(param);
            }
        }
        REWARDONLYGAUGEEvent::Approve {
            owner,
            spender,
            token_ids,
        } => {
            for token_id in token_ids {
                let mut param = BTreeMap::new();
                param.insert(SELF_CONTRACT_PACKAGE_HASH, package.to_string());
                param.insert(EVENT_TYPE, "approve_token".to_string());
                param.insert("owner", owner.to_string());
                param.insert("spender", spender.to_string());
                param.insert("token_id", token_id.to_string());
                events.push(param);
            }
        }
        REWARDONLYGAUGEEvent::Transfer {
            sender,
            recipient,
            token_ids,
        } => {
            for token_id in token_ids {
                let mut param = BTreeMap::new();
                param.insert(SELF_CONTRACT_PACKAGE_HASH, package.to_string());
                param.insert(EVENT_TYPE, "transfer_token".to_string());
                param.insert("sender", sender.to_string());
                param.insert("recipient", recipient.to_string());
                param.insert("token_id", token_id.to_string());
                events.push(param);
            }
        }
        REWARDONLYGAUGEEvent::MetadataUpdate { token_id } => {
            let mut param = BTreeMap::new();
            param.insert(SELF_CONTRACT_PACKAGE_HASH, package.to_string());
            param.insert(EVENT_TYPE, "metadata_update".to_string());
            param.insert("token_id", token_id.to_string());
            events.push(param);
        }
    };
    for param in events {
        let _: URef = storage::new_uref(param);
    }
}
