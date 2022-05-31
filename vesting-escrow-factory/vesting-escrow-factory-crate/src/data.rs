use crate::event::VESTINGESCROWFACTORYEvent;
use alloc::{collections::BTreeMap, string::ToString, vec::Vec};
use casper_contract::{
    contract_api::{runtime::get_call_stack, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{system::CallStackElement, ContractPackageHash, Key, URef, U256};
use common::keys::*;
use contract_utils::{get_key, set_key};

pub const VESTING_ESCROW_FACTORY_MIN_VESTING_DURATION: U256 = U256([56400 * 360, 0, 0, 0]);

pub fn vesting_escrow_simple_contract() -> Key {
    get_key(VESTING_ESCROW_FACTORY_VESTING_ESCROW_SIMPLE_CONTRACT).unwrap_or_revert()
}

pub fn set_vesting_escrow_simple_contract(vesting_escrow_simple_contract: Key) {
    set_key(
        VESTING_ESCROW_FACTORY_VESTING_ESCROW_SIMPLE_CONTRACT,
        vesting_escrow_simple_contract,
    );
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
    get_key(VESTING_ESCROW_FACTORY_ADMIN).unwrap_or_revert()
}

pub fn set_admin(admin: Key) {
    set_key(VESTING_ESCROW_FACTORY_ADMIN, admin);
}

pub fn target() -> Key {
    get_key(VESTING_ESCROW_FACTORY_TARGET).unwrap_or_revert()
}

pub fn set_target(value: Key) {
    set_key(VESTING_ESCROW_FACTORY_TARGET, value);
}

pub fn future_admin() -> Key {
    get_key(VESTING_ESCROW_FACTORY_FUTURE_ADMIN).unwrap_or_revert()
}

pub fn set_future_admin(future_admin: Key) {
    set_key(VESTING_ESCROW_FACTORY_FUTURE_ADMIN, future_admin);
}

pub fn set_hash(contract_hash: Key) {
    set_key(VESTING_ESCROW_FACTORY_SELF_CONTRACT_HASH, contract_hash);
}

pub fn get_hash() -> Key {
    get_key(VESTING_ESCROW_FACTORY_SELF_CONTRACT_HASH).unwrap_or_revert()
}
pub fn set_package_hash(package_hash: ContractPackageHash) {
    set_key(VESTING_ESCROW_FACTORY_CONTRACT_PACKAGE_HASH, package_hash);
}

pub fn get_package_hash() -> ContractPackageHash {
    get_key(VESTING_ESCROW_FACTORY_CONTRACT_PACKAGE_HASH).unwrap_or_revert()
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

pub fn emit(event: &VESTINGESCROWFACTORYEvent) {
    let mut events = Vec::new();
    let package = contract_package_hash();
    match event {
        VESTINGESCROWFACTORYEvent::Mint {
            recipient,
            token_ids,
        } => {
            for token_id in token_ids {
                let mut param = BTreeMap::new();
                param.insert(
                    VESTING_ESCROW_FACTORY_CONTRACT_PACKAGE_HASH,
                    package.to_string(),
                );
                param.insert(
                    "event_type",
                    "vesting_escrow_factory_mint_remove_one".to_string(),
                );
                param.insert("recipient", recipient.to_string());
                param.insert("token_id", token_id.to_string());
                events.push(param);
            }
        }
        VESTINGESCROWFACTORYEvent::Burn { owner, token_ids } => {
            for token_id in token_ids {
                let mut param = BTreeMap::new();
                param.insert(
                    VESTING_ESCROW_FACTORY_CONTRACT_PACKAGE_HASH,
                    package.to_string(),
                );
                param.insert(
                    "event_type",
                    "vesting_escrow_factory_burn_remove_one".to_string(),
                );
                param.insert("owner", owner.to_string());
                param.insert("token_id", token_id.to_string());
                events.push(param);
            }
        }
        VESTINGESCROWFACTORYEvent::Approve {
            owner,
            spender,
            token_ids,
        } => {
            for token_id in token_ids {
                let mut param = BTreeMap::new();
                param.insert(
                    VESTING_ESCROW_FACTORY_CONTRACT_PACKAGE_HASH,
                    package.to_string(),
                );
                param.insert(
                    "event_type",
                    "vesting_escrow_factory_approve_token".to_string(),
                );
                param.insert("owner", owner.to_string());
                param.insert("spender", spender.to_string());
                param.insert("token_id", token_id.to_string());
                events.push(param);
            }
        }
        VESTINGESCROWFACTORYEvent::Transfer {
            sender,
            recipient,
            token_ids,
        } => {
            for token_id in token_ids {
                let mut param = BTreeMap::new();
                param.insert(
                    VESTING_ESCROW_FACTORY_CONTRACT_PACKAGE_HASH,
                    package.to_string(),
                );
                param.insert(
                    "event_type",
                    "vesting_escrow_factory_transfer_token".to_string(),
                );
                param.insert("sender", sender.to_string());
                param.insert("recipient", recipient.to_string());
                param.insert("token_id", token_id.to_string());
                events.push(param);
            }
        }
        VESTINGESCROWFACTORYEvent::MetadataUpdate { token_id } => {
            let mut param = BTreeMap::new();
            param.insert(
                VESTING_ESCROW_FACTORY_CONTRACT_PACKAGE_HASH,
                package.to_string(),
            );
            param.insert(
                "event_type",
                "vesting_escrow_factory_metadata_update".to_string(),
            );
            param.insert("token_id", token_id.to_string());
            events.push(param);
        }
    };
    for param in events {
        let _: URef = storage::new_uref(param);
    }
}
