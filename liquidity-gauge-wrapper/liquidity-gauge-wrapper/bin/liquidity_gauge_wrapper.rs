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
    EntryPointAccess, EntryPointType, EntryPoints, Group, Key, Parameter, RuntimeArgs, URef,
    U256,
};
use contract_utils::{ContractContext, OnChainContractStorage};
use liquidity_gauge_wrapper_crate::{
    self,
    data::{self, *},
    LIQUIDITYGAUGEWRAPPER,
};

#[derive(Default)]
struct LiquidityGaugeWrapper(OnChainContractStorage);
impl ContractContext<OnChainContractStorage> for LiquidityGaugeWrapper {
    fn storage(&self) -> &OnChainContractStorage {
        &self.0
    }
}

impl LIQUIDITYGAUGEWRAPPER<OnChainContractStorage> for LiquidityGaugeWrapper {}
impl LiquidityGaugeWrapper {
    fn constructor(
        &mut self,
        name: String,
        symbol: String,
        gauge: Key,
        admin: Key,
        contract_hash: ContractHash,
        package_hash: ContractPackageHash,
    ) {
        LIQUIDITYGAUGEWRAPPER::init(
            self,
            name,
            symbol,
            gauge,
            admin,
            contract_hash,
            package_hash,
        );
    }
}

#[no_mangle]
fn constructor() {
    let name: String = runtime::get_named_arg("name");
    let symbol: String = runtime::get_named_arg("symbol");
    let gauge: Key = runtime::get_named_arg("gauge");
    let admin: Key = runtime::get_named_arg("admin");
    let contract_hash: ContractHash = runtime::get_named_arg("contract_hash");
    let package_hash: ContractPackageHash = runtime::get_named_arg("package_hash");
    LiquidityGaugeWrapper::default().constructor(
        name,
        symbol,
        gauge,
        admin,
        contract_hash,
        package_hash,
    );
}
#[no_mangle]
fn user_checkpoint() {
    let addr: Key = runtime::get_named_arg("addr");
    let ret: bool = LiquidityGaugeWrapper::default().user_checkpoint(addr);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn user_checkpoint_js_client() {
    let addr: Key = runtime::get_named_arg("addr");
    let ret: bool = LiquidityGaugeWrapper::default().user_checkpoint(addr);
    js_ret(ret);
}
#[no_mangle]
fn claimable_tokens() {
    let addr: Key = runtime::get_named_arg("addr");
    let ret: U256 = LiquidityGaugeWrapper::default().claimable_tokens(addr);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn claimable_tokens_js_client() {
    let addr: Key = runtime::get_named_arg("addr");
    let ret: U256 = LiquidityGaugeWrapper::default().claimable_tokens(addr);
    js_ret(ret);
}
#[no_mangle]
fn claim_tokens() {
    let addr: Option<Key> = runtime::get_named_arg("addr");
    LiquidityGaugeWrapper::default().claim_tokens(addr);
}
#[no_mangle]
fn set_approve_deposit() {
    let addr: Key = runtime::get_named_arg("addr");
    let can_deposit: bool = runtime::get_named_arg("can_deposit");
    LiquidityGaugeWrapper::default().set_approve_deposit(addr, can_deposit);
}

#[no_mangle]
fn deposit() {
    let value: U256 = runtime::get_named_arg("value");
    let addr: Option<Key> = runtime::get_named_arg("addr");
    LiquidityGaugeWrapper::default().deposit(value, addr);
}

#[no_mangle]
fn withdraw() {
    let value: U256 = runtime::get_named_arg("value");
    LiquidityGaugeWrapper::default().withdraw(value);
}
#[no_mangle]
fn allowance() {
    let owner: Key = runtime::get_named_arg("owner");
    let spender: Key = runtime::get_named_arg("spender");
    let ret = LiquidityGaugeWrapper::default().allowance(owner, spender);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn transfer() {
    let recipient: Key = runtime::get_named_arg("recipient");
    let amount: U256 = runtime::get_named_arg("amount");
    let ret = LiquidityGaugeWrapper::default().transfer(recipient, amount);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn transfer_from() {
    let owner: Key = runtime::get_named_arg("owner");
    let recipient: Key = runtime::get_named_arg("recipient");
    let amount: U256 = runtime::get_named_arg("amount");
    let ret = LiquidityGaugeWrapper::default().transfer_from(owner, recipient, amount);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn approve() {
    let spender: Key = runtime::get_named_arg("spender");
    let amount: U256 = runtime::get_named_arg("amount");
    LiquidityGaugeWrapper::default().approve(spender, amount);
}
#[no_mangle]
fn increase_allowance() {
    let spender: Key = runtime::get_named_arg("spender");
    let amount: U256 = runtime::get_named_arg("amount");
    let ret: Result<(), u32> = LiquidityGaugeWrapper::default().increase_allowance(spender, amount);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn decrease_allowance() {
    let spender: Key = runtime::get_named_arg("spender");
    let amount: U256 = runtime::get_named_arg("amount");
    let ret: Result<(), u32> = LiquidityGaugeWrapper::default().decrease_allowance(spender, amount);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn kill_me() {
    LiquidityGaugeWrapper::default().kill_me();
}

#[no_mangle]
fn commit_transfer_ownership() {
    let addr: Key = runtime::get_named_arg("addr");
    LiquidityGaugeWrapper::default().commit_transfer_ownership(addr);
}

#[no_mangle]
fn apply_transfer_ownership() {
    LiquidityGaugeWrapper::default().apply_transfer_ownership();
}
//Variables
#[no_mangle]
fn minter() {
    runtime::ret(CLValue::from_t(data::get_minter()).unwrap_or_revert());
}

#[no_mangle]
fn crv_token() {
    runtime::ret(CLValue::from_t(data::get_crv_token()).unwrap_or_revert());
}

#[no_mangle]
fn lp_token() {
    runtime::ret(CLValue::from_t(data::get_lp_token()).unwrap_or_revert());
}
#[no_mangle]
fn gauge() {
    runtime::ret(CLValue::from_t(data::get_gauge()).unwrap_or_revert());
}
#[no_mangle]
fn balance_of() {
    let key: Key = runtime::get_named_arg("key");
    runtime::ret(CLValue::from_t(data::BalanceOf::instance().get(&key)).unwrap_or_revert());
}

#[no_mangle]
fn total_supply() {
    runtime::ret(CLValue::from_t(data::get_total_supply()).unwrap_or_revert());
}
#[no_mangle]
fn name() {
    runtime::ret(CLValue::from_t(data::name()).unwrap_or_revert());
}
#[no_mangle]
fn symbol() {
    runtime::ret(CLValue::from_t(data::symbol()).unwrap_or_revert());
}
#[no_mangle]
fn decimals() {
    runtime::ret(CLValue::from_t(data::decimals()).unwrap_or_revert());
}
#[no_mangle]
fn approved_to_deposit() {
    let key0: Key = runtime::get_named_arg("key0");
    let key1: Key = runtime::get_named_arg("key1");
    runtime::ret(
        CLValue::from_t(data::ApprovedToDeposit::instance().get(&key0, &key1)).unwrap_or_revert(),
    );
}
#[no_mangle]
fn claimable_crv() {
    let key: Key = runtime::get_named_arg("key");
    runtime::ret(CLValue::from_t(data::ClaimableCrv::instance().get(&key)).unwrap_or_revert());
}
#[no_mangle]
fn admin() {
    runtime::ret(CLValue::from_t(data::get_admin()).unwrap_or_revert());
}

#[no_mangle]
fn future_admin() {
    runtime::ret(CLValue::from_t(data::get_future_admin()).unwrap_or_revert());
}

#[no_mangle]
fn is_killed() {
    runtime::ret(CLValue::from_t(data::get_is_killed()).unwrap_or_revert());
}
//Entry Points
fn get_entry_points() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        "constructor",
        vec![
            Parameter::new("name", String::cl_type()),
            Parameter::new("symbol", String::cl_type()),
            Parameter::new("gauge", Key::cl_type()),
            Parameter::new("admin", Key::cl_type()),
            Parameter::new("contract_hash", ContractHash::cl_type()),
            Parameter::new("package_hash", ContractPackageHash::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Groups(vec![Group::new("constructor")]),
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "user_checkpoint",
        vec![Parameter::new("addr", Key::cl_type())],
        bool::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "user_checkpoint_js_client",
        vec![Parameter::new("addr", Key::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "claimable_tokens",
        vec![Parameter::new("addr", Key::cl_type())],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "claimable_tokens_js_client",
        vec![Parameter::new("addr", Key::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "claim_tokens",
        vec![Parameter::new("addr",CLType::Option(Box::new(CLType::Key)))],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "set_approve_deposit",
        vec![
            Parameter::new("addr", Key::cl_type()),
            Parameter::new("can_deposit", bool::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "deposit",
        vec![
            Parameter::new("value", U256::cl_type()),
            Parameter::new("addr", CLType::Option(Box::new(CLType::Key))),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "withdraw",
        vec![Parameter::new("value", U256::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "allowance",
        vec![
            Parameter::new("owner", Key::cl_type()),
            Parameter::new("spender", Key::cl_type()),
        ],
        U256::cl_type(),
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
    entry_points.add_entry_point(EntryPoint::new(
        "transfer_from",
        vec![
            Parameter::new("owner", Key::cl_type()),
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
    entry_points.add_entry_point(EntryPoint::new(
        "approve",
        vec![
            Parameter::new("spender", Key::cl_type()),
            Parameter::new("amount", U256::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "increase_allowance",
        vec![
            Parameter::new("spender", Key::cl_type()),
            Parameter::new("amount", U256::cl_type()),
        ],
        CLType::Result {
            ok: Box::new(CLType::Unit),
            err: Box::new(CLType::U32),
        },
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "decrease_allowance",
        vec![
            Parameter::new("spender", Key::cl_type()),
            Parameter::new("amount", U256::cl_type()),
        ],
        CLType::Result {
            ok: Box::new(CLType::Unit),
            err: Box::new(CLType::U32),
        },
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
    //Variables
    entry_points.add_entry_point(EntryPoint::new(
        "minter",
        vec![],
        Key::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "crv_token",
        vec![],
        Key::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "lp_token",
        vec![],
        Key::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "gauge",
        vec![],
        Key::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "balance_of",
        vec![Parameter::new("key", Key::cl_type())],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "total_supply",
        vec![],
        U256::cl_type(),
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
        "decimals",
        vec![],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "approved_to_deposit",
        vec![
            Parameter::new("key0", Key::cl_type()),
            Parameter::new("key1", Key::cl_type()),
        ],
        bool::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "claimable_crv",
        vec![Parameter::new("key", Key::cl_type())],
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

        let name: String = runtime::get_named_arg("name");
        let symbol: String = runtime::get_named_arg("symbol");
        let gauge: Key = runtime::get_named_arg("gauge");
        let admin: Key = runtime::get_named_arg("admin");
        let constructor_args = runtime_args! {
            "name" => name,
            "symbol" => symbol,
            "gauge" => gauge,
            "admin" => admin,
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
