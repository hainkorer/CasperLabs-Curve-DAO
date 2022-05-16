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
use casper_types::bytesrepr::Bytes;
use casper_types::{
    runtime_args, CLType, CLTyped, CLValue, ContractHash, ContractPackageHash, EntryPoint,
    EntryPointAccess, EntryPointType, EntryPoints, Group, Key, Parameter, RuntimeArgs, URef, U256,
};
use contract_utils::{ContractContext, OnChainContractStorage};
use vesting_escrow_crate::VESTINGESCROW;

#[derive(Default)]
struct Token(OnChainContractStorage);

impl ContractContext<OnChainContractStorage> for Token {
    fn storage(&self) -> &OnChainContractStorage {
        &self.0
    }
}

impl VESTINGESCROW<OnChainContractStorage> for Token {}
impl Token {
    fn constructor(
        &mut self,
        _token: Key,
        _start_time: U256,
        _end_time: U256,
        _can_disable: bool,
        _fund_admins: Vec<String>,
        contract_hash: ContractHash,
        package_hash: ContractPackageHash,
        lock: u64,
    ) {
        VESTINGESCROW::init(
            self,
            _token,
            _start_time,
            _end_time,
            _can_disable,
            _fund_admins,
            Key::from(contract_hash),
            package_hash,
            lock,
        );
    }
}
/// """
/// @param _token Address of the ERC20 token being distributed
/// @param _start_time Timestamp at which the distribution starts. Should be in
///     the future, so that we have enough time to VoteLock everyone
/// @param _end_time Time until everything should be vested
/// @param _can_disable Whether admin can disable accounts in this deployment
/// @param _fund_admins Temporary admin accounts used only for funding
/// """

#[no_mangle]
fn constructor() {
    let _token: Key = runtime::get_named_arg::<Key>("_token");
    let _start_time: U256 = runtime::get_named_arg("_start_time");
    let _end_time: U256 = runtime::get_named_arg("_end_time");
    let _can_disable: bool = runtime::get_named_arg("_can_disable");
    let _fund_admins: Vec<String> = runtime::get_named_arg("_fund_admins");
    let contract_hash: ContractHash = runtime::get_named_arg("contract_hash");
    let package_hash: ContractPackageHash = runtime::get_named_arg("package_hash");
    let lock: u64 = runtime::get_named_arg("lock");

    Token::default().constructor(
        _token,
        _start_time,
        _end_time,
        _can_disable,
        _fund_admins,
        contract_hash,
        package_hash,
        lock,
    );
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
fn future_admin() {
    let ret: Key = Token::default().future_admin();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn fund_admins() {
    let owner: Key = runtime::get_named_arg("owner");
    let ret: bool = Token::default().fund_admins(owner);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn disabled_at() {
    let owner: Key = runtime::get_named_arg("owner");
    let ret: U256 = Token::default().disabled_at(owner);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn initial_locked() {
    let owner: Key = runtime::get_named_arg("owner");
    let ret: U256 = Token::default().initial_locked(owner);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn total_claimed() {
    let owner: Key = runtime::get_named_arg("owner");
    let ret: U256 = Token::default().total_claimed(owner);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn start_time() {
    let ret: U256 = Token::default().start_time();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn end_time() {
    let ret: U256 = Token::default().end_time();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn token() {
    let ret: Key = Token::default().token();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn fund_admins_enabled() {
    let ret: bool = Token::default().fund_admins_enabled();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn initial_locked_supply() {
    let ret: U256 = Token::default().initial_locked_supply();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn unallocated_supply() {
    let ret: U256 = Token::default().unallocated_supply();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn can_disable() {
    let ret: bool = Token::default().can_disable();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

/// """
/// @notice Accept a pending ownership transfer
/// """

#[no_mangle]
fn accept_transfer_ownership() {
    Token::default().accept_transfer_ownership();
}

/// """
/// @notice Transfer ownership of GaugeController to `addr`
/// @param addr Address to have ownership transferred to
/// """

#[no_mangle]
fn commit_transfer_ownership() {
    let addr: Key = runtime::get_named_arg("addr");
    Token::default().commit_transfer_ownership(addr);
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
        let _token: Key = runtime::get_named_arg("_token");
        let _start_time: U256 = runtime::get_named_arg("_start_time");
        let _end_time: U256 = runtime::get_named_arg("_end_time");
        let _can_disable: bool = runtime::get_named_arg("_can_disable");
        let _fund_admins: Vec<String> = runtime::get_named_arg("_fund_admins");
        let lock: u64 = 0;
        // Prepare constructor args
        let constructor_args = runtime_args! {
            "_token" => _token,
            "_start_time" => _start_time,
            "_end_time" => _end_time,
            "_can_disable" => _can_disable,
            "_fund_admins" => _fund_admins,
            "contract_hash" => contract_hash,
            "package_hash"=> package_hash,
            "lock"=>lock

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
        "future_admin",
        vec![],
        Key::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "initial_locked",
        vec![Parameter::new("owner", Key::cl_type())],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "disabled_at",
        vec![Parameter::new("owner", Key::cl_type())],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "total_claimed",
        vec![Parameter::new("owner", Key::cl_type())],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "fund_admins",
        vec![Parameter::new("owner", Key::cl_type())],
        bool::cl_type(),
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
        "accept_transfer_ownership",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "token",
        vec![],
        Key::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "start_time",
        vec![],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "end_time",
        vec![],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "initial_locked_supply",
        vec![],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "unallocated_supply",
        vec![],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "can_disable",
        vec![],
        bool::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "fund_admins_enabled",
        vec![],
        bool::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points
}
