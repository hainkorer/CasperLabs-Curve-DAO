#![no_main]
#![no_std]
extern crate alloc;
use alloc::{boxed::Box, collections::BTreeSet, format, string::String, vec, vec::Vec};

use casper_contract::{
    contract_api::{account, runtime, storage, system},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    contracts::{ContractHash, ContractPackageHash},
    runtime_args,
    system::handle_payment::RuntimeProvider,
    CLType, CLTyped, CLValue, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, Group,
    Key, Parameter, RuntimeArgs, URef, U256,
};
pub mod mappings;

#[no_mangle]
fn constructor() {
    let contract_hash: ContractHash = runtime::get_named_arg("contract_hash");
    let package_hash: ContractPackageHash = runtime::get_named_arg("package_hash");
    let vesting_escrow_simple: Key = runtime::get_named_arg("vesting_escrow_simple");

    mappings::set_key(&mappings::self_hash_key(), contract_hash);
    mappings::set_key(&mappings::self_package_key(), package_hash);
    mappings::set_key(
        &mappings::vesting_escrow_simple_key(),
        ContractHash::from(vesting_escrow_simple.into_hash().unwrap_or_default()),
    );
}
#[no_mangle]
fn initialize() {
    let vesting_escrow_simple_address: ContractHash =
        mappings::get_key(&mappings::vesting_escrow_simple_key());

    let admin: Key = runtime::get_named_arg("admin");
    let token: Key = runtime::get_named_arg("token");
    let recipient: Key = runtime::get_named_arg("recipient");
    let amount: U256 = runtime::get_named_arg("amount");
    let start_time: U256 = runtime::get_named_arg("start_time");
    let end_time: U256 = runtime::get_named_arg("end_time");
    let can_disable: bool = runtime::get_named_arg("can_disable");
    let ret: bool = runtime::call_contract(
        vesting_escrow_simple_address,
        "initialize",
        runtime_args! {
            "admin"=>admin,
                "token"=>token,
                "recipient"=>recipient,
                "amount"=>amount,
                "start_time"=>start_time,
                "end_time"=>end_time,
                "can_disable"=>can_disable
        },
    );
    mappings::set_key(&mappings::result_key(), ret);
}
#[no_mangle]
fn toggle_disable() {
    let vesting_escrow_simple_address: ContractHash =
        mappings::get_key(&mappings::vesting_escrow_simple_key());
    let recipient: Key = runtime::get_named_arg("recipient");
    let ret: () = runtime::call_contract(
        vesting_escrow_simple_address,
        "toggle_disable",
        runtime_args! {
            "recipient" => recipient
        },
    );
    mappings::set_key(&mappings::result_key(), ret);
}
#[no_mangle]
fn vested_of() {
    let vesting_escrow_simple_address: ContractHash =
        mappings::get_key(&mappings::vesting_escrow_simple_key());
    let recipient: Key = runtime::get_named_arg("recipient");
    let ret: U256 = runtime::call_contract(
        vesting_escrow_simple_address,
        "vested_of",
        runtime_args! {
            "recipient" => recipient
        },
    );
    mappings::set_key(&mappings::result_key(), ret);
}
#[no_mangle]
fn vested_supply() {
    let vesting_escrow_simple_address: ContractHash =
        mappings::get_key(&mappings::vesting_escrow_simple_key());
    let ret: U256 = runtime::call_contract(
        vesting_escrow_simple_address,
        "vested_supply",
        runtime_args! {},
    );
    mappings::set_key(&mappings::result_key(), ret);
}
#[no_mangle]
fn locked_supply() {
    let vesting_escrow_simple_address: ContractHash =
        mappings::get_key(&mappings::vesting_escrow_simple_key());
    let ret: U256 = runtime::call_contract(
        vesting_escrow_simple_address,
        "locked_supply",
        runtime_args! {},
    );
    mappings::set_key(&mappings::result_key(), ret);
}
#[no_mangle]
fn balance_of() {
    let vesting_escrow_simple_address: ContractHash =
        mappings::get_key(&mappings::vesting_escrow_simple_key());
    let recipient: Key = runtime::get_named_arg("recipient");
    let ret: U256 = runtime::call_contract(
        vesting_escrow_simple_address,
        "balance_of",
        runtime_args! {
            "recipient" => recipient
        },
    );
    mappings::set_key(&mappings::result_key(), ret);
}
#[no_mangle]
fn commit_transfer_ownership() {
    let vesting_escrow_simple_address: ContractHash =
        mappings::get_key(&mappings::vesting_escrow_simple_key());
    let addr: Key = runtime::get_named_arg("addr");
    let ret: bool = runtime::call_contract(
        vesting_escrow_simple_address,
        "commit_transfer_ownership",
        runtime_args! {
            "addr" => addr
        },
    );
    mappings::set_key(&mappings::result_key(), ret);
}
#[no_mangle]
fn apply_transfer_ownership() {
    let vesting_escrow_simple_address: ContractHash =
        mappings::get_key(&mappings::vesting_escrow_simple_key());
    let ret: bool = runtime::call_contract(
        vesting_escrow_simple_address,
        "apply_transfer_ownership",
        runtime_args! {},
    );
    mappings::set_key(&mappings::result_key(), ret);
}

#[no_mangle]
fn get_entry_points() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        "constructor",
        vec![
            Parameter::new("package_hash", ContractPackageHash::cl_type()),
            Parameter::new("contract_hash", ContractHash::cl_type()),
            Parameter::new("vesting_escrow_simple", Key::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Groups(vec![Group::new("constructor")]),
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "initialize",
        vec![
            Parameter::new("admin", Key::cl_type()),
            Parameter::new("token", Key::cl_type()),
            Parameter::new("recipient", Key::cl_type()),
            Parameter::new("amount", U256::cl_type()),
            Parameter::new("start_time", U256::cl_type()),
            Parameter::new("end_time", U256::cl_type()),
            Parameter::new("can_disable", bool::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "toggle_disable",
        vec![Parameter::new("recipient", Key::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "vested_of",
        vec![Parameter::new("recipient", Key::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "vested_supply",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "locked_supply",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "balance_of",
        vec![Parameter::new("recipient", Key::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "commit_transfer_ownership",
        vec![Parameter::new("addr", Key::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "apply_transfer_ownership",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points
}

#[no_mangle]
fn call() {
    // Build new package with initial a first version of the contract.
    let (package_hash, access_token) = storage::create_contract_package_at_hash();
    let (contract_hash, _) =
        storage::add_contract_version(package_hash, get_entry_points(), Default::default());
    let vesting_escrow_simple: Key = runtime::get_named_arg("vesting_escrow_simple");

    // Prepare constructor args
    let constructor_args = runtime_args! {
        "contract_hash" => contract_hash,
        "package_hash" => package_hash,
        "vesting_escrow_simple" => vesting_escrow_simple
    };

    // Add the constructor group to the package hash with a single URef.
    let constructor_access: URef =
        storage::create_contract_user_group(package_hash, "constructor", 1, Default::default())
            .unwrap_or_revert()
            .pop()
            .unwrap_or_revert();

    // Call the constructor entry point
    let _: () =
        runtime::call_versioned_contract(package_hash, None, "constructor", constructor_args);

    // Remove all URefs from the constructor group, so no one can call it for the second time.
    let mut urefs = BTreeSet::new();
    urefs.insert(constructor_access);
    storage::remove_contract_user_group_urefs(package_hash, "constructor", urefs)
        .unwrap_or_revert();

    // Store contract in the account's named keys.
    let contract_name: alloc::string::String = runtime::get_named_arg("contract_name");
    runtime::put_key(
        &format!("{}_package_hash", contract_name),
        package_hash.into(),
    );
    runtime::put_key(
        &format!("{}_package_hash_wrapped", contract_name),
        storage::new_uref(package_hash).into(),
    );
    runtime::put_key(
        &format!("{}_contract_hash", contract_name),
        contract_hash.into(),
    );
    runtime::put_key(
        &format!("{}_contract_hash_wrapped", contract_name),
        storage::new_uref(contract_hash).into(),
    );
    runtime::put_key(
        &format!("{}_package_access_token", contract_name),
        access_token.into(),
    );
}
