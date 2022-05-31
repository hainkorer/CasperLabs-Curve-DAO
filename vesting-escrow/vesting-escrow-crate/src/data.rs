use crate::event::VESTINGESCROWEvent;
use alloc::{collections::BTreeMap, string::ToString, vec::Vec};
use casper_contract::{
    contract_api::{runtime::get_call_stack, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{system::CallStackElement, ContractPackageHash, Key, URef, U256};
use common::keys::*;
use contract_utils::{get_key, key_to_str, set_key, Dict};

pub struct FundAdmins {
    dict: Dict,
}

impl FundAdmins {
    pub fn instance() -> FundAdmins {
        FundAdmins {
            dict: Dict::instance(FUND_ADMINS_DICT),
        }
    }

    pub fn init() {
        Dict::init(FUND_ADMINS_DICT)
    }

    pub fn get(&self, owner: &Key) -> bool {
        self.dict.get(&key_to_str(owner)).unwrap_or_default()
    }

    pub fn set(&self, owner: &Key, value: bool) {
        self.dict.set(&key_to_str(owner), value);
    }
}

pub struct DisabledAt {
    dict: Dict,
}

impl DisabledAt {
    pub fn instance() -> DisabledAt {
        DisabledAt {
            dict: Dict::instance(DISABLED_AT_DICT),
        }
    }

    pub fn init() {
        Dict::init(DISABLED_AT_DICT)
    }

    pub fn get(&self, owner: &Key) -> U256 {
        self.dict.get(&key_to_str(owner)).unwrap_or_default()
    }

    pub fn set(&self, owner: &Key, value: U256) {
        self.dict.set(&key_to_str(owner), value);
    }
}

pub struct TotalClaimed {
    dict: Dict,
}

impl TotalClaimed {
    pub fn instance() -> TotalClaimed {
        TotalClaimed {
            dict: Dict::instance(TOTAL_CLAIMED_DICT),
        }
    }

    pub fn init() {
        Dict::init(TOTAL_CLAIMED_DICT)
    }

    pub fn get(&self, owner: &Key) -> U256 {
        self.dict.get(&key_to_str(owner)).unwrap_or_default()
    }

    pub fn set(&self, owner: &Key, value: U256) {
        self.dict.set(&key_to_str(owner), value);
    }
}

pub struct InitialLocked {
    dict: Dict,
}

impl InitialLocked {
    pub fn instance() -> InitialLocked {
        InitialLocked {
            dict: Dict::instance(INITIAL_LOCKED_DICT),
        }
    }

    pub fn init() {
        Dict::init(INITIAL_LOCKED_DICT)
    }

    pub fn get(&self, owner: &Key) -> U256 {
        self.dict.get(&key_to_str(owner)).unwrap_or_default()
    }

    pub fn set(&self, owner: &Key, value: U256) {
        self.dict.set(&key_to_str(owner), value);
    }
}

pub fn set_lock(lock: u64) {
    set_key(LOCK, lock);
}

pub fn get_lock() -> u64 {
    get_key(LOCK).unwrap_or_revert()
}

pub fn zero_address() -> Key {
    Key::from_formatted_str(
        "_hash-0000000000000000000000000000000000000000000000000000000000000000".into(),
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

pub fn token() -> Key {
    get_key(TOKEN).unwrap_or_revert()
}

pub fn set_token(value: Key) {
    set_key(TOKEN, value);
}

pub fn future_admin() -> Key {
    get_key(FUTURE_ADMIN).unwrap_or_revert()
}

pub fn set_future_admin(future_admin: Key) {
    set_key(FUTURE_ADMIN, future_admin);
}

pub fn start_time() -> U256 {
    get_key(START_TIME).unwrap_or_revert()
}

pub fn set_start_time(value: U256) {
    set_key(START_TIME, value);
}

pub fn end_time() -> U256 {
    get_key(END_TIME).unwrap_or_revert()
}

pub fn set_end_time(value: U256) {
    set_key(END_TIME, value);
}

pub fn initial_locked_supply() -> U256 {
    get_key(INITIAL_LOCKED_SUPPLY).unwrap_or_revert()
}

pub fn set_initial_locked_supply(value: U256) {
    set_key(INITIAL_LOCKED_SUPPLY, value);
}

pub fn unallocated_supply() -> U256 {
    get_key(UNALLOCATED_SUPPLY).unwrap_or_revert()
}

pub fn set_unallocated_supply(value: U256) {
    set_key(UNALLOCATED_SUPPLY, value);
}

pub fn can_disable() -> bool {
    get_key(CAN_DISABLE).unwrap_or_revert()
}

pub fn set_can_disable(value: bool) {
    set_key(CAN_DISABLE, value);
}

pub fn fund_admins_enabled() -> bool {
    get_key(CAN_DISABLE).unwrap_or_revert()
}

pub fn set_fund_admins_enabled(value: bool) {
    set_key(CAN_DISABLE, value);
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

pub fn emit(event: &VESTINGESCROWEvent) {
    let mut events = Vec::new();
    let package = contract_package_hash();
    match event {
        VESTINGESCROWEvent::Mint {
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
        VESTINGESCROWEvent::Burn { owner, token_ids } => {
            for token_id in token_ids {
                let mut param = BTreeMap::new();
                param.insert(SELF_CONTRACT_PACKAGE_HASH, package.to_string());
                param.insert(EVENT_TYPE, "burn_remove_one".to_string());
                param.insert("owner", owner.to_string());
                param.insert("token_id", token_id.to_string());
                events.push(param);
            }
        }
        VESTINGESCROWEvent::Approve {
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
        VESTINGESCROWEvent::Transfer {
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
        VESTINGESCROWEvent::MetadataUpdate { token_id } => {
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
