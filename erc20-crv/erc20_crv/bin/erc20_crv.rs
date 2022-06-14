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
use contract_utils::{set_key, ContractContext, OnChainContractStorage};
use erc20_crate::{self, data as erc20_data, ERC20};
use erc20_crv::{self, ERC20CRV, data};

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
    fn constructor(
        &mut self,
        name: String,
        symbol: String,
        decimal: u8,
        contract_hash: ContractHash,
        package_hash: ContractPackageHash,
    ) {
        ERC20CRV::init(
            self,
            name,
            symbol,
            decimal,
            Key::from(contract_hash),
            package_hash,
        );
    }
}

#[no_mangle]
fn constructor() {
    let name: String = runtime::get_named_arg("name");
    let symbol: String = runtime::get_named_arg("symbol");
    let decimal: u8 = runtime::get_named_arg("decimal");
    let contract_hash: ContractHash = runtime::get_named_arg("contract_hash");
    let package_hash: ContractPackageHash = runtime::get_named_arg("package_hash");
    Erc20Crv::default().constructor(name, symbol, decimal, contract_hash, package_hash);
}
#[no_mangle]
fn set_minter() {
    let _minter: Key = runtime::get_named_arg("_minter");
    Erc20Crv::default().set_minter(_minter);
}
#[no_mangle]
fn burn() {
    let _value: U256 = runtime::get_named_arg("_value");
    Erc20Crv::default().burn_caller(_value);
}
#[no_mangle]
fn set_admin() {
    let admin: Key = runtime::get_named_arg("admin");
    Erc20Crv::default().set_admin(admin);
}
#[no_mangle]
fn start_epoch_time_write() {
    let ret = Erc20Crv::default().start_epoch_time_write();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn start_epoch_time_write_js_client() {
    let ret = Erc20Crv::default().start_epoch_time_write();
    set_key("result", ret);
}
#[no_mangle]
fn future_epoch_time_write() {
    let ret = Erc20Crv::default().future_epoch_time_write();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn future_epoch_time_write_js_client() {
    let ret = Erc20Crv::default().future_epoch_time_write();
    set_key("result", ret);
}
#[no_mangle]
fn available_supply() {
    let ret = Erc20Crv::default().available_supply();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn available_supply_js_client() {
    let ret = Erc20Crv::default().available_supply();
    set_key("result", ret);
}

#[no_mangle]
fn mintable_in_timeframe() {
    let start: U256 = runtime::get_named_arg("start");
    let end: U256 = runtime::get_named_arg("end");
    let ret = Erc20Crv::default().mintable_in_timeframe(start, end);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn mintable_in_timeframe_js_client() {
    let start: U256 = runtime::get_named_arg("start");
    let end: U256 = runtime::get_named_arg("end");
    let ret = Erc20Crv::default().mintable_in_timeframe(start, end);
    set_key("result", ret);
}

#[no_mangle]
fn update_mining_parameters() {
    Erc20Crv::default().update_mining_parameters();
}
#[no_mangle]
fn mint() {
    let to: Key = runtime::get_named_arg("to");
    let value: U256 = runtime::get_named_arg("value");
    let ret = Erc20Crv::default().mint(to, value);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn mint_js_client() {
    let to: Key = runtime::get_named_arg("to");
    let value: U256 = runtime::get_named_arg("value");
    let ret = Erc20Crv::default().mint(to, value);
    set_key("result", ret);
}
#[no_mangle]
fn transfer_from() {
    let owner: Key = runtime::get_named_arg("owner");
    let recipient: Key = runtime::get_named_arg("recipient");
    let amount: U256 = runtime::get_named_arg("amount");
    let ret = ERC20::transfer_from(&mut Erc20Crv::default(),owner,recipient, amount);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn approve() {
    let spender: Key = runtime::get_named_arg("spender");
    let amount: U256 = runtime::get_named_arg("amount");
    ERC20::approve(&mut Erc20Crv::default(),spender, amount);
}
#[no_mangle]
fn transfer() {
    let recipient: Key = runtime::get_named_arg("recipient");
    let amount: U256 = runtime::get_named_arg("amount");
    let ret = ERC20::transfer(&mut Erc20Crv::default(),recipient, amount);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
//[no_mangle] of public variables
#[no_mangle]
fn name() {
    runtime::ret(CLValue::from_t(erc20_data::name()).unwrap_or_revert());
#[no_mangle]
fn symbol() {
    runtime::ret(CLValue::from_t(erc20_data::symbol()).unwrap_or_revert());
}
#[no_mangle]
fn decimals() {
    runtime::ret(CLValue::from_t(erc20_data::decimals()).unwrap_or_revert());
}
#[no_mangle]
fn balance_of() {
    let owner: Key = runtime::get_named_arg("owner");
    runtime::ret(CLValue::from_t(erc20_data::Balances::instance().get(&owner)).unwrap_or_revert());
}
#[no_mangle]
fn allowances() {
    let key1: Key = runtime::get_named_arg("key1");
    let key2: Key = runtime::get_named_arg("key2");
    runtime::ret(CLValue::from_t(erc20_data::Allowances::instance().get(&key1,&key2)).unwrap_or_revert());
}
#[no_mangle]
fn minter() {
    runtime::ret(CLValue::from_t(data::get_minter()).unwrap_or_revert());
}
#[no_mangle]
fn admin() {
    runtime::ret(CLValue::from_t(data::get_admin()).unwrap_or_revert());
}

#[no_mangle]
fn rate() {
    runtime::ret(CLValue::from_t(data::get_rate()).unwrap_or_revert());
}

fn get_entry_points() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        "constructor",
        vec![
            Parameter::new("name", String::cl_type()),
            Parameter::new("symbol", String::cl_type()),
            Parameter::new("decimal", u8::cl_type()),
            Parameter::new("contract_hash", ContractHash::cl_type()),
            Parameter::new("package_hash", ContractPackageHash::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Groups(vec![Group::new("constructor")]),
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "set_minter",
        vec![Parameter::new("_minter", Key::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "burn",
        vec![Parameter::new("_value", U256::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "set_admin",
        vec![Parameter::new("admin", Key::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "update_mining_parameters",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "start_epoch_time_write",
        vec![],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "start_epoch_time_write_js_client",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "future_epoch_time_write",
        vec![],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "future_epoch_time_write_js_client",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "available_supply",
        vec![],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "available_supply_js_client",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "mintable_in_timeframe",
        vec![
            Parameter::new("start", U256::cl_type()),
            Parameter::new("end", U256::cl_type()),
        ],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "mintable_in_timeframe_js_client",
        vec![
            Parameter::new("start", U256::cl_type()),
            Parameter::new("end", U256::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "mint",
        vec![
            Parameter::new("to", Key::cl_type()),
            Parameter::new("value", U256::cl_type()),
        ],
        bool::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "mint_js_client",
        vec![
            Parameter::new("to", Key::cl_type()),
            Parameter::new("value", U256::cl_type()),
        ],
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
        CLType::Result {
            ok: Box::new(CLType::Unit),
            err: Box::new(CLType::U32),
        },
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
     //entry points of public variables
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
        "decimals",
        vec![],
        u8::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "balance_of",
        vec![Parameter::new("owner", Key::cl_type())],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "allowances",
        vec![
            Parameter::new("key1", Key::cl_type()),
            Parameter::new("key2", Key::cl_type()),
        ],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "minter",
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
        "rate",
        vec![],
        U256::cl_type(),
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
    let name: String = runtime::get_named_arg("name");
    let symbol: String = runtime::get_named_arg("symbol");
    let decimal: u8 = runtime::get_named_arg("decimal");
    let constructor_args = runtime_args! {
            "name" => name,
            "symbol" => symbol,
            "decimal" => decimal,
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
}
