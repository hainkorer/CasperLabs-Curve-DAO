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
    bytesrepr::ToBytes, system::CallStackElement, CLTyped, ContractPackageHash, Key, URef, U128,
    U256,
};
use casper_types_derive::{CLTyped, FromBytes, ToBytes};
use contract_utils::{get_key, key_and_value_to_str, key_to_str, set_key, values_to_str, Dict};

use crate::event::GAUGECOLTROLLEREvent;

pub const GAUGE_TYPE_NAMES_DICT: &str = "gauge_type_names";
pub const GAUGE_TYPES_DICT: &str = "gauge_types_";
pub const VOTE_USER_SLOPES_DICT: &str = "vote_user_slopes";
pub const VOTE_USER_POWER_DICT: &str = "vote_user_power";
pub const LAST_USER_VOTE_DICT: &str = "last_user_vote";
pub const POINTS_WEIGHT_DICT: &str = "points_weight";
pub const CHANGES_WEIGHT_DICT: &str = "changes_weight";
pub const TIME_WEIGHT_DICT: &str = "time_weight";
pub const GAUGES_DICT: &str = "gauges";
pub const TIME_SUM_DICT: &str = "time_sum";
pub const POINTS_SUM_DICT: &str = "points_sum";
pub const CHANGES_SUM_DICT: &str = "changes_sum";
pub const POINTS_TOTAL_DICT: &str = "points_total";
pub const POINTS_TYPE_WEIGHT_DICT: &str = "points_type_weight";
pub const TIME_TYPE_WEIGHT_DICT: &str = "time_type_weight";

pub const OWNER: &str = "owner";
pub const ADMIN: &str = "admin";
pub const FUTURE_ADMIN: &str = "future_admin";
pub const TIME_TOTAL: &str = "time_total";
pub const TOKEN: &str = "token";
pub const VOTING_ESCROW: &str = "voting_escrow";
pub const REWARD_COUNT: &str = "reward_count";
pub const SELF_CONTRACT_HASH: &str = "self_contract_hash";
pub const CONTRACT_PACKAGE_HASH: &str = "contract_package_hash";
pub const N_GAUGE_TYPES: &str = "n_gauge_types";
pub const N_GAUGES: &str = "n_gauges";
pub const LAST_USER_VOTE: &str = "last_user_vote";

pub const WEEK: U256 = U256([604800, 0, 0, 0]); // all future times are rounded by week
pub const WEIGHT_VOTE_DELAY: U256 = U256([864000, 0, 0, 0]);
pub const MULTIPLIER: U256 = U256([1000000000000000000, 0, 0, 0]);

#[derive(Clone, Copy, CLTyped, ToBytes, FromBytes, Default)]
pub struct Point {
    pub bias: U256,
    pub slope: U256, // - dweight / dt
}

#[derive(Clone, Copy, CLTyped, ToBytes, FromBytes, Default)]
pub struct VotedSlope {
    pub slope: U256,
    pub power: U256,
    pub end: U256,
}

pub struct GAUGETYPENAMES {
    dict: Dict,
}

impl GAUGETYPENAMES {
    pub fn instance() -> GAUGETYPENAMES {
        GAUGETYPENAMES {
            dict: Dict::instance(GAUGE_TYPE_NAMES_DICT),
        }
    }

    pub fn init() {
        Dict::init(GAUGE_TYPE_NAMES_DICT)
    }

    pub fn get(&self, key: &U128) -> String {
        self.dict.get(&key.to_string()).unwrap_or_default()
    }

    pub fn set(&self, key: &U128, value: String) {
        self.dict.set(&key.to_string(), value);
    }
}

pub struct GAUGETYPES_ {
    dict: Dict,
}

impl GAUGETYPES_ {
    pub fn instance() -> GAUGETYPES_ {
        GAUGETYPES_ {
            dict: Dict::instance(GAUGE_TYPES_DICT),
        }
    }

    pub fn init() {
        Dict::init(GAUGE_TYPES_DICT)
    }

    pub fn get(&self, owner: &Key) -> U128 {
        self.dict.get(&key_to_str(owner)).unwrap_or_default()
    }

    pub fn set(&self, owner: &Key, value: U128) {
        self.dict.set(&key_to_str(owner), value);
    }
}

pub struct VOTEUSERSLOPES {
    dict: Dict,
}

impl VOTEUSERSLOPES {
    pub fn instance() -> VOTEUSERSLOPES {
        VOTEUSERSLOPES {
            dict: Dict::instance(VOTE_USER_SLOPES_DICT),
        }
    }

    pub fn init() {
        Dict::init(VOTE_USER_SLOPES_DICT)
    }

    pub fn get(&self, owner: &Key, spender: &Key) -> VotedSlope {
        self.dict.get_by_keys((owner, spender)).unwrap_or_default()
    }

    pub fn set(&self, owner: &Key, spender: &Key, value: VotedSlope) {
        self.dict.set_by_keys((owner, spender), value);
    }
}

pub struct VOTEUSERPOWER {
    dict: Dict,
}

impl VOTEUSERPOWER {
    pub fn instance() -> VOTEUSERPOWER {
        VOTEUSERPOWER {
            dict: Dict::instance(VOTE_USER_POWER_DICT),
        }
    }

    pub fn init() {
        Dict::init(VOTE_USER_POWER_DICT)
    }

    pub fn get(&self, owner: &Key) -> U256 {
        self.dict.get(&key_to_str(owner)).unwrap_or_default()
    }

    pub fn set(&self, owner: &Key, value: U256) {
        self.dict.set(&key_to_str(owner), value);
    }
}

pub struct LASTUSERVOTE {
    dict: Dict,
}

impl LASTUSERVOTE {
    pub fn instance() -> LASTUSERVOTE {
        LASTUSERVOTE {
            dict: Dict::instance(LAST_USER_VOTE_DICT),
        }
    }

    pub fn init() {
        Dict::init(LAST_USER_VOTE_DICT)
    }

    pub fn get(&self, owner: &Key, spender: &Key) -> U256 {
        self.dict.get_by_keys((owner, spender)).unwrap_or_default()
    }

    pub fn set(&self, owner: &Key, spender: &Key, value: U256) {
        self.dict.set_by_keys((owner, spender), value);
    }
}

pub struct POINTSWEIGHT {
    dict: Dict,
}

impl POINTSWEIGHT {
    pub fn instance() -> POINTSWEIGHT {
        POINTSWEIGHT {
            dict: Dict::instance(POINTS_WEIGHT_DICT),
        }
    }

    pub fn init() {
        Dict::init(POINTS_WEIGHT_DICT)
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

pub struct CHANGESWEIGHT {
    dict: Dict,
}

impl CHANGESWEIGHT {
    pub fn instance() -> CHANGESWEIGHT {
        CHANGESWEIGHT {
            dict: Dict::instance(CHANGES_WEIGHT_DICT),
        }
    }

    pub fn init() {
        Dict::init(CHANGES_WEIGHT_DICT)
    }

    pub fn get(&self, key: &Key, _key: &U256) -> U256 {
        let key_: String = key_and_value_to_str(key, _key);
        self.dict.get(key_.as_str()).unwrap_or_default()
    }

    pub fn set(&self, key: &Key, _key: &U256, value: U256) {
        let key_: String = key_and_value_to_str(key, _key);
        self.dict.set(key_.as_str(), value);
    }
}

pub struct TIMEWEIGHT {
    dict: Dict,
}

impl TIMEWEIGHT {
    pub fn instance() -> TIMEWEIGHT {
        TIMEWEIGHT {
            dict: Dict::instance(TIME_WEIGHT_DICT),
        }
    }

    pub fn init() {
        Dict::init(TIME_WEIGHT_DICT)
    }

    pub fn get(&self, owner: &Key) -> U256 {
        self.dict.get(&key_to_str(owner)).unwrap_or_default()
    }

    pub fn set(&self, owner: &Key, value: U256) {
        self.dict.set(&key_to_str(owner), value);
    }
}
pub struct GAUGES {
    dict: Dict,
    length: U256,
}

impl GAUGES {
    pub fn instance() -> GAUGES {
        GAUGES {
            dict: Dict::instance(GAUGES_DICT),
            length: 0.into(),
        }
    }

    pub fn init() {
        Dict::init(GAUGES_DICT)
    }

    pub fn get(&self, indx: &U256) -> Key {
        self.dict.get(indx.to_string().as_str()).unwrap_or_revert()
    }

    pub fn set(&self, indx: &U256, value: Key) {
        self.dict.set(indx.to_string().as_str(), value);
    }

    pub fn push(&mut self, value: Key) {
        self.dict.set(self.length.to_string().as_str(), value);
        self.length = self.length.checked_add(1.into()).unwrap_or_revert();
    }
}

pub struct TIMESUM {
    dict: Dict,
    length: U256,
}

impl TIMESUM {
    pub fn instance() -> TIMESUM {
        TIMESUM {
            dict: Dict::instance(TIME_SUM_DICT),
            length: 0.into(),
        }
    }

    pub fn init() {
        Dict::init(TIME_SUM_DICT)
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

pub struct POINTSSUM {
    dict: Dict,
}

impl POINTSSUM {
    pub fn instance() -> POINTSSUM {
        POINTSSUM {
            dict: Dict::instance(POINTS_SUM_DICT),
        }
    }

    pub fn init() {
        Dict::init(POINTS_SUM_DICT)
    }

    pub fn get(&self, key: &U128, _key: &U256) -> Point {
        let key_: String = values_to_str(&U256::from(key.as_u128()), _key);
        self.dict.get(key_.as_str()).unwrap_or_default()
    }

    pub fn set(&self, key: &U128, _key: &U256, value: Point) {
        let key_: String = values_to_str(&U256::from(key.as_u128()), _key);
        self.dict.set(key_.as_str(), value);
    }
}

pub struct CHANGESSUM {
    dict: Dict,
}

impl CHANGESSUM {
    pub fn instance() -> CHANGESSUM {
        CHANGESSUM {
            dict: Dict::instance(CHANGES_SUM_DICT),
        }
    }

    pub fn init() {
        Dict::init(CHANGES_SUM_DICT)
    }

    pub fn get(&self, key: &U128, _key: &U256) -> U256 {
        let key_: String = values_to_str(&U256::from(key.as_u128()), _key);
        self.dict.get(key_.as_str()).unwrap_or_default()
    }

    pub fn set(&self, key: &U128, _key: &U256, value: U256) {
        let key_: String = values_to_str(&U256::from(key.as_u128()), _key);
        self.dict.set(key_.as_str(), value);
    }
}

pub struct POINTSTOTAL {
    dict: Dict,
}

impl POINTSTOTAL {
    pub fn instance() -> POINTSTOTAL {
        POINTSTOTAL {
            dict: Dict::instance(POINTS_TOTAL_DICT),
        }
    }

    pub fn init() {
        Dict::init(POINTS_TOTAL_DICT)
    }

    pub fn get(&self, key: &U256) -> U256 {
        self.dict.get(&key.to_string()).unwrap_or_default()
    }

    pub fn set(&self, key: &U256, value: U256) {
        self.dict.set(&key.to_string(), value);
    }
}

pub struct POINTSTYPEWEIGHT {
    dict: Dict,
}

impl POINTSTYPEWEIGHT {
    pub fn instance() -> POINTSTYPEWEIGHT {
        POINTSTYPEWEIGHT {
            dict: Dict::instance(POINTS_TYPE_WEIGHT_DICT),
        }
    }

    pub fn init() {
        Dict::init(POINTS_TYPE_WEIGHT_DICT)
    }

    pub fn get(&self, key: &U128, _key: &U256) -> U256 {
        let key_: String = values_to_str(&U256::from(key.as_u128()), _key);
        self.dict.get(key_.as_str()).unwrap_or_default()
    }

    pub fn set(&self, key: &U128, _key: &U256, value: U256) {
        let key_: String = values_to_str(&U256::from(key.as_u128()), _key);
        self.dict.set(key_.as_str(), value);
    }
}

pub struct TIMETYPEWEIGHT {
    dict: Dict,
    length: U256,
}

impl TIMETYPEWEIGHT {
    pub fn instance() -> TIMETYPEWEIGHT {
        TIMETYPEWEIGHT {
            dict: Dict::instance(TIME_TYPE_WEIGHT_DICT),
            length: 0.into(),
        }
    }

    pub fn init() {
        Dict::init(TIME_TYPE_WEIGHT_DICT)
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

pub fn zero_address() -> Key {
    Key::from_formatted_str(
        "hash-0000000000000000000000000000000000000000000000000000000000000000".into(),
    )
    .unwrap()
}
pub fn time_total() -> U256 {
    get_key(TIME_TOTAL).unwrap_or_revert()
}

pub fn set_time_total(time_total: U256) {
    set_key(TIME_TOTAL, time_total);
}

pub fn owner() -> Key {
    get_key(OWNER).unwrap_or_revert()
}

pub fn set_owner(owner: Key) {
    set_key(OWNER, owner);
}

pub fn admin() -> Key {
    get_key(ADMIN).unwrap_or_revert()
}

pub fn set_admin(admin: Key) {
    set_key(ADMIN, admin);
}

pub fn future_admin() -> Key {
    get_key(FUTURE_ADMIN).unwrap_or_revert()
}

pub fn set_future_admin(future_admin: Key) {
    set_key(FUTURE_ADMIN, future_admin);
}

pub fn token() -> Key {
    get_key(TOKEN).unwrap_or_revert()
}

pub fn set_token(token: Key) {
    set_key(TOKEN, token);
}

pub fn n_gauge_types() -> U128 {
    get_key(N_GAUGE_TYPES).unwrap_or_revert()
}

pub fn set_n_gauge_types(n_gauge_types: U128) {
    set_key(N_GAUGE_TYPES, n_gauge_types);
}
pub fn n_gauges() -> U128 {
    get_key(N_GAUGES).unwrap_or_revert()
}

pub fn set_n_gauges(n_gauges: U128) {
    set_key(N_GAUGES, n_gauges);
}

pub fn voting_escrow() -> Key {
    get_key(VOTING_ESCROW).unwrap_or_revert()
}

pub fn set_voting_escrow(voting_escrow: Key) {
    set_key(VOTING_ESCROW, voting_escrow);
}

pub fn reward_count() -> U256 {
    get_key(REWARD_COUNT).unwrap_or_default()
}

pub fn set_reward_count(reward_count: U256) {
    set_key(REWARD_COUNT, reward_count);
}

pub fn set_hash(contract_hash: Key) {
    set_key(SELF_CONTRACT_HASH, contract_hash);
}

pub fn get_hash() -> Key {
    get_key(SELF_CONTRACT_HASH).unwrap_or_revert()
}
pub fn set_package_hash(package_hash: ContractPackageHash) {
    set_key(CONTRACT_PACKAGE_HASH, package_hash);
}

pub fn get_package_hash() -> ContractPackageHash {
    get_key(CONTRACT_PACKAGE_HASH).unwrap_or_revert()
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

pub fn emit(event: &GAUGECOLTROLLEREvent) {
    let mut events = Vec::new();
    let package = contract_package_hash();
    match event {
        GAUGECOLTROLLEREvent::Mint {
            recipient,
            token_ids,
        } => {
            for token_id in token_ids {
                let mut param = BTreeMap::new();
                param.insert(CONTRACT_PACKAGE_HASH, package.to_string());
                param.insert("event_type", "gauge_controller_mint_remove_one".to_string());
                param.insert("recipient", recipient.to_string());
                param.insert("token_id", token_id.to_string());
                events.push(param);
            }
        }
        GAUGECOLTROLLEREvent::Burn { owner, token_ids } => {
            for token_id in token_ids {
                let mut param = BTreeMap::new();
                param.insert(CONTRACT_PACKAGE_HASH, package.to_string());
                param.insert("event_type", "gauge_controller_burn_remove_one".to_string());
                param.insert("owner", owner.to_string());
                param.insert("token_id", token_id.to_string());
                events.push(param);
            }
        }
        GAUGECOLTROLLEREvent::Approve {
            owner,
            spender,
            token_ids,
        } => {
            for token_id in token_ids {
                let mut param = BTreeMap::new();
                param.insert(CONTRACT_PACKAGE_HASH, package.to_string());
                param.insert("event_type", "gauge_controller_approve_token".to_string());
                param.insert("owner", owner.to_string());
                param.insert("spender", spender.to_string());
                param.insert("token_id", token_id.to_string());
                events.push(param);
            }
        }
        GAUGECOLTROLLEREvent::Transfer {
            sender,
            recipient,
            token_ids,
        } => {
            for token_id in token_ids {
                let mut param = BTreeMap::new();
                param.insert(CONTRACT_PACKAGE_HASH, package.to_string());
                param.insert("event_type", "gauge_controller_transfer_token".to_string());
                param.insert("sender", sender.to_string());
                param.insert("recipient", recipient.to_string());
                param.insert("token_id", token_id.to_string());
                events.push(param);
            }
        }
        GAUGECOLTROLLEREvent::MetadataUpdate { token_id } => {
            let mut param = BTreeMap::new();
            param.insert(CONTRACT_PACKAGE_HASH, package.to_string());
            param.insert("event_type", "gauge_controller_metadata_update".to_string());
            param.insert("token_id", token_id.to_string());
            events.push(param);
        }
    };
    for param in events {
        let _: URef = storage::new_uref(param);
    }
}