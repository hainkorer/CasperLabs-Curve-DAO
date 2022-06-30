use crate::erc20_crv_instance::ERC20CRVInstance;
use casper_types::{
    account::AccountHash, runtime_args, Key, RuntimeArgs, U256
};
use common::keys::*;
use casperlabs_test_env::{TestContract, TestEnv};

fn deploy() -> (TestEnv, AccountHash, ERC20CRVInstance) {
    let env = TestEnv::new();
    let owner = env.next_user();
    let instance = ERC20CRVInstance::new(
        &env,
        "ERC20CRV",
        owner,
        "ERC20CRV".to_string(),
        "erc20_crv".to_string(),
        u8::from(2 as u8),
    );
    (env, owner, instance)
}

#[test]
fn test_deploy() {
    let (_, _, _) = deploy();
}

#[test]
fn burn() {
    let (env, owner, contract) = deploy();
    let to: Key = Key::Account(owner);
    let value: U256 = 10.into();
    let minter = Key::from(owner);
    contract.set_minter(owner, minter);
    TestContract::new(
        &env,
        "erc20-crv-session-code.wasm",
        "SessionCode",
        owner,
        runtime_args! {
            "entrypoint" => String::from(MINT),
            "package_hash" => Key::Hash(contract.package_hash()),
            "to"=>to,
            "value"=>value
        },
        1000000000,
    );

    let ret: bool = env.query_account_named_key(owner, &[MINT.into()]);
    assert_eq!(ret, true);

    contract.burn(owner, value);
}
#[test]
fn set_admin() {
    let (env, owner, contract) = deploy();
    let admin: Key = Key::from(env.next_user());
    contract.set_admin(owner, admin);
}
#[test]
fn test_set_minter() {
    let (env, owner, contract) = deploy();
    let minter = Key::from(env.next_user());
    contract.set_minter(owner, minter);
}
#[test]
fn test_update_mining_parameters() {
    let (_, owner, contract) = deploy();
    contract.update_mining_parameters(owner);
}
#[test]
fn test_start_epoch_time_write() {
    let (env, owner, contract) = deploy();
    TestContract::new(
        &env,
        "erc20-crv-session-code.wasm",
        "SessionCode",
        owner,
        runtime_args! {
            "entrypoint" => String::from(START_EPOCH_TIME_WRITE),
            "package_hash" => Key::Hash(contract.package_hash())
        },
        1000000000,
    );
    let ret: U256 = env.query_account_named_key(owner, &[START_EPOCH_TIME_WRITE.into()]);
    assert_eq!(ret, 100086400.into());
}
#[test]
fn test_start_epoch_time_write_js_client() {
    let (_, owner, contract) = deploy();
    contract.start_epoch_time_write_js_client(owner);
    let ret: U256 = contract.key_value(RESULT.to_string());
    assert_eq!(ret, 100086400.into());
}

#[test]
fn test_future_epoch_time_write() {
    let (env, owner, contract) = deploy();
    TestContract::new(
        &env,
        "erc20-crv-session-code.wasm",
        "SessionCode",
        owner,
        runtime_args! {
            "entrypoint" => String::from(FUTURE_EPOCH_TIME_WRITE),
            "package_hash" => Key::Hash(contract.package_hash())
        },
        1000000000,
    );

    let ret: U256 = env.query_account_named_key(owner, &[FUTURE_EPOCH_TIME_WRITE.into()]);
    assert_eq!(ret, 131622400.into());
}
#[test]
fn test_future_epoch_time_write_js_client() {
    let (_, owner, contract) = deploy();
    contract.future_epoch_time_write_js_client(owner);
    let ret: U256 = contract.key_value(RESULT.to_string());

    assert_eq!(ret, 131622400.into());
}
#[test]
fn test_available_supply() {
    let (env, owner, contract) = deploy();

    TestContract::new(
        &env,
        "erc20-crv-session-code.wasm",
        "SessionCode",
        owner,
        runtime_args! {
            "entrypoint" => String::from(AVAILABLE_SUPPLY),
            "package_hash" => Key::Hash(contract.package_hash())
        },
        1000000000,
    );

    let ret: U256 = env.query_account_named_key(owner, &[AVAILABLE_SUPPLY.into()]);
    assert_eq!(ret, 130303030300u128.into());
}
#[test]
fn test_available_supply_js_client() {
    let (_, owner, contract) = deploy();
    contract.available_supply_js_client(owner);
    let ret: U256 = contract.key_value(RESULT.to_string());
    assert_eq!(ret, 130303030300u128.into());
}
#[test]
fn test_mintable_in_timeframe() {
    let (env, owner, contract) = deploy();
    contract.update_mining_parameters(owner);
    let start: U256 = 50000000.into();
    let end: U256 = 100000000.into();
    TestContract::new(
        &env,
        "erc20-crv-session-code.wasm",
        "SessionCode",
        owner,
        runtime_args! {
            "entrypoint" => String::from(MINTABLE_IN_TIMEFRAME),
            "package_hash" => Key::Hash(contract.package_hash()),
            "start"=>start,
            "end"=>end
        },
        1000000000,
    );

    let ret: U256 = env.query_account_named_key(owner, &[MINTABLE_IN_TIMEFRAME.into()]);
    assert_eq!(ret, 0.into());
}
#[test]
fn test_mintable_in_timeframe_js_client() {
    let (_, owner, contract) = deploy();
    let start: U256 = 50000000.into();
    let end: U256 = 100000000.into();
    contract.mintable_in_timeframe_js_client(owner, start, end);
    let ret: U256 = contract.key_value(RESULT.to_string());
    assert_eq!(ret, 0.into());
}
#[test]
fn test_mint() {
    let (env, owner, contract) = deploy();
    let to: Key = Key::Account(owner);
    let value: U256 = 10.into();
    let minter = Key::from(owner);
    contract.set_minter(owner, minter);
    TestContract::new(
        &env,
        "erc20-crv-session-code.wasm",
        "SessionCode",
        owner,
        runtime_args! {
            "entrypoint" => String::from(MINT),
            "package_hash" => Key::Hash(contract.package_hash()),
            "to"=>to,
            "value"=>value
        },
        1000000000,
    );

    let ret: bool = env.query_account_named_key(owner, &[MINT.into()]);
    assert_eq!(ret, true);
}
#[test]
fn test_mint_js_client() {
    let (_, owner, contract) = deploy();
    let to: Key = Key::Account(owner);
    let value: U256 = 10.into();
    let minter = Key::from(owner);
    contract.set_minter(owner, minter);
    contract.mint_js_client(owner, to, value);
    let ret: bool = contract.key_value(RESULT.to_string());
    assert_eq!(ret, true);
}
