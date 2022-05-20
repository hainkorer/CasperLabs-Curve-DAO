#![no_main]
#![no_std]
extern crate alloc;
use alloc::{boxed::Box, collections::BTreeSet, format, string::String, vec, vec::Vec};

use casper_contract::{
    contract_api::{account, runtime::{self, get_named_arg}, storage, system},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    contracts::{ContractPackageHash},
    runtime_args,
    system::handle_payment::RuntimeProvider,
    CLType, CLTyped, CLValue, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, Group,
    Key, Parameter, RuntimeArgs, URef, U256,
};
pub mod mappings;

#[no_mangle]
fn constructor() {
    let contract_hash: ContractPackageHash = runtime::get_named_arg("contract_hash");
    let package_hash: ContractPackageHash = runtime::get_named_arg("package_hash");
    let erc20_crv: Key = runtime::get_named_arg("erc20_crv");
    mappings::set_key(&mappings::self_hash_key(), contract_hash);
    mappings::set_key(&mappings::self_package_key(), package_hash);
    mappings::set_key(
        &mappings::erc20_crv_key(),
        ContractPackageHash::from(erc20_crv.into_hash().unwrap_or_default()),
    );
}
#[no_mangle]
fn start_epoch_time_write() {
    let erc20_crv_address: ContractPackageHash = mappings::get_key(&mappings::erc20_crv_key());
    let ret: U256 = runtime::call_versioned_contract(
        erc20_crv_address,
        None,
        "start_epoch_time_write",
        runtime_args! {},
    );
    mappings::set_key(&mappings::result_key(), ret);
}
#[no_mangle]
fn future_epoch_time_write() {
    let erc20_crv_address: ContractPackageHash = mappings::get_key(&mappings::erc20_crv_key());
    let ret: U256 = runtime::call_versioned_contract(
        erc20_crv_address,
        None,
        "future_epoch_time_write",
        runtime_args! {},
    );
    mappings::set_key(&mappings::result_key(), ret);
}

#[no_mangle]
fn available_supply() {
    let erc20_crv_address: ContractPackageHash = mappings::get_key(&mappings::erc20_crv_key());
    let ret: U256 = runtime::call_versioned_contract(
        erc20_crv_address,
        None,
        "available_supply",
        runtime_args! {},
    );
    mappings::set_key(&mappings::result_key(), ret);
}
#[no_mangle]
fn mintable_in_timeframe() {
    let erc20_crv_address: ContractPackageHash = mappings::get_key(&mappings::erc20_crv_key());
    let start:U256=get_named_arg("start");
    let end:U256=get_named_arg("end");
    let ret: U256 = runtime::call_versioned_contract(
        erc20_crv_address,
        None,
        "mintable_in_timeframe",
        runtime_args! {
            "start"=>start,
            "end"=>end
        },
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
            Parameter::new("contract_hash", ContractPackageHash::cl_type()),
            Parameter::new("erc20_crv", Key::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Groups(vec![Group::new("constructor")]),
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "start_epoch_time_write",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "future_epoch_time_write",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "available_supply",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "mintable_in_timeframe",
        vec![
            Parameter::new("start", U256::cl_type()),
            Parameter::new("end", U256::cl_type())
        ],
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
    let erc20_crv: Key = runtime::get_named_arg("erc20_crv");

    // Prepare constructor args
    let constructor_args = runtime_args! {
        "contract_hash" => contract_hash,
        "package_hash" => package_hash,
        "erc20_crv" => erc20_crv
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
