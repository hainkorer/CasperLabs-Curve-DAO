#![no_main]
#![no_std]
extern crate alloc;
use alloc::{collections::BTreeSet, format, vec};
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    runtime_args, CLTyped, CLValue, ContractHash, ContractPackageHash, EntryPoint,
    EntryPointAccess, EntryPointType, EntryPoints, Group, Key, Parameter, RuntimeArgs, URef, U256,
};

use contract_utils::{ContractContext, OnChainContractStorage};
use liquidity_gauge_v3_crate::{self, LIQUIDITYTGAUGEV3};

#[derive(Default)]
struct LiquidityGaugeV3(OnChainContractStorage);

impl ContractContext<OnChainContractStorage> for LiquidityGaugeV3 {
    fn storage(&self) -> &OnChainContractStorage {
        &self.0
    }
}

impl LIQUIDITYTGAUGEV3<OnChainContractStorage> for LiquidityGaugeV3 {}

impl LiquidityGaugeV3 {
    fn constructor(
        &mut self,
        lp_addr: Key,
        minter: Key,
        admin: Key,
        contract_hash: ContractHash,
        package_hash: ContractPackageHash,
    ) {
        LIQUIDITYTGAUGEV3::init(self, lp_addr, minter, admin, contract_hash, package_hash);
    }
}

#[no_mangle]
fn constructor() {
    let lp_addr: Key = runtime::get_named_arg("lp_addr");
    let minter: Key = runtime::get_named_arg("minter");
    let admin: Key = runtime::get_named_arg("admin");
    let contract_hash: ContractHash = runtime::get_named_arg("contract_hash");
    let package_hash: ContractPackageHash = runtime::get_named_arg("package_hash");
    LiquidityGaugeV3::default().constructor(lp_addr, minter, admin, contract_hash, package_hash);
}

fn get_entry_points() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        "constructor",
        vec![
            Parameter::new("lp_addr", Key::cl_type()),
            Parameter::new("minter", Key::cl_type()),
            Parameter::new("admin", Key::cl_type()),
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
    let lp_addr: Key = runtime::get_named_arg("lp_addr");
    let minter: Key = runtime::get_named_arg("minter");
    let admin: Key = runtime::get_named_arg("admin");

    // Build new package with initial a first version of the contract.
    let (package_hash, access_token) = storage::create_contract_package_at_hash();
    let (contract_hash, _) =
        storage::add_contract_version(package_hash, get_entry_points(), Default::default());

    // Add the constructor group to the package hash with a single URef.
    let constructor_access: URef =
        storage::create_contract_user_group(package_hash, "constructor", 1, Default::default())
            .unwrap_or_revert()
            .pop()
            .unwrap_or_revert();

    // Call the constructor entry point
    let _: () = runtime::call_versioned_contract(
        package_hash,
        None,
        "constructor",
        runtime_args! {
            "lp_addr" => lp_addr,
            "minter" => minter,
            "admin" => admin,
            "contract_hash" => contract_hash,
            "package_hash" => package_hash,
        },
    );

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
