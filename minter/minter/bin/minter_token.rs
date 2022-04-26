#![no_main]
#![no_std]

#[macro_use]
extern crate alloc;
use alloc::vec::Vec;

use alloc::{boxed::Box, collections::BTreeSet, format, string::String};
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    runtime_args, CLType, CLTyped, CLValue, ContractHash, ContractPackageHash, EntryPoint,
    EntryPointAccess, EntryPointType, EntryPoints, Group, Key, Parameter, RuntimeArgs, URef, U256,
};
use contract_utils::{ContractContext, OnChainContractStorage};
use minter_crate::MINTER;

#[derive(Default)]
struct Token(OnChainContractStorage);

impl ContractContext<OnChainContractStorage> for Token {
    fn storage(&self) -> &OnChainContractStorage {
        &self.0
    }
}

impl MINTER<OnChainContractStorage> for Token {}
impl Token {
    fn constructor(
        &mut self,
        token: Key,
        controller: Key,
        contract_hash: ContractHash,
        package_hash: ContractPackageHash,
    ) {
        MINTER::init(
            self,
            token,
            controller,
            Key::from(contract_hash),
            package_hash,
        );
    }
}

#[no_mangle]
fn constructor() {
    let token: Key = runtime::get_named_arg::<Key>("token");
    let controller: Key = runtime::get_named_arg("controller");
    let contract_hash: ContractHash = runtime::get_named_arg("contract_hash");
    let package_hash: ContractPackageHash = runtime::get_named_arg("package_hash");

    Token::default().constructor(token, controller, contract_hash, package_hash);
}

///"""
///@notice Mint everything which belongs to `msg.sender` and send to them
///@param gauge_addr `LiquidityGauge` address to get mintable amount from
///"""

#[no_mangle]
fn mint() {
    let gauge_addr: Key = runtime::get_named_arg("gauge_addr");
    Token::default().mint(gauge_addr);
}

///"""
///@notice Mint everything which belongs to `msg.sender` across multiple gauges
///@param gauge_addrs List of `LiquidityGauge` addresses
///"""

#[no_mangle]
fn mint_many() {
    let _gauge_addrs: Vec<String> = runtime::get_named_arg("gauge_addrs");
    let mut gauge_addrs: Vec<Key> = Vec::new();
    for i in 0..(_gauge_addrs.len()) {
        gauge_addrs.push(Key::from_formatted_str(&_gauge_addrs[i]).unwrap());
    }
    Token::default().mint_many(gauge_addrs);
}

/// """
/// @notice Mint tokens for `_for`
/// @dev Only possible when `msg.sender` has been approved via `toggle_approve_mint`
/// @param gauge_addr `LiquidityGauge` address to get mintable amount from
/// @param _for Address to mint to
/// """

#[no_mangle]
fn mint_for() {
    let gauge_addr: Key = runtime::get_named_arg("gauge_addr");
    let _for: Key = runtime::get_named_arg("for");
    Token::default().mint_for(gauge_addr, _for);
}

/// """
/// @notice allow `minting_user` to mint for `msg.sender`
/// @param minting_user Address to toggle permission for
/// """

#[no_mangle]
fn toggle_approve_mint() {
    let minting_user: Key = runtime::get_named_arg("minting_user");
    Token::default().toggle_approve_mint(minting_user);
}

#[no_mangle]
fn package_hash() {
    let ret: ContractPackageHash = Token::default().get_package_hash();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn call() {
    // Contract name must be same for all new versions of the contracts
    let contract_name: String = runtime::get_named_arg("contract_name");

    // If this is the first deployment
    if !runtime::has_key(&format!("{}_package_hash", contract_name)) {
        // Build new package with initial a first version of the contract.
        let (package_hash, access_token) = storage::create_contract_package_at_hash();
        let (contract_hash, _) =
            storage::add_contract_version(package_hash, get_entry_points(), Default::default());
        // Read arguments for the constructor call.
        let token: Key = runtime::get_named_arg("token");
        let controller: Key = runtime::get_named_arg("controller");

        // Prepare constructor args
        let constructor_args = runtime_args! {
            "token" => token,
            "controller" => controller,
            "contract_hash" => contract_hash,
            "package_hash"=> package_hash

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
    } else {
        // this is a contract upgrade

        let package_hash: ContractPackageHash =
            runtime::get_key(&format!("{}_package_hash", contract_name))
                .unwrap_or_revert()
                .into_hash()
                .unwrap()
                .into();

        let (contract_hash, _): (ContractHash, _) =
            storage::add_contract_version(package_hash, get_entry_points(), Default::default());

        // update contract hash
        runtime::put_key(
            &format!("{}_contract_hash", contract_name),
            contract_hash.into(),
        );
        runtime::put_key(
            &format!("{}_contract_hash_wrapped", contract_name),
            storage::new_uref(contract_hash).into(),
        );
    }
}

fn get_entry_points() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        "constructor",
        vec![
            Parameter::new("token", Key::cl_type()),
            Parameter::new("controller", Key::cl_type()),
            Parameter::new("contract_hash", ContractHash::cl_type()),
            Parameter::new("package_hash", ContractPackageHash::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Groups(vec![Group::new("constructor")]),
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "mint",
        vec![Parameter::new("gauge_addr", Key::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Groups(vec![Group::new("constructor")]),
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "mint_many",
        vec![Parameter::new(
            "gauge_addrs",
            CLType::List(Box::new(String::cl_type())),
        )],
        <()>::cl_type(),
        EntryPointAccess::Groups(vec![Group::new("constructor")]),
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "mint_for",
        vec![
            Parameter::new("gauge_addr", Key::cl_type()),
            Parameter::new("for", Key::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Groups(vec![Group::new("constructor")]),
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "toggle_approve_mint",
        vec![Parameter::new("minting_user", Key::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Groups(vec![Group::new("constructor")]),
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "package_hash",
        vec![],
        ContractPackageHash::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points
}
