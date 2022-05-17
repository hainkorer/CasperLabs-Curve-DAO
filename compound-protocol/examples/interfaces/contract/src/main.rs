#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

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
use contract_utils::{set_key, ContractContext, ContractStorage, OnChainContractStorage};
use interest_rate_model_interface;
use interest_rate_model_interface::{constants::*, interface::InterestRateModel};

// contract state struct
#[derive(Default)]
struct IntegrationTests(OnChainContractStorage);

// contract state trait
impl ContractContext<OnChainContractStorage> for IntegrationTests {
    fn storage(&self) -> &OnChainContractStorage {
        &self.0
    }
}

// contract logic traits
pub trait IntegrationTestsTrait<Storage: ContractStorage>: ContractContext<Storage> {
    fn stub(&self) {}
}
impl IntegrationTestsTrait<OnChainContractStorage> for IntegrationTests {}

// interest rate model trait and method implementation
impl interest_rate_model_interface::interface::InterestRateModel for IntegrationTests {
    /// @notice Calculates the current borrow interest rate per block
    /// @param cash The total amount of cash the market has
    /// @param borrows The total amount of borrows the market has outstanding
    /// @param reserves The total amount of reserves the market has
    /// @param package_hash The optional ContractPackageHash of an InterestRateModel contract
    /// @return The borrow rate per block (as a percentage, and scaled by 1e18)
    ///
    fn get_borrow_rate(&self, _cash: U256, _borrows: U256, _reserves: U256) -> U256 {
        let ret = _cash + _borrows + _reserves;
        set_key("result", ret);
        ret
    }

    /// @notice Calculates the current supply interest rate per block
    /// @param cash The total amount of cash the market has
    /// @param borrows The total amount of borrows the market has outstanding
    /// @param reserves The total amount of reserves the market has
    /// @param reserveFactorMantissa The current reserve factor the market has
    /// @return The supply rate per block (as a percentage, and scaled by 1e18)
    ///
    fn get_supply_rate(
        &self,
        _cash: U256,
        _borrows: U256,
        _reserves: U256,
        _reserve_factor_mantissa: U256,
    ) -> U256 {
        let ret = _cash + _borrows + _reserves + _reserve_factor_mantissa;
        set_key("result", ret);
        ret
    }
}

impl IntegrationTests {
    fn constructor(&self, package_hash: ContractPackageHash) {
        set_key("package_hash", package_hash);
        self.initialize();
    }
}

#[no_mangle]
fn constructor() {
    let package_hash: ContractPackageHash = runtime::get_named_arg("package_hash");

    IntegrationTests::default().constructor(package_hash);
}

#[no_mangle]
fn get_borrow_rate() {
    let cash = runtime::get_named_arg(RUNTIME_ARG_CASH);
    let borrows = runtime::get_named_arg(RUNTIME_ARG_BORROWS);
    let reserves = runtime::get_named_arg(RUNTIME_ARG_RESERVES);

    let ret: U256 = IntegrationTests::default().get_borrow_rate(cash, borrows, reserves);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn get_supply_rate() {
    let cash = runtime::get_named_arg(RUNTIME_ARG_CASH);
    let borrows = runtime::get_named_arg(RUNTIME_ARG_BORROWS);
    let reserves = runtime::get_named_arg(RUNTIME_ARG_RESERVES);
    let mantissa = runtime::get_named_arg(RUNTIME_ARG_RESERVE_FACTOR_MANTISSA);
    let ret: U256 = IntegrationTests::default().get_supply_rate(cash, borrows, reserves, mantissa);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

fn get_entry_points() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        "constructor",
        vec![Parameter::new(
            "package_hash",
            ContractPackageHash::cl_type(),
        )],
        <()>::cl_type(),
        EntryPointAccess::Groups(vec![Group::new("constructor")]),
        EntryPointType::Contract,
    ));

    // import entrypoints from crate
    let interest_rate_model_entry_points =
        interest_rate_model_interface::entrypoints::entry_points_install().take_entry_points();
    for ept in interest_rate_model_entry_points {
        entry_points.add_entry_point(ept);
    }
    entry_points
}

#[no_mangle]
fn call() {
    // Store contract in the account's named keys. Contract name must be same for all new versions of the contracts
    let contract_name: alloc::string::String = runtime::get_named_arg("contract_name");

    // If this is the first deployment
    if !runtime::has_key(&format!("{}_package_hash", contract_name)) {
        // Build new package.
        let (package_hash, access_token) = storage::create_contract_package_at_hash();
        // add a first version to this package
        let (contract_hash, _): (ContractHash, _) =
            storage::add_contract_version(package_hash, get_entry_points(), Default::default());

        let constructor_args = runtime_args! {
            "package_hash" => package_hash,
            "contract_hash" => contract_hash,
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
    // If contract package did already exist
    else {
        // get the package
        let package_hash: ContractPackageHash =
            runtime::get_key(&format!("{}_package_hash", contract_name))
                .unwrap_or_revert()
                .into_hash()
                .unwrap()
                .into();
        // create new version and install it
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
