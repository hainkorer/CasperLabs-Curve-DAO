#![no_main]
#![no_std]

extern crate alloc;
use alloc::{collections::BTreeSet, format, vec};

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    contracts::{ContractHash, ContractPackageHash},
    runtime_args, CLTyped, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, Group, Key,
    Parameter, RuntimeArgs, URef,
};
pub mod mappings;

#[no_mangle]
fn constructor() {
    let contract_hash: ContractHash = runtime::get_named_arg("contract_hash");
    let package_hash: ContractPackageHash = runtime::get_named_arg("package_hash");
    let ownable: Key = runtime::get_named_arg("ownable");

    mappings::set_key(&mappings::self_hash_key(), contract_hash);
    mappings::set_key(&mappings::self_package_key(), package_hash);
    mappings::set_key(
        &mappings::ownable_key(),
        ContractPackageHash::from(ownable.into_hash().unwrap_or_default()),
    );
}

#[no_mangle]
fn owner() {
    let ownable_address: ContractPackageHash = mappings::get_key(&mappings::ownable_key());
    let ret: Key =
        runtime::call_versioned_contract(ownable_address, None, "owner", runtime_args! {});
    mappings::set_key(&mappings::result_key(), ret);
}
#[no_mangle]
fn is_owner() {
    let ownable_address: ContractPackageHash = mappings::get_key(&mappings::ownable_key());
    let ret: bool =
        runtime::call_versioned_contract(ownable_address, None, "is_owner", runtime_args! {});
    mappings::set_key(&mappings::result_key(), ret);
}
//Entry Points
fn get_entry_points() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        "constructor",
        vec![
            Parameter::new("package_hash", ContractPackageHash::cl_type()),
            Parameter::new("contract_hash", ContractHash::cl_type()),
            Parameter::new("ownable", Key::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Groups(vec![Group::new("constructor")]),
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "owner",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "is_owner",
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
    let ownable: Key = runtime::get_named_arg("ownable");

    // Prepare constructor args
    let constructor_args = runtime_args! {
        "contract_hash" => contract_hash,
        "package_hash" => package_hash,
        "ownable" => ownable
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
