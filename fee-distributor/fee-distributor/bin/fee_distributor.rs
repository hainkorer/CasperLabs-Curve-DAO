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
use casperlabs_contract_utils::{ContractContext, OnChainContractStorage};
use fee_distributor_crate::{self, data::*, FEEDISTRIBUTOR};

#[derive(Default)]
struct FeeDistributor(OnChainContractStorage);
impl ContractContext<OnChainContractStorage> for FeeDistributor {
    fn storage(&self) -> &OnChainContractStorage {
        &self.0
    }
}

impl FEEDISTRIBUTOR<OnChainContractStorage> for FeeDistributor {}
#[allow(clippy::too_many_arguments)]
impl FeeDistributor {
    fn constructor(
        &mut self,
        voting_escrow: Key,
        start_time: U256,
        token: Key,
        admin: Key,
        emergency_return: Key,
        contract_hash: ContractHash,
        package_hash: ContractPackageHash,
    ) {
        FEEDISTRIBUTOR::init(
            self,
            voting_escrow,
            start_time,
            token,
            admin,
            emergency_return,
            contract_hash,
            package_hash,
        );
    }
}

#[no_mangle]
fn constructor() {
    let voting_escrow: Key = runtime::get_named_arg("voting_escrow");
    let start_time: U256 = runtime::get_named_arg("start_time");
    let token: Key = runtime::get_named_arg("token");
    let admin: Key = runtime::get_named_arg("admin");
    let emergency_return: Key = runtime::get_named_arg("emergency_return");
    let package_hash: ContractPackageHash = runtime::get_named_arg("package_hash");
    let contract_hash: ContractHash = runtime::get_named_arg("contract_hash");
    FeeDistributor::default().constructor(
        voting_escrow,
        start_time,
        token,
        admin,
        emergency_return,
        contract_hash,
        package_hash,
    );
}

#[no_mangle]
fn checkpoint_token() {
    FeeDistributor::default().checkpoint_token();
}

#[no_mangle]
fn ve_for_at() {
    let user: Key = runtime::get_named_arg("user");
    let timestamp: U256 = runtime::get_named_arg("timestamp");
    let ret: U256 = FeeDistributor::default().ve_for_at(user, timestamp);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn checkpoint_total_supply() {
    FeeDistributor::default().checkpoint_total_supply();
}

#[no_mangle]
fn claim() {
    let addr: Option<Key> = runtime::get_named_arg("addr");
    let ret: U256 = FeeDistributor::default().claim(addr);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn claim_many() {
    let _receivers: Vec<String> = runtime::get_named_arg("receivers");
    let mut receivers: Vec<Key> = Vec::new();
    for receiver in &_receivers {
        receivers.push(Key::from_formatted_str(receiver).unwrap());
    }
    let ret: bool = FeeDistributor::default().claim_many(receivers);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn burn() {
    let coin: Key = runtime::get_named_arg("coin");
    let ret: bool = FeeDistributor::default().burn(coin);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn commit_admin() {
    let addr: Key = runtime::get_named_arg("addr");
    FeeDistributor::default().commit_admin(addr);
}

#[no_mangle]
fn apply_admin() {
    FeeDistributor::default().apply_admin();
}

#[no_mangle]
fn toggle_allow_checkpoint_token() {
    FeeDistributor::default().toggle_allow_checkpoint_token();
}

#[no_mangle]
fn kill_me() {
    FeeDistributor::default().kill_me();
}

#[no_mangle]
fn recover_balance() {
    let coin: Key = runtime::get_named_arg("coin");
    let ret: bool = FeeDistributor::default().recover_balance(coin);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

// Variables

#[no_mangle]
fn start_time() {
    runtime::ret(CLValue::from_t(get_start_time()).unwrap_or_revert());
}

#[no_mangle]
fn time_cursor() {
    runtime::ret(CLValue::from_t(get_time_cursor()).unwrap_or_revert());
}

#[no_mangle]
fn time_cursor_of() {
    let addr: Key = runtime::get_named_arg("addr");
    runtime::ret(CLValue::from_t(TimeCursorOf::instance().get(&addr)).unwrap_or_revert());
}

#[no_mangle]
fn user_epoch_of() {
    let addr: Key = runtime::get_named_arg("addr");
    runtime::ret(CLValue::from_t(UserEpochOf::instance().get(&addr)).unwrap_or_revert());
}

#[no_mangle]
fn last_token_time() {
    runtime::ret(CLValue::from_t(get_last_token_time()).unwrap_or_revert());
}

#[no_mangle]
fn tokens_per_week() {
    let week: U256 = runtime::get_named_arg("week");
    runtime::ret(CLValue::from_t(TokensPerWeek::instance().get(&week)).unwrap_or_revert());
}

#[no_mangle]
fn voting_escrow() {
    runtime::ret(CLValue::from_t(get_voting_escrow()).unwrap_or_revert());
}

#[no_mangle]
fn token() {
    runtime::ret(CLValue::from_t(get_token()).unwrap_or_revert());
}

#[no_mangle]
fn total_received() {
    runtime::ret(CLValue::from_t(get_total_received()).unwrap_or_revert());
}

#[no_mangle]
fn token_last_balance() {
    runtime::ret(CLValue::from_t(get_token_last_balance()).unwrap_or_revert());
}

#[no_mangle]
fn ve_supply() {
    let week: U256 = runtime::get_named_arg("week");
    runtime::ret(CLValue::from_t(VeSupply::instance().get(&week)).unwrap_or_revert());
}

#[no_mangle]
fn admin() {
    runtime::ret(CLValue::from_t(get_admin()).unwrap_or_revert());
}

#[no_mangle]
fn future_admin() {
    runtime::ret(CLValue::from_t(get_future_admin()).unwrap_or_revert());
}

#[no_mangle]
fn can_checkpoint_token() {
    runtime::ret(CLValue::from_t(get_can_checkpoint_token()).unwrap_or_revert());
}

#[no_mangle]
fn emergency_return() {
    runtime::ret(CLValue::from_t(get_emergency_return()).unwrap_or_revert());
}

#[no_mangle]
fn is_killed() {
    runtime::ret(CLValue::from_t(get_is_killed()).unwrap_or_revert());
}

fn get_entry_points() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        "constructor",
        vec![
            Parameter::new("voting_escrow", Key::cl_type()),
            Parameter::new("start_time", U256::cl_type()),
            Parameter::new("token", Key::cl_type()),
            Parameter::new("admin", Key::cl_type()),
            Parameter::new("emergency_return", Key::cl_type()),
            Parameter::new("package_hash", ContractPackageHash::cl_type()),
            Parameter::new("contract_hash", ContractHash::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Groups(vec![Group::new("constructor")]),
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "checkpoint_token",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "ve_for_at",
        vec![
            Parameter::new("user", Key::cl_type()),
            Parameter::new("timestamp", U256::cl_type()),
        ],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "checkpoint_total_supply",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "claim",
        vec![Parameter::new("addr", CLType::Option(Box::new(CLType::Key)))],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "claim_many",
        vec![Parameter::new(
            "receivers",
            CLType::List(Box::new(String::cl_type())),
        )],
        bool::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "burn",
        vec![Parameter::new("coin", Key::cl_type())],
        bool::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "commit_admin",
        vec![Parameter::new("addr", Key::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "apply_admin",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "toggle_allow_checkpoint_token",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "kill_me",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "recover_balance",
        vec![Parameter::new("coin", Key::cl_type())],
        bool::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    // Variables
    entry_points.add_entry_point(EntryPoint::new(
        "start_time",
        vec![],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "time_cursor",
        vec![],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "time_cursor_of",
        vec![Parameter::new("addr", Key::cl_type())],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "user_epoch_of",
        vec![Parameter::new("addr", Key::cl_type())],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "last_token_time",
        vec![],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "tokens_per_week",
        vec![Parameter::new("week", U256::cl_type())],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "voting_escrow",
        vec![],
        Key::cl_type(),
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
        "total_received",
        vec![],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "token_last_balance",
        vec![],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "ve_supply",
        vec![Parameter::new("week", U256::cl_type())],
        U256::cl_type(),
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
        "can_checkpoint_token",
        vec![],
        bool::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "emergency_return",
        vec![],
        Key::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "is_killed",
        vec![],
        bool::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
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

        let voting_escrow: Key = runtime::get_named_arg("voting_escrow");
        let start_time: U256 = runtime::get_named_arg("start_time");
        let token: Key = runtime::get_named_arg("token");
        let admin: Key = runtime::get_named_arg("admin");
        let emergency_return: Key = runtime::get_named_arg("emergency_return");
        let constructor_args = runtime_args! {
            "voting_escrow" => voting_escrow,
            "start_time" => start_time,
            "token" => token,
            "admin" => admin,
            "emergency_return" => emergency_return,
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
