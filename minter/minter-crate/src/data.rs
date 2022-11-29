use casper_contract::{contract_api::runtime::get_call_stack, unwrap_or_revert::UnwrapOrRevert};
use casper_types::{system::CallStackElement, ContractPackageHash, Key, U256};
use casperlabs_contract_utils::{get_key, set_key, Dict};
use common::keys::*;

pub struct Minted {
    dict: Dict,
}

impl Minted {
    pub fn instance() -> Minted {
        Minted {
            dict: Dict::instance(MINTED_DICT),
        }
    }

    pub fn init() {
        Dict::init(MINTED_DICT)
    }

    pub fn get(&self, owner: &Key, spender: &Key) -> U256 {
        self.dict.get_by_keys((owner, spender)).unwrap_or_default()
    }

    pub fn set(&self, owner: &Key, spender: &Key, value: U256) {
        self.dict.set_by_keys((owner, spender), value);
    }
}

pub struct AllowedToMintFor {
    dict: Dict,
}

impl AllowedToMintFor {
    pub fn instance() -> AllowedToMintFor {
        AllowedToMintFor {
            dict: Dict::instance(ALLOWED_TO_MINT_FOR_DICT),
        }
    }

    pub fn init() {
        Dict::init(ALLOWED_TO_MINT_FOR_DICT)
    }

    pub fn get(&self, owner: &Key, spender: &Key) -> bool {
        self.dict.get_by_keys((owner, spender)).unwrap_or_default()
    }

    pub fn set(&self, owner: &Key, spender: &Key, value: bool) {
        self.dict.set_by_keys((owner, spender), value);
    }
}

pub fn token() -> Key {
    get_key(TOKEN).unwrap_or_revert()
}

pub fn set_token(token: Key) {
    set_key(TOKEN, token);
}

pub fn controller() -> Key {
    get_key(CONTROLLER).unwrap_or_revert()
}

pub fn set_controller(controller: Key) {
    set_key(CONTROLLER, controller);
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

pub fn set_lock(lock: u64) {
    set_key(LOCK, lock);
}

pub fn get_lock() -> u64 {
    get_key(LOCK).unwrap_or_revert()
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
