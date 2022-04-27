use crate::voting_escrow_instance::VOTINGESCROWInstance;
use casper_types::{account::AccountHash, runtime_args, Key, RuntimeArgs, U128, U256};
use test_env::{TestContract, TestEnv};
use voting_escrow_crate::data::*;

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
            "decimals" => 9 as u8
        },
        0,
    )
}

fn deploy() -> (TestEnv, AccountHash, VOTINGESCROWInstance, TestContract) {
    let env = TestEnv::new();
    let owner = env.next_user();
    let erc20 = deploy_erc20(&env, owner);
    let instance = VOTINGESCROWInstance::new(
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
fn test_commit_smart_wallet_checker() {
    let (env, owner, instance, _) = deploy();
    let addr: Key = Key::Account(env.next_user());
    instance.commit_smart_wallet_checker(owner, addr);
    let ret: Key = instance.key_value(FUTURE_SMART_WALLET_CHECKER.to_string());
    assert_eq!(ret, addr, "Invalid Smart contract checker");
}

#[test]
fn test_apply_smart_wallet_checker() {
    let (_, owner, instance, _) = deploy();
    instance.apply_smart_wallet_checker(owner);
    let ret: Key = instance.key_value(SMART_WALLET_CHECKER.to_string());
    assert_eq!(ret, zero_address(), "Invalid Smart contract checker");
}

#[test]
fn test_get_last_user_slope_js_client() {
    let (env, owner, instance, _) = deploy();
    let addr: Key = Key::Account(env.next_user());
    instance.get_last_user_slope_js_client(owner, addr);
    let ret: U128 = instance.key_value(RESULT.to_string());
    assert_eq!(ret, 0.into(), "Invalid default value");
}

#[test]
fn test_user_point_history_ts_js_client() {
    let (env, owner, instance, _) = deploy();
    let addr: Key = Key::Account(env.next_user());
    let idx: U256 = 10.into();
    instance.user_point_history_ts_js_client(owner, addr, idx);
    let ret: U256 = instance.key_value(RESULT.to_string());
    assert_eq!(ret, 0.into(), "Invalid default value");
}

#[test]
fn test_locked_end_js_client() {
    let (env, owner, instance, _) = deploy();
    let addr: Key = Key::Account(env.next_user());
    instance.locked_end_js_client(owner, addr);
    let ret: U256 = instance.key_value(RESULT.to_string());
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
fn test_balance_of_js_client() {
    let (env, owner, instance, _) = deploy();
    let addr: Key = Key::Account(env.next_user());
    let t: U256 = 123.into();
    instance.balance_of_js_client(owner, addr, t);
    let ret: U256 = instance.key_value(RESULT.to_string());
    assert_eq!(ret, 0.into(), "Invalid default balance");
}

#[test]
fn test_balance_of_at_js_client() {
    let (env, owner, instance, _) = deploy();
    let addr: Key = Key::Account(env.next_user());
    let block: U256 = 123.into();
    instance.balance_of_at_js_client(owner, addr, block);
    let ret: U256 = instance.key_value(RESULT.to_string());
    assert_eq!(ret, 0.into(), "Invalid default balance");
}

#[test]
fn test_total_supply_js_client() {
    let (_, owner, instance, _) = deploy();
    let t: U256 = 123.into();
    instance.total_supply_js_client(owner, t);
    let ret: U256 = instance.key_value(RESULT.to_string());
    assert_eq!(ret, 0.into(), "Invalid default total supply");
}

#[test]
fn test_total_supply_at_js_client() {
    let (_, owner, instance, _) = deploy();
    let block: U256 = 123.into();
    instance.total_supply_at_js_client(owner, block);
    let ret: U256 = instance.key_value(RESULT.to_string());
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
