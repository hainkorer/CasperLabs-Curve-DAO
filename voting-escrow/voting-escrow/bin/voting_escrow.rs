#![no_main]
#![no_std]
extern crate alloc;
use alloc::{boxed::Box, collections::BTreeSet, format, string::String, vec};
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    runtime_args, CLType, CLTyped, CLValue, ContractHash, ContractPackageHash, EntryPoint,
    EntryPointAccess, EntryPointType, EntryPoints, Group, Key, Parameter, RuntimeArgs, URef, U128,
    U256,
};
use casperlabs_contract_utils::{ContractContext, OnChainContractStorage};
use voting_escrow_crate::{self, data, VOTINGESCROW};

#[derive(Default)]
struct VotingEscrow(OnChainContractStorage);
impl ContractContext<OnChainContractStorage> for VotingEscrow {
    fn storage(&self) -> &OnChainContractStorage {
        &self.0
    }
}

impl VOTINGESCROW<OnChainContractStorage> for VotingEscrow {}
impl VotingEscrow {
    fn constructor(
        &mut self,
        token_addr: Key,
        name: String,
        symbol: String,
        version: String,
        contract_hash: ContractHash,
        package_hash: ContractPackageHash,
    ) {
        VOTINGESCROW::init(
            self,
            token_addr,
            name,
            symbol,
            version,
            contract_hash,
            package_hash,
        );
    }
}

#[no_mangle]
fn constructor() {
    let token_addr: Key = runtime::get_named_arg("token_addr");
    let name: String = runtime::get_named_arg("name");
    let symbol: String = runtime::get_named_arg("symbol");
    let version: String = runtime::get_named_arg("version");
    let package_hash: ContractPackageHash = runtime::get_named_arg("package_hash");
    let contract_hash: ContractHash = runtime::get_named_arg("contract_hash");
    VotingEscrow::default().constructor(
        token_addr,
        name,
        symbol,
        version,
        contract_hash,
        package_hash,
    );
}

#[no_mangle]
fn commit_transfer_ownership() {
    let addr: Key = runtime::get_named_arg("addr");
    VotingEscrow::default().commit_transfer_ownership(addr);
}

#[no_mangle]
fn apply_transfer_ownership() {
    VotingEscrow::default().apply_transfer_ownership();
}

#[no_mangle]
fn get_last_user_slope() {
    let addr: Key = runtime::get_named_arg("addr");
    let ret: U128 = VotingEscrow::default().get_last_user_slope(addr);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn get_last_user_slope_js_client() {
    let addr: Key = runtime::get_named_arg("addr");
    let ret: U128 = VotingEscrow::default().get_last_user_slope(addr);
    data::js_ret(ret);
}

#[no_mangle]
fn user_point_history_ts() {
    let addr: Key = runtime::get_named_arg("addr");
    let idx: U256 = runtime::get_named_arg("idx");
    let ret: U256 = VotingEscrow::default().user_point_history_ts(addr, idx);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn user_point_history_ts_js_client() {
    let addr: Key = runtime::get_named_arg("addr");
    let idx: U256 = runtime::get_named_arg("idx");
    let ret: U256 = VotingEscrow::default().user_point_history_ts(addr, idx);
    data::js_ret(ret);
}

#[no_mangle]
fn locked_end() {
    let addr: Key = runtime::get_named_arg("addr");
    let ret: U256 = VotingEscrow::default().locked_end(addr);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn locked_end_js_client() {
    let addr: Key = runtime::get_named_arg("addr");
    let ret: U256 = VotingEscrow::default().locked_end(addr);
    data::js_ret(ret);
}

#[no_mangle]
fn checkpoint() {
    VotingEscrow::default().checkpoint();
}

#[no_mangle]
fn deposit_for() {
    let addr: Key = runtime::get_named_arg("addr");
    let value: U256 = runtime::get_named_arg("value");
    VotingEscrow::default().deposit_for(addr, value);
}

#[no_mangle]
fn create_lock() {
    let value: U256 = runtime::get_named_arg("value");
    let unlock_time: U256 = runtime::get_named_arg("unlock_time");
    VotingEscrow::default().create_lock(value, unlock_time);
}

#[no_mangle]
fn increase_amount() {
    let value: U256 = runtime::get_named_arg("value");
    VotingEscrow::default().increase_amount(value)
}

#[no_mangle]
fn increase_unlock_time() {
    let unlock_time: U256 = runtime::get_named_arg("unlock_time");
    VotingEscrow::default().increase_unlock_time(unlock_time);
}

#[no_mangle]
fn withdraw() {
    VotingEscrow::default().withdraw();
}

#[no_mangle]
fn balance_of() {
    let addr: Key = runtime::get_named_arg("addr");
    let t: Option<U256> = runtime::get_named_arg("t");
    let ret: U256 = VotingEscrow::default().balance_of(addr, t);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn balance_of_js_client() {
    let addr: Key = runtime::get_named_arg("addr");
    let t: Option<U256> = runtime::get_named_arg("t");
    let ret: U256 = VotingEscrow::default().balance_of(addr, t);
    data::js_ret(ret);
}

#[no_mangle]
fn balance_of_at() {
    let addr: Key = runtime::get_named_arg("addr");
    let block: U256 = runtime::get_named_arg("block");
    let ret: U256 = VotingEscrow::default().balance_of_at(addr, block);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn balance_of_at_js_client() {
    let addr: Key = runtime::get_named_arg("addr");
    let block: U256 = runtime::get_named_arg("block");
    let ret: U256 = VotingEscrow::default().balance_of_at(addr, block);
    data::js_ret(ret);
}

#[no_mangle]
fn total_supply() {
    let t: Option<U256> = runtime::get_named_arg("t");
    let ret: U256 = VotingEscrow::default().total_supply(t);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn total_supply_js_client() {
    let t: Option<U256> = runtime::get_named_arg("t");
    let ret: U256 = VotingEscrow::default().total_supply(t);
    data::js_ret(ret);
}

#[no_mangle]
fn total_supply_at() {
    let block: U256 = runtime::get_named_arg("block");
    let ret: U256 = VotingEscrow::default().total_supply_at(block);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn total_supply_at_js_client() {
    let block: U256 = runtime::get_named_arg("block");
    let ret: U256 = VotingEscrow::default().total_supply_at(block);
    data::js_ret(ret);
}

#[no_mangle]
fn change_controller() {
    let new_controller: Key = runtime::get_named_arg("new_controller");
    VotingEscrow::default().change_controller(new_controller);
}

// Variables

#[no_mangle]
fn token() {
    runtime::ret(CLValue::from_t(data::get_token()).unwrap_or_revert())
}

#[no_mangle]
fn supply() {
    runtime::ret(CLValue::from_t(data::get_supply()).unwrap_or_revert())
}

#[no_mangle]
fn locked() {
    let addr: Key = runtime::get_named_arg("addr");
    runtime::ret(CLValue::from_t(data::Locked::instance().get(&addr)).unwrap_or_revert())
}

#[no_mangle]
fn epoch() {
    runtime::ret(CLValue::from_t(data::get_epoch()).unwrap_or_revert())
}

#[no_mangle]
fn point_history() {
    let epoch: U256 = runtime::get_named_arg("epoch");
    runtime::ret(CLValue::from_t(data::PointHistory::instance().get(&epoch)).unwrap_or_revert())
}

#[no_mangle]
fn user_point_history() {
    let user: Key = runtime::get_named_arg("user");
    let user_epoch: U256 = runtime::get_named_arg("user_epoch");
    runtime::ret(
        CLValue::from_t(data::UserPointHistory::instance().get(&user, &user_epoch))
            .unwrap_or_revert(),
    )
}

#[no_mangle]
fn user_point_epoch() {
    let user: Key = runtime::get_named_arg("user");
    runtime::ret(CLValue::from_t(data::UserPointEpoch::instance().get(&user)).unwrap_or_revert())
}

#[no_mangle]
fn slope_changes() {
    let time: U256 = runtime::get_named_arg("time");
    runtime::ret(CLValue::from_t(data::SlopeChanges::instance().get(&time)).unwrap_or_revert())
}

#[no_mangle]
fn controller() {
    runtime::ret(CLValue::from_t(data::get_controller()).unwrap_or_revert())
}

#[no_mangle]
fn transfers_enabled() {
    runtime::ret(CLValue::from_t(data::get_transfers_enabled()).unwrap_or_revert())
}

#[no_mangle]
fn name() {
    runtime::ret(CLValue::from_t(data::get_name()).unwrap_or_revert())
}

#[no_mangle]
fn symbol() {
    runtime::ret(CLValue::from_t(data::get_symbol()).unwrap_or_revert())
}

#[no_mangle]
fn version() {
    runtime::ret(CLValue::from_t(data::get_version()).unwrap_or_revert())
}

#[no_mangle]
fn decimals() {
    runtime::ret(CLValue::from_t(data::get_decimals()).unwrap_or_revert())
}

#[no_mangle]
fn admin() {
    runtime::ret(CLValue::from_t(data::get_admin()).unwrap_or_revert())
}

#[no_mangle]
fn future_admin() {
    runtime::ret(CLValue::from_t(data::get_future_admin()).unwrap_or_revert())
}

fn get_entry_points() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        "constructor",
        vec![
            Parameter::new("token_addr", Key::cl_type()),
            Parameter::new("name", String::cl_type()),
            Parameter::new("symbol", String::cl_type()),
            Parameter::new("version", String::cl_type()),
            Parameter::new("package_hash", ContractPackageHash::cl_type()),
            Parameter::new("contract_hash", ContractHash::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Groups(vec![Group::new("constructor")]),
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
        "apply_transfer_ownership",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "get_last_user_slope",
        vec![Parameter::new("addr", Key::cl_type())],
        U128::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "get_last_user_slope_js_client",
        vec![Parameter::new("addr", Key::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "user_point_history_ts",
        vec![
            Parameter::new("addr", Key::cl_type()),
            Parameter::new("idx", U256::cl_type()),
        ],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "user_point_history_ts_js_client",
        vec![
            Parameter::new("addr", Key::cl_type()),
            Parameter::new("idx", U256::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "locked_end",
        vec![Parameter::new("addr", Key::cl_type())],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "locked_end_js_client",
        vec![Parameter::new("addr", Key::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "checkpoint",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "deposit_for",
        vec![
            Parameter::new("addr", Key::cl_type()),
            Parameter::new("value", U256::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "create_lock",
        vec![
            Parameter::new("value", U256::cl_type()),
            Parameter::new("unlock_time", U256::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "increase_amount",
        vec![Parameter::new("value", U256::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "increase_unlock_time",
        vec![Parameter::new("unlock_time", U256::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "withdraw",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "balance_of",
        vec![
            Parameter::new("addr", Key::cl_type()),
            Parameter::new("t", CLType::Option(Box::new(CLType::U256))),
        ],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "balance_of_js_client",
        vec![
            Parameter::new("addr", Key::cl_type()),
            Parameter::new("t", CLType::Option(Box::new(CLType::U256))),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "balance_of_at",
        vec![
            Parameter::new("addr", Key::cl_type()),
            Parameter::new("block", U256::cl_type()),
        ],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "balance_of_at_js_client",
        vec![
            Parameter::new("addr", Key::cl_type()),
            Parameter::new("block", U256::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "total_supply",
        vec![Parameter::new("t", CLType::Option(Box::new(CLType::U256)))],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "total_supply_js_client",
        vec![Parameter::new("t", CLType::Option(Box::new(CLType::U256)))],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "total_supply_at",
        vec![Parameter::new("block", U256::cl_type())],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "total_supply_at_js_client",
        vec![Parameter::new("block", U256::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "change_controller",
        vec![Parameter::new("new_controller", Key::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    // Variables
    entry_points.add_entry_point(EntryPoint::new(
        "token",
        vec![],
        Key::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "supply",
        vec![],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "locked",
        vec![Parameter::new("addr", Key::cl_type())],
        data::LockedBalance::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "epoch",
        vec![],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "point_history",
        vec![Parameter::new("epoch", U256::cl_type())],
        data::Point::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "user_point_history",
        vec![
            Parameter::new("user", Key::cl_type()),
            Parameter::new("user_epoch", U256::cl_type()),
        ],
        data::Point::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "user_point_epoch",
        vec![Parameter::new("user", Key::cl_type())],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "slope_changes",
        vec![Parameter::new("time", U256::cl_type())],
        U128::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "controller",
        vec![],
        Key::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "transfers_enabled",
        vec![],
        bool::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "name",
        vec![],
        String::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "symbol",
        vec![],
        String::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "version",
        vec![],
        String::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "decimals",
        vec![],
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

        let token_addr: Key = runtime::get_named_arg("token_addr");
        let name: String = runtime::get_named_arg("name");
        let symbol: String = runtime::get_named_arg("symbol");
        let version: String = runtime::get_named_arg("version");
        let constructor_args = runtime_args! {
           "token_addr" => token_addr,
           "name" => name,
           "symbol" => symbol,
           "version" => version,
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
