use crate::voting_escrow_instance::VOTINGESCROWInstance;
use casper_types::{account::AccountHash, runtime_args, Key, RuntimeArgs, U128, U256};
use casperlabs_test_env::{TestContract, TestEnv};
use common::keys::*;
use voting_escrow_crate::data::WEEK;

fn deploy_erc20(env: &TestEnv, sender: AccountHash) -> TestContract {
    TestContract::new(
        env,
        "erc20-token.wasm",
        "erc20",
        sender,
        runtime_args! {
            "initial_supply" => U256::from(0),
            "name" => "Token",
            "symbol" => "ERC20",
            "decimals" => 9_u8
        },
        0,
    )
}

fn deploy() -> (TestEnv, AccountHash, VOTINGESCROWInstance, TestContract) {
    let env = TestEnv::new();
    let owner = env.next_user();
    let erc20 = deploy_erc20(&env, owner);
    let instance = VOTINGESCROWInstance::new_deploy(
        &env,
        "Voting Escrow",
        owner,
        Key::Hash(erc20.package_hash()),
        "VotingEscrow".into(),
        "VE".into(),
        "1".into(),
    );

    (env, owner, instance, erc20)
}

#[test]
fn test_deploy() {
    let (_, _, _, _) = deploy();
}

#[test]
fn test_commit_transfer_ownership() {
    let (env, owner, instance, _) = deploy();
    let addr: Key = Key::Account(env.next_user());
    instance.commit_transfer_ownership(owner, addr);
    let ret: Key = instance.key_value(FUTURE_ADMIN.to_string());
    assert_eq!(ret, addr, "Ownership not transferred");
}

#[test]
fn test_apply_transfer_ownership() {
    let (env, owner, instance, _) = deploy();
    let addr: Key = Key::Account(env.next_user());
    instance.commit_transfer_ownership(owner, addr);
    instance.apply_transfer_ownership(owner);
    let ret: Key = instance.key_value(ADMIN.to_string());
    assert_eq!(ret, addr, "Ownership transfer not applied");
}

#[test]
fn test_get_last_user_slope() {
    let (env, owner, instance, _) = deploy();
    let addr: Key = Key::Account(env.next_user());
    TestContract::new(
        &env,
        SESSION_CODE_WASM,
        SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(GET_LAST_USER_SLOPE),
            "package_hash" => Key::Hash(instance.package_hash()),
            "addr" => addr
        },
        0,
    );
    let ret: (bool, U128) = env.query_account_named_key(owner, &[GET_LAST_USER_SLOPE.into()]);
    assert_eq!(ret, (false, 0.into()), "Invalid last user scope value");
}

#[test]
fn test_user_point_history_ts() {
    let (env, owner, instance, _) = deploy();
    let addr: Key = Key::Account(env.next_user());
    let idx: U256 = 10.into();
    TestContract::new(
        &env,
        SESSION_CODE_WASM,
        SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(USER_POINT_HISTORY_TS),
            "package_hash" => Key::Hash(instance.package_hash()),
            "addr" => addr,
            "idx" => idx,
        },
        0,
    );
    let ret: U256 = env.query_account_named_key(owner, &[USER_POINT_HISTORY_TS.into()]);
    assert_eq!(ret, 0.into(), "Invalid default value");
}

#[test]
fn test_locked_end() {
    let (env, owner, instance, _) = deploy();
    let addr: Key = Key::Account(env.next_user());
    TestContract::new(
        &env,
        SESSION_CODE_WASM,
        SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(LOCKED_END),
            "package_hash" => Key::Hash(instance.package_hash()),
            "addr" => addr,
        },
        0,
    );
    let ret: U256 = env.query_account_named_key(owner, &[LOCKED_END.into()]);
    assert_eq!(ret, 0.into(), "Invalid default value");
}

#[test]
fn test_checkpoint() {
    let (_, owner, instance, _) = deploy();
    instance.checkpoint(owner);
}

#[test]
fn test_deposit_for() {
    let (_, owner, instance, erc20) = deploy();
    let addr: Key = Key::Account(owner);
    let value: U256 = 1000.into();
    let unlock_time: U256 = WEEK;
    erc20.call_contract(
        owner,
        "mint",
        runtime_args! {
            "to" => Key::Account(owner),
            "amount" => value + value
        },
        0,
    );
    erc20.call_contract(
        owner,
        "approve",
        runtime_args! {
            "spender" => Key::Hash(instance.package_hash()),
            "amount" => value + value
        },
        0,
    );
    instance.create_lock(owner, value, unlock_time);
    instance.deposit_for(owner, addr, value);
}

#[test]
fn test_create_lock() {
    let (_, owner, instance, erc20) = deploy();
    let value: U256 = 1000.into();
    let unlock_time: U256 = WEEK;
    erc20.call_contract(
        owner,
        "mint",
        runtime_args! {
            "to" => Key::Account(owner),
            "amount" => value
        },
        0,
    );
    erc20.call_contract(
        owner,
        "approve",
        runtime_args! {
            "spender" => Key::Hash(instance.package_hash()),
            "amount" => value
        },
        0,
    );
    instance.create_lock(owner, value, unlock_time);
}

#[test]
fn test_increase_amount() {
    let (_, owner, instance, erc20) = deploy();
    let value: U256 = 1000.into();
    let unlock_time: U256 = WEEK;
    erc20.call_contract(
        owner,
        "mint",
        runtime_args! {
            "to" => Key::Account(owner),
            "amount" => value + value
        },
        0,
    );
    erc20.call_contract(
        owner,
        "approve",
        runtime_args! {
            "spender" => Key::Hash(instance.package_hash()),
            "amount" => value + value
        },
        0,
    );
    instance.create_lock(owner, value, unlock_time);
    instance.increase_amount(owner, value);
}

#[test]
fn test_increase_unlock_time() {
    let (_, owner, instance, erc20) = deploy();
    let value: U256 = 1000.into();
    let unlock_time: U256 = WEEK;
    erc20.call_contract(
        owner,
        "mint",
        runtime_args! {
            "to" => Key::Account(owner),
            "amount" => value + value
        },
        0,
    );
    erc20.call_contract(
        owner,
        "approve",
        runtime_args! {
            "spender" => Key::Hash(instance.package_hash()),
            "amount" => value + value
        },
        0,
    );
    instance.create_lock(owner, value, unlock_time);
    instance.increase_unlock_time(owner, unlock_time + unlock_time);
}

#[test]
fn test_withdraw() {
    let (_, owner, instance, erc20) = deploy();
    let value: U256 = 1000.into();
    let unlock_time: U256 = WEEK;
    erc20.call_contract(
        owner,
        "mint",
        runtime_args! {
            "to" => Key::Account(owner),
            "amount" => value + value
        },
        0,
    );
    erc20.call_contract(
        owner,
        "approve",
        runtime_args! {
            "spender" => Key::Hash(instance.package_hash()),
            "amount" => value + value
        },
        0,
    );
    instance.create_lock(owner, value, unlock_time);
    instance.withdraw(owner, 1234567891099);
    let ret: U256 = instance.key_value(SUPPLY.to_string());
    assert_eq!(ret, 0.into(), "Withdrawal not done");
}

#[test]
fn test_balance_of() {
    let (env, owner, instance, _) = deploy();
    let addr: Key = Key::Account(env.next_user());
    let t: U256 = 123.into();
    TestContract::new(
        &env,
        SESSION_CODE_WASM,
        SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(BALANCE_OF),
            "package_hash" => Key::Hash(instance.package_hash()),
            "addr" => addr,
            "t" => t
        },
        0,
    );
    let ret: U256 = env.query_account_named_key(owner, &[BALANCE_OF.into()]);
    assert_eq!(ret, 0.into(), "Invalid default value");
}

#[test]
fn test_balance_of_at() {
    let (env, owner, instance, _) = deploy();
    let addr: Key = Key::Account(env.next_user());
    let time: U256 = 123.into();
    TestContract::new(
        &env,
        SESSION_CODE_WASM,
        SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(BALANCE_OF_AT),
            "package_hash" => Key::Hash(instance.package_hash()),
            "addr" => addr,
            "time" => time
        },
        123456789, // blocktime
    );
    let ret: U256 = env.query_account_named_key(owner, &[BALANCE_OF_AT.into()]);
    assert_eq!(ret, 0.into(), "Invalid default value");
}

#[test]
fn test_total_supply() {
    let (env, owner, instance, _) = deploy();
    let t: U256 = 123.into();
    TestContract::new(
        &env,
        SESSION_CODE_WASM,
        SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(TOTAL_SUPPLY),
            "package_hash" => Key::Hash(instance.package_hash()),
            "t" => t,
        },
        0,
    );
    let ret: U256 = env.query_account_named_key(owner, &[TOTAL_SUPPLY.into()]);
    assert_eq!(ret, 0.into(), "Invalid default total supply");
}

#[test]
fn test_total_supply_at() {
    let (env, owner, instance, _) = deploy();
    let time: U256 = 123.into();
    TestContract::new(
        &env,
        SESSION_CODE_WASM,
        SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(TOTAL_SUPPLY_AT),
            "package_hash" => Key::Hash(instance.package_hash()),
            "time" => time,
        },
        0,
    );
    let ret: U256 = env.query_account_named_key(owner, &[TOTAL_SUPPLY_AT.into()]);
    assert_eq!(ret, 0.into(), "Invalid default total supply");
}

#[test]
fn test_change_controller() {
    let (env, owner, instance, _) = deploy();
    let new_controller: Key = Key::Account(env.next_user());
    instance.change_controller(owner, new_controller);
    let ret: Key = instance.key_value(CONTROLLER.to_string());
    assert_eq!(ret, new_controller, "Controller not changed");
}
