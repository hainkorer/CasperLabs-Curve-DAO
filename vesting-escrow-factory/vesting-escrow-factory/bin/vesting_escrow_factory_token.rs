#![no_main]
#![no_std]

#[macro_use]
extern crate alloc;

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
use vesting_escrow_factory_crate::VESTINGESCROWFACTORY;

#[derive(Default)]
struct Token(OnChainContractStorage);

impl ContractContext<OnChainContractStorage> for Token {
    fn storage(&self) -> &OnChainContractStorage {
        &self.0
    }
}

impl VESTINGESCROWFACTORY<OnChainContractStorage> for Token {}
impl Token {
    fn constructor(
        &mut self,
        _target: Key,
        _admin: Key,
        contract_hash: ContractHash,
        package_hash: ContractPackageHash,
    ) {
        VESTINGESCROWFACTORY::init(
            self,
            _target,
            _admin,
            Key::from(contract_hash),
            package_hash,
        );
    }
}
/// """
/// @notice Contract constructor
/// @dev Prior to deployment you must deploy one copy of `VestingEscrowSimple` which
///      is used as a library for vesting contracts deployed by this factory
/// @param _target `VestingEscrowSimple` contract address
/// """

#[no_mangle]
fn constructor() {
    let _target: Key = runtime::get_named_arg::<Key>("_target");
    let _admin: Key = runtime::get_named_arg("_admin");
    let contract_hash: ContractHash = runtime::get_named_arg("contract_hash");
    let package_hash: ContractPackageHash = runtime::get_named_arg("package_hash");

    Token::default().constructor(_target, _admin, contract_hash, package_hash);
}

#[no_mangle]
fn package_hash() {
    let ret: ContractPackageHash = Token::default().get_package_hash();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn admin() {
    let ret: Key = Token::default().admin();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn target() {
    let ret: Key = Token::default().target();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn future_admin() {
    let ret: Key = Token::default().future_admin();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

/// """
/// @notice Accept a pending ownership transfer
/// """

#[no_mangle]
fn apply_transfer_ownership() {
    let ret: bool = Token::default().apply_transfer_ownership();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

/// """
/// @notice Transfer ownership of GaugeController to `addr`
/// @param addr Address to have ownership transferred to
/// """

#[no_mangle]
fn commit_transfer_ownership() {
    let addr: Key = runtime::get_named_arg("addr");
    let ret: bool = Token::default().commit_transfer_ownership(addr);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

/// """
/// @notice Deploy a new vesting contract
/// @dev Each contract holds tokens which vest for a single account. Tokens
///         must be sent to this contract via the regular `ERC20.transfer` method
///         prior to calling this method.
/// @param _token Address of the ERC20 token being distributed
/// @param _recipient Address to vest tokens for
/// @param _amount Amount of tokens being vested for `_recipient`
/// @param _can_disable Can admin disable recipient's ability to claim tokens?
/// @param _vesting_duration Time period over which tokens are released
/// @param _vesting_start Epoch time when tokens begin to vest
/// """

#[no_mangle]
fn deploy_vesting_contract() {
    let _token: Key = runtime::get_named_arg("_token");
    let _recipient: Key = runtime::get_named_arg("_recipient");
    let _amount: U256 = runtime::get_named_arg("_amount");
    let _can_disable: bool = runtime::get_named_arg("_can_disable");
    let _vesting_duration: U256 = runtime::get_named_arg("_vesting_duration");
    let _vesting_start: Option<U256> = runtime::get_named_arg("_vesting_start");
    let _vesting_escrow_simple_contract: Key =
        runtime::get_named_arg("_vesting_escrow_simple_contract");
    let ret: Key = Token::default().deploy_vesting_contract(
        _token,
        _recipient,
        _amount,
        _can_disable,
        _vesting_duration,
        _vesting_start,
        _vesting_escrow_simple_contract,
    );
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
        let _target: Key = runtime::get_named_arg("_target");
        let _admin: Key = runtime::get_named_arg("_admin");

        // Prepare constructor args
        let constructor_args = runtime_args! {
            "_target" => _target,
            "_admin" => _admin,
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
            Parameter::new("_token", Key::cl_type()),
            Parameter::new("_start_time", U256::cl_type()),
            Parameter::new("_end_time", U256::cl_type()),
            Parameter::new("_can_disable", bool::cl_type()),
            Parameter::new("_fund_admins", CLType::List(Box::new(String::cl_type()))),
            Parameter::new("contract_hash", ContractHash::cl_type()),
            Parameter::new("package_hash", ContractPackageHash::cl_type()),
            Parameter::new("lock", u64::cl_type()),
        ],
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
    entry_points.add_entry_point(EntryPoint::new(
        "admin",
        vec![],
        Key::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "target",
        vec![],
        Key::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "future_admin",
        vec![],
        Key::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "commit_transfer_ownership",
        vec![Parameter::new("addr", Key::cl_type())],
        bool::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "apply_transfer_ownership",
        vec![],
        bool::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "deploy_vesting_contract",
        vec![
            Parameter::new("_token", Key::cl_type()),
            Parameter::new("_recipient", Key::cl_type()),
            Parameter::new("_amount", U256::cl_type()),
            Parameter::new("_can_disable", bool::cl_type()),
            Parameter::new("_vesting_duration", U256::cl_type()),
            Parameter::new("_vesting_start", CLType::Option(Box::new(U256::cl_type()))),
            Parameter::new("_vesting_escrow_simple_contract", Key::cl_type()),
        ],
        Key::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));


    // ENTRYPOINTS OF VESTING ESCROW SIMPLE
    entry_points.add_entry_point(EntryPoint::new(
        "constructor",
        vec![
            Parameter::new("token", Key::cl_type()),
            Parameter::new("contract_hash", ContractHash::cl_type()),
            Parameter::new("package_hash", ContractPackageHash::cl_type()),
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
        bool::cl_type(),
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
        "disable_can_disable",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "vested_of",
        vec![Parameter::new("recipient", Key::cl_type())],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "vested_supply",
        vec![],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "locked_supply",
        vec![],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "locked_of",
        vec![Parameter::new("recipient", Key::cl_type())],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "commit_transfer_ownership",
        vec![Parameter::new("addr", Key::cl_type())],
        bool::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "apply_transfer_ownership",
        vec![],
        bool::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "balance_of_vest",
        vec![Parameter::new("recipient", Key::cl_type())],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "claim",
        vec![Parameter::new("addr", Key::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points
}
