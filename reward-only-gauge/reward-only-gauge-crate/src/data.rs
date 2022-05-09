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
    bytesrepr::{Bytes, ToBytes},
    system::CallStackElement,
    CLTyped, ContractPackageHash, Key, URef, U128, U256,
};
use casper_types_derive::{CLTyped, FromBytes, ToBytes};
use contract_utils::{get_key, key_and_value_to_str, key_to_str, set_key, values_to_str, Dict};

use crate::event::REWARDONLYGAUGEEvent;

pub const BALANCES_DICT: &str = "balances";
pub const NONCES_DICT: &str = "nonces";
pub const ALLOWANCES_DICT: &str = "allowances";
pub const REWARD_TOKENS_DICT: &str = "reward_tokens";
pub const REWARD_BALANCES_DICT: &str = "reward_balances";
pub const REWARDS_RECEIVER_DICT: &str = "reward_receiver";
pub const REWARD_INTEGRAL_DICT: &str = "reward_integral";
pub const REWARD_INTEGRAL_FOR_DICT: &str = "reward_integral_for";
pub const CLAIM_DATA_DICT: &str = "claim_data";
pub const CLAIM_SIG_DICT: &str = "claim_sig";

pub const NAME: &str = "name";
pub const SYMBOL: &str = "symbol";
pub const DECIMALS: &str = "decimals";
pub const TOTAL_SUPPLY: &str = "total_supply";

pub const ADMIN: &str = "admin";
pub const LP_TOKEN: &str = "lp_token";
pub const FUTURE_ADMIN: &str = "future_admin";
pub const REWARD_DATA: &str = "reward_data";
pub const SELF_CONTRACT_HASH: &str = "self_contract_hash";
pub const CONTRACT_PACKAGE_HASH: &str = "contract_package_hash";
pub const MAX_REWARDS: U256 = U256([8, 0, 0, 0]);
pub const CLAIM_FREQUENCY: U256 = U256([3600, 0, 0, 0]);

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
        self.dict.get(&key_to_str(owner)).unwrap_or_revert()
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

    pub fn get(&self, user: &Key, claiming_address: &Key) -> U256 {
        self.dict
            .get_by_keys((user, claiming_address))
            .unwrap_or_default()
    }

    pub fn set(&self, user: &Key, claiming_address: &Key, claimed_amount: U256) {
        self.dict
            .set_by_keys((user, claiming_address), claimed_amount);
    }
}

pub struct ClaimSig {
    dict: Dict,
    length: U256,
}

impl ClaimSig {
    pub fn instance() -> ClaimSig {
        ClaimSig {
            dict: Dict::instance(CLAIM_SIG_DICT),
            length: 0.into(),
        }
    }

    pub fn init() {
        Dict::init(CLAIM_SIG_DICT)
    }

    pub fn get(&self, indx: &U256) -> Bytes {
        self.dict.get(indx.to_string().as_str()).unwrap_or_revert()
    }

    pub fn set(&self, indx: &U256, value: Bytes) {
        self.dict.set(indx.to_string().as_str(), value);
    }

    pub fn push(&mut self, value: U256) {
        self.dict.set(self.length.to_string().as_str(), value);
        self.length = self.length.checked_add(1.into()).unwrap_or_revert();
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

pub fn total_supply() -> U256 {
    get_key(TOTAL_SUPPLY).unwrap_or_default()
}

pub fn set_total_supply(total_supply: U256) {
    set_key(TOTAL_SUPPLY, total_supply);
}
pub fn reward_data() -> U256 {
    get_key(REWARD_DATA).unwrap_or_default()
}

pub fn set_reward_data(reward_data: U256) {
    set_key(REWARD_DATA, reward_data);
}

// #[derive(Clone, Copy, CLTyped, ToBytes, FromBytes, Default)]
// pub struct Point {
//     pub bias: U256,
//     pub slope: U256, // - dweight / dt
// }

// #[derive(Clone, Copy, CLTyped, ToBytes, FromBytes, Default)]
// pub struct VotedSlope {
//     pub slope: U256,
//     pub power: U256,
//     pub end: U256,
// }

// pub struct GaugeTypeNames {
//     dict: Dict,
// }

// impl GaugeTypeNames {
//     pub fn instance() -> GaugeTypeNames {
//         GaugeTypeNames {
//             dict: Dict::instance(GAUGE_TYPE_NAMES_DICT),
//         }
//     }

//     pub fn init() {
//         Dict::init(GAUGE_TYPE_NAMES_DICT)
//     }

//     pub fn get(&self, key: &U128) -> String {
//         self.dict.get(&key.to_string()).unwrap_or_default()
//     }

//     pub fn set(&self, key: &U128, value: String) {
//         self.dict.set(&key.to_string(), value);
//     }
// }

// pub struct GaugeTypes_ {
//     dict: Dict,
// }

// impl GaugeTypes_ {
//     pub fn instance() -> GaugeTypes_ {
//         GaugeTypes_ {
//             dict: Dict::instance(GAUGE_TYPES_DICT),
//         }
//     }

//     pub fn init() {
//         Dict::init(GAUGE_TYPES_DICT)
//     }

//     pub fn get(&self, owner: &Key) -> U128 {
//         self.dict.get(&key_to_str(owner)).unwrap_or_default()
//     }

//     pub fn set(&self, owner: &Key, value: U128) {
//         self.dict.set(&key_to_str(owner), value);
//     }
// }

// pub struct VoteUserSlopes {
//     dict: Dict,
// }

// impl VoteUserSlopes {
//     pub fn instance() -> VoteUserSlopes {
//         VoteUserSlopes {
//             dict: Dict::instance(VOTE_USER_SLOPES_DICT),
//         }
//     }

//     pub fn init() {
//         Dict::init(VOTE_USER_SLOPES_DICT)
//     }

//     pub fn get(&self, owner: &Key, spender: &Key) -> VotedSlope {
//         self.dict.get_by_keys((owner, spender)).unwrap_or_default()
//     }

//     pub fn set(&self, owner: &Key, spender: &Key, value: VotedSlope) {
//         self.dict.set_by_keys((owner, spender), value);
//     }
// }

// pub struct VoteUserPower {
//     dict: Dict,
// }

// impl VoteUserPower {
//     pub fn instance() -> VoteUserPower {
//         VoteUserPower {
//             dict: Dict::instance(VOTE_USER_POWER_DICT),
//         }
//     }

//     pub fn init() {
//         Dict::init(VOTE_USER_POWER_DICT)
//     }

//     pub fn get(&self, owner: &Key) -> U256 {
//         self.dict.get(&key_to_str(owner)).unwrap_or_default()
//     }

//     pub fn set(&self, owner: &Key, value: U256) {
//         self.dict.set(&key_to_str(owner), value);
//     }
// }

// pub struct LastUserVote {
//     dict: Dict,
// }

// impl LastUserVote {
//     pub fn instance() -> LastUserVote {
//         LastUserVote {
//             dict: Dict::instance(LAST_USER_VOTE_DICT),
//         }
//     }

//     pub fn init() {
//         Dict::init(LAST_USER_VOTE_DICT)
//     }

//     pub fn get(&self, owner: &Key, spender: &Key) -> U256 {
//         self.dict.get_by_keys((owner, spender)).unwrap_or_default()
//     }

//     pub fn set(&self, owner: &Key, spender: &Key, value: U256) {
//         self.dict.set_by_keys((owner, spender), value);
//     }
// }

// pub struct PointsWeight {
//     dict: Dict,
// }

// impl PointsWeight {
//     pub fn instance() -> PointsWeight {
//         PointsWeight {
//             dict: Dict::instance(POINTS_WEIGHT_DICT),
//         }
//     }

//     pub fn init() {
//         Dict::init(POINTS_WEIGHT_DICT)
//     }

//     pub fn get(&self, key: &Key, _key: &U256) -> Point {
//         let key_: String = key_and_value_to_str(key, _key);
//         self.dict.get(key_.as_str()).unwrap_or_default()
//     }

//     pub fn set(&self, key: &Key, _key: &U256, value: Point) {
//         let key_: String = key_and_value_to_str(key, _key);
//         self.dict.set(key_.as_str(), value);
//     }
// }

// pub struct ChangesWeight {
//     dict: Dict,
// }

// impl ChangesWeight {
//     pub fn instance() -> ChangesWeight {
//         ChangesWeight {
//             dict: Dict::instance(CHANGES_WEIGHT_DICT),
//         }
//     }

//     pub fn init() {
//         Dict::init(CHANGES_WEIGHT_DICT)
//     }

//     pub fn get(&self, key: &Key, _key: &U256) -> U256 {
//         let key_: String = key_and_value_to_str(key, _key);
//         self.dict.get(key_.as_str()).unwrap_or_default()
//     }

//     pub fn set(&self, key: &Key, _key: &U256, value: U256) {
//         let key_: String = key_and_value_to_str(key, _key);
//         self.dict.set(key_.as_str(), value);
//     }
// }

// pub struct TimeWeight {
//     dict: Dict,
// }

// impl TimeWeight {
//     pub fn instance() -> TimeWeight {
//         TimeWeight {
//             dict: Dict::instance(TIME_WEIGHT_DICT),
//         }
//     }

//     pub fn init() {
//         Dict::init(TIME_WEIGHT_DICT)
//     }

//     pub fn get(&self, owner: &Key) -> U256 {
//         self.dict.get(&key_to_str(owner)).unwrap_or_default()
//     }

//     pub fn set(&self, owner: &Key, value: U256) {
//         self.dict.set(&key_to_str(owner), value);
//     }
// }
// pub struct Gauges {
//     dict: Dict,
//     length: U256,
// }

// impl Gauges {
//     pub fn instance() -> Gauges {
//         Gauges {
//             dict: Dict::instance(GAUGES_DICT),
//             length: 0.into(),
//         }
//     }

//     pub fn init() {
//         Dict::init(GAUGES_DICT)
//     }

//     pub fn get(&self, indx: &U256) -> Key {
//         self.dict.get(indx.to_string().as_str()).unwrap_or_revert()
//     }

//     pub fn set(&self, indx: &U256, value: Key) {
//         self.dict.set(indx.to_string().as_str(), value);
//     }

//     pub fn push(&mut self, value: Key) {
//         self.dict.set(self.length.to_string().as_str(), value);
//         self.length = self.length.checked_add(1.into()).unwrap_or_revert();
//     }
// }

// pub struct TimeSum {
//     dict: Dict,
//     length: U256,
// }

// impl TimeSum {
//     pub fn instance() -> TimeSum {
//         TimeSum {
//             dict: Dict::instance(TIME_SUM_DICT),
//             length: 0.into(),
//         }
//     }

//     pub fn init() {
//         Dict::init(TIME_SUM_DICT)
//     }

//     pub fn get(&self, indx: &U256) -> U256 {
//         self.dict.get(indx.to_string().as_str()).unwrap_or_default()
//     }

//     pub fn set(&self, indx: &U256, value: U256) {
//         self.dict.set(indx.to_string().as_str(), value);
//     }

//     pub fn push(&mut self, value: U256) {
//         self.dict.set(self.length.to_string().as_str(), value);
//         self.length = self.length.checked_add(1.into()).unwrap_or_revert();
//     }
// }

// pub struct PointsSum {
//     dict: Dict,
// }

// impl PointsSum {
//     pub fn instance() -> PointsSum {
//         PointsSum {
//             dict: Dict::instance(POINTS_SUM_DICT),
//         }
//     }

//     pub fn init() {
//         Dict::init(POINTS_SUM_DICT)
//     }

//     pub fn get(&self, key: &U128, _key: &U256) -> Point {
//         let key_: String = values_to_str(&U256::from(key.as_u128()), _key);
//         self.dict.get(key_.as_str()).unwrap_or_default()
//     }

//     pub fn set(&self, key: &U128, _key: &U256, value: Point) {
//         let key_: String = values_to_str(&U256::from(key.as_u128()), _key);
//         self.dict.set(key_.as_str(), value);
//     }
// }

// pub struct ChangeSum {
//     dict: Dict,
// }

// impl ChangeSum {
//     pub fn instance() -> ChangeSum {
//         ChangeSum {
//             dict: Dict::instance(CHANGES_SUM_DICT),
//         }
//     }

//     pub fn init() {
//         Dict::init(CHANGES_SUM_DICT)
//     }

//     pub fn get(&self, key: &U128, _key: &U256) -> U256 {
//         let key_: String = values_to_str(&U256::from(key.as_u128()), _key);
//         self.dict.get(key_.as_str()).unwrap_or_default()
//     }

//     pub fn set(&self, key: &U128, _key: &U256, value: U256) {
//         let key_: String = values_to_str(&U256::from(key.as_u128()), _key);
//         self.dict.set(key_.as_str(), value);
//     }
// }

// pub struct PointsTotal {
//     dict: Dict,
// }

// impl PointsTotal {
//     pub fn instance() -> PointsTotal {
//         PointsTotal {
//             dict: Dict::instance(POINTS_TOTAL_DICT),
//         }
//     }

//     pub fn init() {
//         Dict::init(POINTS_TOTAL_DICT)
//     }

//     pub fn get(&self, key: &U256) -> U256 {
//         self.dict.get(&key.to_string()).unwrap_or_default()
//     }

//     pub fn set(&self, key: &U256, value: U256) {
//         self.dict.set(&key.to_string(), value);
//     }
// }

// pub struct PointsTypeWeight {
//     dict: Dict,
// }

// impl PointsTypeWeight {
//     pub fn instance() -> PointsTypeWeight {
//         PointsTypeWeight {
//             dict: Dict::instance(POINTS_TYPE_WEIGHT_DICT),
//         }
//     }

//     pub fn init() {
//         Dict::init(POINTS_TYPE_WEIGHT_DICT)
//     }

//     pub fn get(&self, key: &U128, _key: &U256) -> U256 {
//         let key_: String = values_to_str(&U256::from(key.as_u128()), _key);
//         self.dict.get(key_.as_str()).unwrap_or_default()
//     }

//     pub fn set(&self, key: &U128, _key: &U256, value: U256) {
//         let key_: String = values_to_str(&U256::from(key.as_u128()), _key);
//         self.dict.set(key_.as_str(), value);
//     }
// }

// pub struct RewardTokens {
//     dict: Dict,
//     length: U256,
// }

// impl RewardTokens {
//     pub fn instance() -> RewardTokens {
//         RewardTokens {
//             dict: Dict::instance(TIME_TYPE_WEIGHT_DICT),
//             length: 0.into(),
//         }
//     }

//     pub fn init() {
//         Dict::init(TIME_TYPE_WEIGHT_DICT)
//     }

//     pub fn get(&self, indx: &U256) -> U256 {
//         self.dict.get(indx.to_string().as_str()).unwrap_or_default()
//     }

//     pub fn set(&self, indx: &U256, value: U256) {
//         self.dict.set(indx.to_string().as_str(), value);
//     }

//     pub fn push(&mut self, value: U256) {
//         self.dict.set(self.length.to_string().as_str(), value);
//         self.length = self.length.checked_add(1.into()).unwrap_or_revert();
//     }
// }

// pub fn zero_address() -> Key {
//     Key::from_formatted_str(
//         "hash-0000000000000000000000000000000000000000000000000000000000000000".into(),
//     )
//     .unwrap()
// }
// pub fn time_total() -> U256 {
//     get_key(TIME_TOTAL).unwrap_or_revert()
// }

// pub fn set_time_total(time_total: U256) {
//     set_key(TIME_TOTAL, time_total);
// }

// pub fn owner() -> Key {
//     get_key(OWNER).unwrap_or_revert()
// }

// pub fn set_owner(owner: Key) {
//     set_key(OWNER, owner);
// }

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

// pub fn token() -> Key {
//     get_key(TOKEN).unwrap_or_revert()
// }

// pub fn set_token(token: Key) {
//     set_key(TOKEN, token);
// }

// pub fn n_gauge_types() -> U128 {
//     get_key(N_GAUGE_TYPES).unwrap_or_revert()
// }

// pub fn set_n_gauge_types(n_gauge_types: U128) {
//     set_key(N_GAUGE_TYPES, n_gauge_types);
// }
// pub fn n_gauges() -> U128 {
//     get_key(N_GAUGES).unwrap_or_revert()
// }

// pub fn set_n_gauges(n_gauges: U128) {
//     set_key(N_GAUGES, n_gauges);
// }

// pub fn voting_escrow() -> Key {
//     get_key(VOTING_ESCROW).unwrap_or_revert()
// }

// pub fn set_voting_escrow(voting_escrow: Key) {
//     set_key(VOTING_ESCROW, voting_escrow);
// }

// pub fn reward_count() -> U256 {
//     get_key(REWARD_COUNT).unwrap_or_default()
// }

// pub fn set_reward_count(reward_count: U256) {
//     set_key(REWARD_COUNT, reward_count);
// }

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
                param.insert(CONTRACT_PACKAGE_HASH, package.to_string());
                param.insert(
                    "event_type",
                    "reward_only_gauge_mint_remove_one".to_string(),
                );
                param.insert("recipient", recipient.to_string());
                param.insert("token_id", token_id.to_string());
                events.push(param);
            }
        }
        REWARDONLYGAUGEEvent::Burn { owner, token_ids } => {
            for token_id in token_ids {
                let mut param = BTreeMap::new();
                param.insert(CONTRACT_PACKAGE_HASH, package.to_string());
                param.insert(
                    "event_type",
                    "reward_only_gauge_burn_remove_one".to_string(),
                );
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
                param.insert(CONTRACT_PACKAGE_HASH, package.to_string());
                param.insert("event_type", "reward_only_gauge_approve_token".to_string());
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
                param.insert(CONTRACT_PACKAGE_HASH, package.to_string());
                param.insert("event_type", "reward_only_gauge_transfer_token".to_string());
                param.insert("sender", sender.to_string());
                param.insert("recipient", recipient.to_string());
                param.insert("token_id", token_id.to_string());
                events.push(param);
            }
        }
        REWARDONLYGAUGEEvent::MetadataUpdate { token_id } => {
            let mut param = BTreeMap::new();
            param.insert(CONTRACT_PACKAGE_HASH, package.to_string());
            param.insert(
                "event_type",
                "reward_only_gauge_metadata_update".to_string(),
            );
            param.insert("token_id", token_id.to_string());
            events.push(param);
        }
    };
    for param in events {
        let _: URef = storage::new_uref(param);
    }
}
