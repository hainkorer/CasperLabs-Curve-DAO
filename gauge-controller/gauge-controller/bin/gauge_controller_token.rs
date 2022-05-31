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
    EntryPointAccess, EntryPointType, EntryPoints, Group, Key, Parameter, RuntimeArgs, URef, U128,
    U256,
};
use contract_utils::{ContractContext, OnChainContractStorage};
use gauge_controller_crate::data::{Point, VotedSlope};
use gauge_controller_crate::GAUGECONLTROLLER;

#[derive(Default)]
struct Token(OnChainContractStorage);

impl ContractContext<OnChainContractStorage> for Token {
    fn storage(&self) -> &OnChainContractStorage {
        &self.0
    }
}

impl GAUGECONLTROLLER<OnChainContractStorage> for Token {}
impl Token {
    fn constructor(
        &mut self,
        token: Key,
        voting_escrow: Key,
        contract_hash: ContractHash,
        package_hash: ContractPackageHash,
    ) {
        GAUGECONLTROLLER::init(
            self,
            token,
            voting_escrow,
            Key::from(contract_hash),
            package_hash,
        );
    }
}

#[no_mangle]
fn constructor() {
    let token: Key = runtime::get_named_arg::<Key>("token");
    let voting_escrow: Key = runtime::get_named_arg("voting_escrow");
    let contract_hash: ContractHash = runtime::get_named_arg("contract_hash");
    let package_hash: ContractPackageHash = runtime::get_named_arg("package_hash");

    Token::default().constructor(token, voting_escrow, contract_hash, package_hash);
}

/// """
/// @notice Transfer ownership of GaugeController to `addr`
/// @param addr Address to have ownership transferred to
/// """

#[no_mangle]
fn commit_transfer_ownership() {
    let addr: Key = runtime::get_named_arg::<Key>("addr");
    Token::default().commit_transfer_ownership(addr);
}

/// """
/// @notice Apply pending ownership transfer
/// """
#[no_mangle]
fn apply_transfer_ownership() {
    Token::default().apply_transfer_ownership();
}

/// """
/// @notice Get gauge type for address
/// @param _addr Gauge address
/// @return Gauge type id
/// """
#[no_mangle]
fn gauge_types() {
    let addr: Key = runtime::get_named_arg::<Key>("addr");
    let ret: U128 = Token::default().gauge_types(addr);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

/// """
/// @notice Checkpoint to fill data common for all gauges
/// """
#[no_mangle]
fn checkpoint() {
    Token::default().checkpoint();
}

/// """
/// @notice Checkpoint to fill data for both a specific gauge and common for all gauges
/// @param addr Gauge address
/// """
#[no_mangle]
fn checkpoint_gauge() {
    let addr: Key = runtime::get_named_arg::<Key>("addr");
    Token::default().checkpoint_gauge(addr);
}

#[no_mangle]
fn package_hash() {
    let ret: ContractPackageHash = Token::default().get_package_hash();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn gauge_relative_weight() {
    // @notice Get Gauge relative weight (not more than 1.0) normalized to 1e18
    //         (e.g. 1.0 == 1e18). Inflation which will be received by it is
    //         inflation_rate * relative_weight / 1e18
    // @param addr Gauge address
    // @param time Relative weight at the specified timestamp in the past or present
    // @return Value of relative weight normalized to 1e18

    let addr: Key = runtime::get_named_arg("addr");
    let ret: U256 = Token::default().gauge_relative_weight(addr);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn gauge_relative_weight_write() {
    // @notice Get gauge weight normalized to 1e18 and also fill all the unfilled
    //         values for type and gauge records
    // @dev Any address can call, however nothing is recorded if the values are filled already
    // @param addr Gauge address
    // @param time Relative weight at the specified timestamp in the past or present
    // @return Value of relative weight normalized to 1e18

    let addr: Key = runtime::get_named_arg("addr");
    let ret: U256 = Token::default().gauge_relative_weight_write(addr);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn change_type_weight() {
    // @notice Change gauge type `type_id` weight to `weight`
    // @param type_id Gauge type id
    // @param weight New Gauge weight

    let type_id: U128 = runtime::get_named_arg("type_id");
    let weight: U256 = runtime::get_named_arg("weight");

    Token::default().change_type_weight(type_id, weight);
}

#[no_mangle]
fn change_gauge_weight() {
    // @notice Change weight of gauge `addr` to `weight`
    // @param addr `GaugeController` contract address
    // @param weight New Gauge weight

    let addr: Key = runtime::get_named_arg("addr");
    let weight: U256 = runtime::get_named_arg("weight");

    Token::default().change_gauge_weight(addr, weight);
}

#[no_mangle]
fn get_gauge_weight() {
    // @notice Get current gauge weight
    // @param addr Gauge address
    // @return Gauge weight

    let addr: Key = runtime::get_named_arg("addr");
    let ret: U256 = Token::default().get_gauge_weight(addr);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn get_type_weight() {
    // @notice Get current type weight
    // @param type_id Type id
    // @return Type weight

    let type_id: U128 = runtime::get_named_arg("type_id");
    let ret: U256 = Token::default().get_type_weight(type_id);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn get_total_weight() {
    // @notice Get current total (type-weighted) weight
    // @return Total weight

    let ret: U256 = Token::default().get_total_weight();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn get_weights_sum_per_type() {
    // @notice Get sum of gauge weights per type
    // @param type_id Type id
    // @return Sum of gauge weights

    let type_id: U128 = runtime::get_named_arg("type_id");
    let ret: U256 = Token::default().get_weights_sum_per_type(type_id);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn change_sum() {
    let key0: U128 = runtime::get_named_arg("key0");
    let key1: U256 = runtime::get_named_arg("key1");
    let ret: U256 = Token::default().change_sum(key0, key1);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn changes_weight() {
    let key0: Key = runtime::get_named_arg("key0");
    let key1: U256 = runtime::get_named_arg("key1");
    let ret: U256 = Token::default().changes_weight(key0, key1);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn gauge_type_names() {
    let key0: U128 = runtime::get_named_arg("key0");
    let ret: String = Token::default().gauge_type_names(key0);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn gauge_types_() {
    let key0: Key = runtime::get_named_arg("key0");
    let ret: U128 = Token::default().gauge_types_(key0);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn gauges() {
    let key0: U256 = runtime::get_named_arg("key0");
    let ret: Key = Token::default().gauges(key0);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn last_user_vote() {
    let key0: Key = runtime::get_named_arg("key0");
    let key1: Key = runtime::get_named_arg("key1");
    let ret: U256 = Token::default().last_user_vote(key0, key1);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn points_sum() {
    let key0: U128 = runtime::get_named_arg("key0");
    let key1: U256 = runtime::get_named_arg("key1");
    let ret: Point = Token::default().points_sum(key0, key1);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn points_total() {
    let key0: U256 = runtime::get_named_arg("key0");
    let ret: U256 = Token::default().points_total(key0);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn points_type_weight() {
    let key0: U128 = runtime::get_named_arg("key0");
    let key1: U256 = runtime::get_named_arg("key1");
    let ret: U256 = Token::default().points_type_weight(key0, key1);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn points_weight() {
    let key0: Key = runtime::get_named_arg("key0");
    let key1: U256 = runtime::get_named_arg("key1");
    let ret: Point = Token::default().points_weight(key0, key1);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn time_sum() {
    let key0: U256 = runtime::get_named_arg("key0");
    let ret: U256 = Token::default().time_sum(key0);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn time_type_weight() {
    let key0: U256 = runtime::get_named_arg("key0");
    let ret: U256 = Token::default().time_type_weight(key0);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn time_weight() {
    let key0: Key = runtime::get_named_arg("key0");
    let ret: U256 = Token::default().time_weight(key0);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn vote_user_power() {
    let key0: Key = runtime::get_named_arg("key0");
    let ret: U256 = Token::default().vote_user_power(key0);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn vote_user_slopes() {
    let key0: Key = runtime::get_named_arg("key0");
    let key1: Key = runtime::get_named_arg("key1");
    let ret: VotedSlope = Token::default().vote_user_slopes(key0, key1);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
// TimeWeight, VoteUserPower, VoteUserSlopes,
#[no_mangle]
fn time_total() {
    let ret: U256 = Token::default().time_total();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn token() {
    let ret: Key = Token::default().token();
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
fn voting_escrow() {
    let ret: Key = Token::default().voting_escrow();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn n_gauge_types() {
    let ret: U128 = Token::default().n_gauge_types();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn add_type() {
    // @notice Add gauge type with name `_name` and weight `weight`
    // @param _name Name of gauge type
    // @param weight Weight of gauge type

    let _name: String = runtime::get_named_arg("_name");
    Token::default().add_type(_name);
}

#[no_mangle]
fn add_gauge() {
    // @notice Add gauge `addr` of type `gauge_type` with weight `weight`
    // @param addr Gauge address
    // @param gauge_type Gauge type
    // @param weight Gauge weight

    let addr: Key = runtime::get_named_arg("addr");
    let gauge_type: U128 = runtime::get_named_arg("gauge_type");
    let weight: Option<U256> = runtime::get_named_arg("weight");
    Token::default().add_gauge(addr, gauge_type, weight);
}

#[no_mangle]
fn vote_for_gauge_weights() {
    // @notice Allocate voting power for changing pool weights
    // @param _gauge_addr Gauge which `msg.sender` votes for
    // @param _user_weight Weight for a gauge in bps (units of 0.01%). Minimal is 0.01%. Ignored if 0

    let _gauge_addr: Key = runtime::get_named_arg("_gauge_addr");
    let _user_weight: U256 = runtime::get_named_arg("_user_weight");
    Token::default().vote_for_gauge_weights(_gauge_addr, _user_weight);
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
        let voting_escrow: Key = runtime::get_named_arg("voting_escrow");

        // Prepare constructor args
        let constructor_args = runtime_args! {
            "token" => token,
            "voting_escrow" => voting_escrow,
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
            Parameter::new("voting_escrow", Key::cl_type()),
            Parameter::new("contract_hash", ContractHash::cl_type()),
            Parameter::new("package_hash", ContractPackageHash::cl_type()),
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
        "checkpoint",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "checkpoint_gauge",
        vec![Parameter::new("addr", Key::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "gauge_types",
        vec![Parameter::new("addr", Key::cl_type())],
        U128::cl_type(),
        EntryPointAccess::Public,
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
        "n_gauge_types",
        vec![],
        U128::cl_type(),
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
        "future_admin",
        vec![],
        Key::cl_type(),
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
        "token",
        vec![],
        Key::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "time_total",
        vec![],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "vote_user_slopes",
        vec![
            Parameter::new("key0", Key::cl_type()),
            Parameter::new("key1", Key::cl_type()),
        ],
        VotedSlope::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "vote_user_power",
        vec![Parameter::new("key0", Key::cl_type())],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "time_weight",
        vec![Parameter::new("key0", Key::cl_type())],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "time_type_weight",
        vec![Parameter::new("key0", U256::cl_type())],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "time_sum",
        vec![Parameter::new("key0", U256::cl_type())],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "points_weight",
        vec![
            Parameter::new("key0", Key::cl_type()),
            Parameter::new("key1", U256::cl_type()),
        ],
        Point::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "points_type_weight",
        vec![
            Parameter::new("key0", U128::cl_type()),
            Parameter::new("key1", U256::cl_type()),
        ],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "points_total",
        vec![Parameter::new("key0", U256::cl_type())],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "points_sum",
        vec![
            Parameter::new("key0", U128::cl_type()),
            Parameter::new("key1", U256::cl_type()),
        ],
        Point::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "last_user_vote",
        vec![
            Parameter::new("key0", Key::cl_type()),
            Parameter::new("key1", Key::cl_type()),
        ],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "gauges",
        vec![Parameter::new("key0", U256::cl_type())],
        Key::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "gauge_types_",
        vec![Parameter::new("key0", Key::cl_type())],
        U128::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "gauge_type_names",
        vec![Parameter::new("key0", U128::cl_type())],
        String::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "changes_weight",
        vec![
            Parameter::new("key0", Key::cl_type()),
            Parameter::new("key1", U256::cl_type()),
        ],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "change_sum",
        vec![
            Parameter::new("key0", U128::cl_type()),
            Parameter::new("key1", U256::cl_type()),
        ],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "gauge_relative_weight",
        vec![Parameter::new("addr", Key::cl_type())],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "gauge_relative_weight_write",
        vec![Parameter::new("addr", Key::cl_type())],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "change_type_weight",
        vec![
            Parameter::new("type_id", U128::cl_type()),
            Parameter::new("weight", U256::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "change_gauge_weight",
        vec![
            Parameter::new("addr", Key::cl_type()),
            Parameter::new("weight", U256::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "get_gauge_weight",
        vec![Parameter::new("addr", Key::cl_type())],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "get_type_weight",
        vec![Parameter::new("type_id", U128::cl_type())],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "get_total_weight",
        vec![],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "get_weights_sum_per_type",
        vec![Parameter::new("type_id", U128::cl_type())],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "add_type",
        vec![Parameter::new("_name", String::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "add_gauge",
        vec![
            Parameter::new("addr", Key::cl_type()),
            Parameter::new("type_id", U128::cl_type()),
            Parameter::new("weight", CLType::Option(Box::new(U256::cl_type()))),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "vote_for_gauge_weights",
        vec![
            Parameter::new("_gauge_addr", Key::cl_type()),
            Parameter::new("_user_weight", U256::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points
}
