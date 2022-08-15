use casper_contract::{contract_api::runtime::get_call_stack, unwrap_or_revert::UnwrapOrRevert};
use casper_types::{system::CallStackElement, ContractPackageHash, Key, U256};
use casperlabs_contract_utils::{get_key, set_key};
use common::{keys::*, utils::*};

pub const MIN_VESTING_DURATION: U256 = U256([86400000 * 365, 0, 0, 0]);

pub fn vesting_escrow_simple_contract() -> Key {
    get_key(VESTING_ESCROW_SIMPLE_CONTRACT).unwrap_or_revert()
}

pub fn set_vesting_escrow_simple_contract(vesting_escrow_simple_contract: Key) {
    set_key(
        VESTING_ESCROW_SIMPLE_CONTRACT,
        vesting_escrow_simple_contract,
    );
}

pub fn admin() -> Key {
    get_key(ADMIN).unwrap_or_else(zero_address)
}

pub fn set_admin(admin: Key) {
    set_key(ADMIN, admin);
}

pub fn target() -> Key {
    get_key(TARGET).unwrap_or_else(zero_address)
}

pub fn set_target(value: Key) {
    set_key(TARGET, value);
}

pub fn future_admin() -> Key {
    get_key(FUTURE_ADMIN).unwrap_or_else(zero_address)
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

pub fn set_vesting_escrow_simple_contract_hash(contract_hash: Key) {
    set_key(SELF_CONTRACT_HASH, contract_hash);
}

pub fn get_vesting_escrow_simple_contract_hash() -> Key {
    get_key(SELF_CONTRACT_HASH).unwrap_or_revert()
}
pub fn set_vesting_escrow_simple_package_hash(package_hash: ContractPackageHash) {
    set_key(VESTING_ESCROW_SIMPLE_CONTRACT_PACKAGE_HASH, package_hash);
}

pub fn get_vesting_escrow_simple_package_hash() -> ContractPackageHash {
    get_key(VESTING_ESCROW_SIMPLE_CONTRACT_PACKAGE_HASH).unwrap_or_revert()
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
