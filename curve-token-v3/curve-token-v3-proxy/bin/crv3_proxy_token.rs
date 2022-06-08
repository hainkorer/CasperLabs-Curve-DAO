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
    let curve_token_v3: Key = runtime::get_named_arg("curve_token_v3");

    mappings::set_key(&mappings::self_hash_key(), contract_hash);
    mappings::set_key(&mappings::self_package_key(), package_hash);
    mappings::set_key(
        &mappings::curv_token_v3_key(),
        ContractHash::from(curve_token_v3.into_hash().unwrap_or_default()),
    );
}
#[no_mangle]
fn transfer() {
    let curve_token_v3_address: ContractHash = mappings::get_key(&mappings::curv_token_v3_key());

    let recipient: Key = runtime::get_named_arg("recipient");
    let amount: U256 = runtime::get_named_arg("amount");

    let args: RuntimeArgs = runtime_args! {
        "recipient" => recipient,
        "amount" => amount,
    };

    let ret: Result<(), u32> = runtime::call_contract(curve_token_v3_address, "transfer", args);
    mappings::set_key(&mappings::result_key(), ret);
}
#[no_mangle]
fn transfer_from() {
    let curve_token_v3_address: ContractHash = mappings::get_key(&mappings::curv_token_v3_key());

    let owner: Key = runtime::get_named_arg("owner");
    let recipient: Key = runtime::get_named_arg("recipient");
    let amount: U256 = runtime::get_named_arg("amount");

    let args: RuntimeArgs = runtime_args! {
        "owner" => owner,
        "recipient" => recipient,
        "amount" => amount,
    };

    let ret: Result<(), u32> =
        runtime::call_contract(curve_token_v3_address, "transfer_from", args);
    mappings::set_key(&mappings::result_key(), ret);
}
#[no_mangle]
fn get_total_supply_crv3() {
    let curve_token_v3_address: ContractHash = mappings::get_key(&mappings::curv_token_v3_key());

    let ret: U256 = runtime::call_contract(
        curve_token_v3_address,
        "get_total_supply_crv3",
        runtime_args! {},
    );
    mappings::set_key(&mappings::result_key(), ret);
}
#[no_mangle]
fn mint() {
    let curve_token_v3_address: ContractHash = mappings::get_key(&mappings::curv_token_v3_key());
    let _to: Key = runtime::get_named_arg("_to");
    let _value: U256 = runtime::get_named_arg("_value");
    let ret: bool = runtime::call_contract(
        curve_token_v3_address,
        "mint",
        runtime_args! {
            "_to" => _to,
            "_value" => _value,
        },
    );
    mappings::set_key(&mappings::result_key(), ret);
}
#[no_mangle]
fn decimals() {
    let curve_token_v3_address: ContractHash = mappings::get_key(&mappings::curv_token_v3_key());

    let ret: U256 = runtime::call_contract(curve_token_v3_address, "decimals", runtime_args! {});
    mappings::set_key(&mappings::result_key(), ret);
}

#[no_mangle]
fn approve() {
    let curve_token_v3_address: ContractHash = mappings::get_key(&mappings::curv_token_v3_key());

    let spender: Key = runtime::get_named_arg("spender");
    let amount: U256 = runtime::get_named_arg("amount");

    let args: RuntimeArgs = runtime_args! {
        "spender" => spender,
        "amount" => amount,
    };

    let ret: () = runtime::call_contract(curve_token_v3_address, "approve", args);
    mappings::set_key(&mappings::result_key(), ret);
}
#[no_mangle]
fn increase_allowance() {
    let curve_token_v3_address: ContractHash = mappings::get_key(&mappings::curv_token_v3_key());

    let spender: Key = runtime::get_named_arg("spender");
    let amount: U256 = runtime::get_named_arg("amount");
    let args: RuntimeArgs = runtime_args! {
        "spender" => spender,
        "amount" => amount,
    };

    let ret: Result<(), u32> =
        runtime::call_contract(curve_token_v3_address, "increase_allowance", args);
    mappings::set_key(&mappings::result_key(), ret);
}
#[no_mangle]
fn decrease_allowance() {
    let curve_token_v3_address: ContractHash = mappings::get_key(&mappings::curv_token_v3_key());

    let spender: Key = runtime::get_named_arg("spender");
    let amount: U256 = runtime::get_named_arg("amount");

    let args: RuntimeArgs = runtime_args! {
        "spender" => spender,
        "amount" => amount,
    };

    let ret: Result<(), u32> =
        runtime::call_contract(curve_token_v3_address, "decrease_allowance", args);
    mappings::set_key(&mappings::result_key(), ret);
}
#[no_mangle]
fn burn_from() {
    let curve_token_v3_address: ContractHash = mappings::get_key(&mappings::curv_token_v3_key());

    let _to: Key = runtime::get_named_arg("_to");
    let _value: U256 = runtime::get_named_arg("_value");

    let args: RuntimeArgs = runtime_args! {
        "_to" => _to,
        "_value" => _value,
    };
    let ret: bool = runtime::call_contract(curve_token_v3_address, "burn_from", args);
    mappings::set_key(&mappings::result_key(), ret);
}

fn get_entry_points() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        "constructor",
        vec![
            Parameter::new("contract_hash", ContractHash::cl_type()),
            Parameter::new("package_hash", ContractPackageHash::cl_type()),
            Parameter::new("curve_token_v3", Key::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Groups(vec![Group::new("constructor")]),
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "decimals",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "get_total_supply_crv3",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "transfer",
        vec![
            Parameter::new("recipient", Key::cl_type()),
            Parameter::new("amount", U256::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "transfer_from",
        vec![
            Parameter::new("owner", Key::cl_type()),
            Parameter::new("recipient", Key::cl_type()),
            Parameter::new("amount", U256::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "approve",
        vec![
            Parameter::new("spender", Key::cl_type()),
            Parameter::new("amount", U256::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "increase_allowance",
        vec![
            Parameter::new("spender", Key::cl_type()),
            Parameter::new("amount", U256::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "decrease_allowance",
        vec![
            Parameter::new("spender", Key::cl_type()),
            Parameter::new("amount", U256::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "mint",
        vec![
            Parameter::new("_to", Key::cl_type()),
            Parameter::new("_value", U256::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "burn_from",
        vec![
            Parameter::new("_to", Key::cl_type()),
            Parameter::new("_value", U256::cl_type()),
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
    let curve_token_v3: Key = runtime::get_named_arg("curve_token_v3");

    // Prepare constructor args
    let constructor_args = runtime_args! {
        "contract_hash" => contract_hash,
        "package_hash" => package_hash,
        "curve_token_v3" => curve_token_v3
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
