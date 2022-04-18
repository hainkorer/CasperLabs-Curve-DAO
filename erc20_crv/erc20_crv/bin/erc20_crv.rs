#![no_main]
#![no_std]

extern crate alloc;
use alloc::{boxed::Box, collections::BTreeSet, format, string::String, vec, vec::Vec};
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    runtime_args, CLType, CLTyped, CLValue, ContractHash, ContractPackageHash, EntryPoint,
    EntryPointAccess, EntryPointType, EntryPoints, Group, Key, Parameter, RuntimeArgs, URef, U256,
};
use contract_utils::{ContractContext, OnChainContractStorage};
use erc20_crate::{self, ERC20};
use erc20_crv::{self, ERC20CRV};

#[derive(Default)]
struct Erc20Crv(OnChainContractStorage);
impl ContractContext<OnChainContractStorage> for Erc20Crv {
    fn storage(&self) -> &OnChainContractStorage {
        &self.0
    }
}
impl ERC20<OnChainContractStorage> for Erc20Crv {}
impl ERC20CRV<OnChainContractStorage> for Erc20Crv {}

impl Erc20Crv {
    fn constructor(  &mut self,
        name: String,
        symbol: String,
        decimal: u8,
        supply: U256,
        contract_hash: ContractHash,
        package_hash: ContractPackageHash,) {
        ERC20CRV::init(  self,
            name,
            symbol,
            decimal,
            supply,
            Key::from(contract_hash),
            package_hash,);
    }
}

#[no_mangle]
fn constructor() {
    let name: String = runtime::get_named_arg("name");
    let symbol: String = runtime::get_named_arg("symbol");
    let decimal: u8 = runtime::get_named_arg("decimal");
    let supply: U256 = runtime::get_named_arg("supply");
    let contract_hash: ContractHash = runtime::get_named_arg("contract_hash");
    let package_hash: ContractPackageHash = runtime::get_named_arg("package_hash");
    Erc20Crv::default().constructor(name, symbol, decimal, supply, contract_hash, package_hash);
    
}

fn get_entry_points() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        "constructor",
        vec![
            Parameter::new("name", String::cl_type()),
            Parameter::new("symbol", String::cl_type()),
            Parameter::new("decimal", u8::cl_type()),
            Parameter::new("supply", U256::cl_type()),
            Parameter::new("contract_hash", ContractHash::cl_type()),
            Parameter::new("package_hash", ContractPackageHash::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Groups(vec![Group::new("constructor")]),
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
        let name: String = runtime::get_named_arg("name");
        let symbol: String = runtime::get_named_arg("symbol");
        let decimal: u8 = runtime::get_named_arg("decimal");
        let supply: U256 = runtime::get_named_arg("supply");
    let constructor_args = runtime_args! {
        "name" => name,
            "symbol" => symbol,
            "decimal" => decimal,
            "supply" => supply,
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
