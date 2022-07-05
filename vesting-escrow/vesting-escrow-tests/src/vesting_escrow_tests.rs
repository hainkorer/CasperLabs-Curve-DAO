use crate::vesting_escrow_instance::VESTINGESCROWInstance;
use casper_types::{account::AccountHash, runtime_args, Key, RuntimeArgs, U256};
use casperlabs_test_env::{TestContract, TestEnv};
use common::keys::*;

const NAME: &str = "VESTINGESCROW";

const TOKEN_NAME: &str = "ERC20";
const TOKEN_SYMBOL: &str = "ERC";
const DECIMALS: u8 = 8;
const INIT_TOTAL_SUPPLY: u64 = 0;

fn deploy() -> (
    TestEnv,
    VESTINGESCROWInstance,
    AccountHash,
    AccountHash,
    TestContract,
) {
    let env = TestEnv::new();
    let owner = env.next_user();
    let _token: TestContract = VESTINGESCROWInstance::erc20(
        &env,
        owner,
        TOKEN_NAME,
        TOKEN_SYMBOL,
        DECIMALS,
        INIT_TOTAL_SUPPLY.into(),
    );
    let _start_time: U256 = 10000.into();
    let _end_time: U256 = 10001.into();
    let _can_disable: bool = true;
    let user1 = env.next_user();
    let _fund_admins: Vec<String> = vec![
        user1.to_formatted_string(),
        env.next_user().to_formatted_string(),
        env.next_user().to_formatted_string(),
        env.next_user().to_formatted_string(),
    ];
    let token: TestContract = VESTINGESCROWInstance::new_deploy(
        &env,
        NAME,
        owner,
        Key::Hash(_token.package_hash()),
        _start_time,
        _end_time,
        _can_disable,
        _fund_admins,
    );
    (
        env,
        VESTINGESCROWInstance::instance(token),
        owner,
        user1,
        _token,
    )
}

#[test]
fn test_vesting_escrow_deploy() {
    let (env, vesting_escrow_instance, _owner, user1, token) = deploy();
    let _user = env.next_user();
    assert_eq!(
        vesting_escrow_instance.token(),
        Key::Hash(token.package_hash())
    );
    assert_eq!(vesting_escrow_instance.start_time(), 10000.into());
    assert_eq!(vesting_escrow_instance.end_time(), 10001.into());
    assert!(vesting_escrow_instance.can_disable());
    assert_eq!(vesting_escrow_instance.admin(), _owner.into());
    assert_eq!(vesting_escrow_instance.lock(), 0);
    assert!(vesting_escrow_instance.fund_admins_enabled());
    assert!(!vesting_escrow_instance.fund_admins(_owner));
    assert!(vesting_escrow_instance.fund_admins(user1));
}

#[test]
fn test_vesting_escrow_disable_fund_admins() {
    let (env, vesting_escrow_instance, _owner, user1, token) = deploy();
    let _user = env.next_user();
    assert_eq!(
        vesting_escrow_instance.token(),
        Key::Hash(token.package_hash())
    );
    assert_eq!(vesting_escrow_instance.start_time(), 10000.into());
    assert_eq!(vesting_escrow_instance.end_time(), 10001.into());
    assert!(vesting_escrow_instance.can_disable());
    assert_eq!(vesting_escrow_instance.admin(), _owner.into());
    assert_eq!(vesting_escrow_instance.lock(), 0);
    assert!(vesting_escrow_instance.fund_admins_enabled());
    assert!(!vesting_escrow_instance.fund_admins(_owner));
    assert!(vesting_escrow_instance.fund_admins(user1));
    vesting_escrow_instance.disable_fund_admins(_owner);
    assert!(!vesting_escrow_instance.fund_admins_enabled());
    assert!(vesting_escrow_instance.can_disable());
}

#[test]
fn test_vesting_escrow_disable_can_disable() {
    let (env, vesting_escrow_instance, _owner, user1, token) = deploy();
    let _user = env.next_user();
    assert_eq!(
        vesting_escrow_instance.token(),
        Key::Hash(token.package_hash())
    );
    assert_eq!(vesting_escrow_instance.start_time(), 10000.into());
    assert_eq!(vesting_escrow_instance.end_time(), 10001.into());
    assert!(vesting_escrow_instance.can_disable());
    assert_eq!(vesting_escrow_instance.admin(), _owner.into());
    assert_eq!(vesting_escrow_instance.lock(), 0);
    assert!(vesting_escrow_instance.fund_admins_enabled());
    assert!(!vesting_escrow_instance.fund_admins(_owner));
    assert!(vesting_escrow_instance.fund_admins(user1));
    vesting_escrow_instance.disable_can_disable(_owner);
    assert!(vesting_escrow_instance.fund_admins_enabled());
    assert!(!vesting_escrow_instance.can_disable());
}

#[test]
fn test_vesting_escrow_toggle_disable() {
    let (env, vesting_escrow_instance, _owner, _user1, token) = deploy();
    let _user = env.next_user();
    assert_eq!(
        vesting_escrow_instance.token(),
        Key::Hash(token.package_hash())
    );
    assert_eq!(vesting_escrow_instance.start_time(), 10000.into());
    assert_eq!(vesting_escrow_instance.end_time(), 10001.into());
    assert!(vesting_escrow_instance.can_disable());
    assert_eq!(vesting_escrow_instance.admin(), _owner.into());
    assert_eq!(vesting_escrow_instance.lock(), 0);
    assert!(vesting_escrow_instance.fund_admins_enabled());
    assert!(!vesting_escrow_instance.fund_admins(_owner));
    vesting_escrow_instance.toggle_disable(_owner, _user);
    assert_eq!(vesting_escrow_instance.disabled_at(_owner), 0.into());
    assert_eq!(vesting_escrow_instance.disabled_at(_user), 1000.into());
}

#[test]
fn test_vesting_escrow_toggle_disable_after_toggle_disable() {
    let (env, vesting_escrow_instance, _owner, _user1, token) = deploy();
    let _user = env.next_user();
    assert_eq!(
        vesting_escrow_instance.token(),
        Key::Hash(token.package_hash())
    );
    assert_eq!(vesting_escrow_instance.start_time(), 10000.into());
    assert_eq!(vesting_escrow_instance.end_time(), 10001.into());
    assert!(vesting_escrow_instance.can_disable());
    assert_eq!(vesting_escrow_instance.admin(), _owner.into());
    assert_eq!(vesting_escrow_instance.lock(), 0);
    assert!(vesting_escrow_instance.fund_admins_enabled());
    assert!(!vesting_escrow_instance.fund_admins(_owner));
    vesting_escrow_instance.toggle_disable(_owner, _user);
    assert_eq!(vesting_escrow_instance.disabled_at(_owner), 0.into());
    assert_eq!(vesting_escrow_instance.disabled_at(_user), 1000.into());
    vesting_escrow_instance.toggle_disable(_owner, _user);
    assert_eq!(vesting_escrow_instance.disabled_at(_owner), 0.into());
    assert_eq!(vesting_escrow_instance.disabled_at(_user), 0.into());
}

#[test]
#[should_panic]
fn test_vesting_escrow_toggle_disable_by_user() {
    let (env, vesting_escrow_instance, _owner, user1, token) = deploy();
    let _user = env.next_user();
    assert_eq!(
        vesting_escrow_instance.token(),
        Key::Hash(token.package_hash())
    );
    assert_eq!(vesting_escrow_instance.start_time(), 10000.into());
    assert_eq!(vesting_escrow_instance.end_time(), 10001.into());
    assert!(vesting_escrow_instance.can_disable());
    assert_eq!(vesting_escrow_instance.admin(), _owner.into());
    assert_eq!(vesting_escrow_instance.lock(), 0);
    assert!(vesting_escrow_instance.fund_admins_enabled());
    assert!(!vesting_escrow_instance.fund_admins(_owner));
    assert!(vesting_escrow_instance.fund_admins(user1));
    vesting_escrow_instance.disable_can_disable(_owner);
    assert!(vesting_escrow_instance.fund_admins_enabled());
    assert!(!vesting_escrow_instance.can_disable());
    vesting_escrow_instance.toggle_disable(_user, _user);
}

#[test]
#[should_panic]
fn test_vesting_escrow_toggle_disable_when_disabled() {
    let (env, vesting_escrow_instance, _owner, user1, token) = deploy();
    let _user = env.next_user();
    assert_eq!(
        vesting_escrow_instance.token(),
        Key::Hash(token.package_hash())
    );
    assert_eq!(vesting_escrow_instance.start_time(), 10000.into());
    assert_eq!(vesting_escrow_instance.end_time(), 10001.into());
    assert!(vesting_escrow_instance.can_disable());
    assert_eq!(vesting_escrow_instance.admin(), _owner.into());
    assert_eq!(vesting_escrow_instance.lock(), 0);
    assert!(vesting_escrow_instance.fund_admins_enabled());
    assert!(!vesting_escrow_instance.fund_admins(_owner));
    assert!(vesting_escrow_instance.fund_admins(user1));
    vesting_escrow_instance.disable_can_disable(_owner);
    assert!(vesting_escrow_instance.fund_admins_enabled());
    assert!(!vesting_escrow_instance.can_disable());
    vesting_escrow_instance.toggle_disable(_owner, _user);
}

#[test]
fn test_vesting_escrow_add_tokens() {
    let (env, vesting_escrow_instance, owner, user1, token) = deploy();
    let _user = env.next_user();
    assert_eq!(
        vesting_escrow_instance.token(),
        Key::Hash(token.package_hash())
    );
    assert_eq!(vesting_escrow_instance.start_time(), 10000.into());
    assert_eq!(vesting_escrow_instance.end_time(), 10001.into());
    assert!(vesting_escrow_instance.can_disable());
    assert_eq!(vesting_escrow_instance.admin(), owner.into());
    assert_eq!(vesting_escrow_instance.lock(), 0);
    assert!(vesting_escrow_instance.fund_admins_enabled());
    assert!(!vesting_escrow_instance.fund_admins(owner));
    assert!(vesting_escrow_instance.fund_admins(user1));
    let amount: U256 = 100.into();

    let value: U256 = 1000.into();
    token.call_contract(
        owner,
        "mint",
        runtime_args! {
            "to" => Key::Account(owner),
            "amount" => value + value
        },
        0,
    );
    token.call_contract(
        owner,
        "approve",
        runtime_args! {
            "spender" => Key::from(vesting_escrow_instance.package_hash()),
            "amount" => value + value
        },
        0,
    );
    vesting_escrow_instance.add_tokens(owner, amount);
    assert_eq!(vesting_escrow_instance.unallocated_supply(), amount);
    vesting_escrow_instance.add_tokens(owner, amount);
    assert_eq!(
        vesting_escrow_instance.unallocated_supply(),
        amount + amount
    );
}

#[test]
#[should_panic]
fn test_vesting_escrow_add_tokens_by_user() {
    let (env, vesting_escrow_instance, owner, user1, token) = deploy();
    let _user = env.next_user();
    assert_eq!(
        vesting_escrow_instance.token(),
        Key::Hash(token.package_hash())
    );
    assert_eq!(vesting_escrow_instance.start_time(), 10000.into());
    assert_eq!(vesting_escrow_instance.end_time(), 10001.into());
    assert!(vesting_escrow_instance.can_disable());
    assert_eq!(vesting_escrow_instance.admin(), owner.into());
    assert_eq!(vesting_escrow_instance.lock(), 0);
    assert!(vesting_escrow_instance.fund_admins_enabled());
    assert!(!vesting_escrow_instance.fund_admins(owner));
    assert!(vesting_escrow_instance.fund_admins(user1));
    let amount: U256 = 100.into();

    let value: U256 = 1000.into();
    token.call_contract(
        owner,
        "mint",
        runtime_args! {
            "to" => Key::Account(owner),
            "amount" => value + value
        },
        0,
    );
    token.call_contract(
        owner,
        "approve",
        runtime_args! {
            "spender" => Key::from(vesting_escrow_instance.package_hash()),
            "amount" => value + value
        },
        0,
    );
    vesting_escrow_instance.add_tokens(user1, amount);
    assert_eq!(vesting_escrow_instance.unallocated_supply(), amount);
}

#[test]
fn test_vesting_escrow_fund() {
    let (env, vesting_escrow_instance, owner, user1, token) = deploy();
    let _user = env.next_user();
    assert_eq!(
        vesting_escrow_instance.token(),
        Key::Hash(token.package_hash())
    );
    assert_eq!(vesting_escrow_instance.start_time(), 10000.into());
    assert_eq!(vesting_escrow_instance.end_time(), 10001.into());
    assert!(vesting_escrow_instance.can_disable());
    assert_eq!(vesting_escrow_instance.admin(), owner.into());
    assert_eq!(vesting_escrow_instance.lock(), 0);
    assert!(vesting_escrow_instance.fund_admins_enabled());
    assert!(!vesting_escrow_instance.fund_admins(owner));
    assert!(vesting_escrow_instance.fund_admins(user1));

    let amount: U256 = 100.into();

    let value: U256 = 1000.into();
    token.call_contract(
        owner,
        "mint",
        runtime_args! {
            "to" => Key::Account(owner),
            "amount" => value + value
        },
        0,
    );
    token.call_contract(
        owner,
        "approve",
        runtime_args! {
            "spender" => Key::from(vesting_escrow_instance.package_hash()),
            "amount" => value + value
        },
        0,
    );
    vesting_escrow_instance.add_tokens(owner, amount);
    assert_eq!(vesting_escrow_instance.unallocated_supply(), amount);
    let user_1 = env.next_user();
    let user_2 = env.next_user();
    let user_3 = env.next_user();
    let user_4 = env.next_user();
    let _recipients: Vec<String> = vec![
        user_1.to_formatted_string(),
        user_2.to_formatted_string(),
        user_3.to_formatted_string(),
        user_4.to_formatted_string(),
    ];
    let _amounts: Vec<U256> = vec![1.into(), 2.into(), 3.into(), 4.into()];
    vesting_escrow_instance.fund(owner, _recipients, _amounts);
    assert_eq!(vesting_escrow_instance.initial_locked_supply(), 10.into());
    assert_eq!(vesting_escrow_instance.unallocated_supply(), 90.into());
    assert_eq!(vesting_escrow_instance.initial_locked(user_1), 1.into());
    assert_eq!(vesting_escrow_instance.initial_locked(user_2), 2.into());
    assert_eq!(vesting_escrow_instance.initial_locked(user_3), 3.into());
    assert_eq!(vesting_escrow_instance.initial_locked(user_4), 4.into());
}

#[test]
#[should_panic]
fn test_vesting_escrow_fund_by_user() {
    let (env, vesting_escrow_instance, owner, user1, token) = deploy();
    let _user = env.next_user();
    assert_eq!(
        vesting_escrow_instance.token(),
        Key::Hash(token.package_hash())
    );
    assert_eq!(vesting_escrow_instance.start_time(), 10000.into());
    assert_eq!(vesting_escrow_instance.end_time(), 10001.into());
    assert!(vesting_escrow_instance.can_disable());
    assert_eq!(vesting_escrow_instance.admin(), owner.into());
    assert_eq!(vesting_escrow_instance.lock(), 0);
    assert!(vesting_escrow_instance.fund_admins_enabled());
    assert!(!vesting_escrow_instance.fund_admins(owner));
    assert!(vesting_escrow_instance.fund_admins(user1));

    let amount: U256 = 100.into();

    let value: U256 = 1000.into();
    token.call_contract(
        owner,
        "mint",
        runtime_args! {
            "to" => Key::Account(owner),
            "amount" => value + value
        },
        0,
    );
    token.call_contract(
        owner,
        "approve",
        runtime_args! {
            "spender" => Key::from(vesting_escrow_instance.package_hash()),
            "amount" => value + value
        },
        0,
    );
    vesting_escrow_instance.add_tokens(owner, amount);
    assert_eq!(vesting_escrow_instance.unallocated_supply(), amount);
    let user_1 = env.next_user();
    let user_2 = env.next_user();
    let user_3 = env.next_user();
    let user_4 = env.next_user();
    let _recipients: Vec<String> = vec![
        user_1.to_formatted_string(),
        user_2.to_formatted_string(),
        user_3.to_formatted_string(),
        user_4.to_formatted_string(),
    ];
    let _amounts: Vec<U256> = vec![1.into(), 2.into(), 3.into(), 4.into()];
    vesting_escrow_instance.fund(_user, _recipients, _amounts);
    assert_eq!(vesting_escrow_instance.initial_locked_supply(), 10.into());
    assert_eq!(vesting_escrow_instance.unallocated_supply(), 90.into());
    assert_eq!(vesting_escrow_instance.initial_locked(user_1), 1.into());
    assert_eq!(vesting_escrow_instance.initial_locked(user_2), 2.into());
    assert_eq!(vesting_escrow_instance.initial_locked(user_3), 3.into());
    assert_eq!(vesting_escrow_instance.initial_locked(user_4), 4.into());
}

#[test]
fn test_vesting_escrow_commit_transfer_ownership() {
    let (env, vesting_escrow_instance, owner, user1, token) = deploy();
    let _user = env.next_user();
    assert_eq!(
        vesting_escrow_instance.token(),
        Key::Hash(token.package_hash())
    );
    assert_eq!(vesting_escrow_instance.start_time(), 10000.into());
    assert_eq!(vesting_escrow_instance.end_time(), 10001.into());
    assert!(vesting_escrow_instance.can_disable());
    assert_eq!(vesting_escrow_instance.admin(), owner.into());
    assert_eq!(vesting_escrow_instance.lock(), 0);
    assert!(vesting_escrow_instance.fund_admins_enabled());
    assert!(!vesting_escrow_instance.fund_admins(owner));
    assert!(vesting_escrow_instance.fund_admins(user1));
    TestContract::new(
        &env,
        "vesting-escrow-session-code.wasm",
        "SessionCode",
        owner,
        runtime_args! {
            "entrypoint" => String::from(COMMIT_TRANSFER_OWNERSHIP),
            "package_hash" => Key::from(vesting_escrow_instance.package_hash()),
            "addr"=>Key::from(_user)
        },
        1000,
    );

    let ret: bool = env.query_account_named_key(owner, &[COMMIT_TRANSFER_OWNERSHIP.into()]);
    assert!(ret);
    assert_eq!(vesting_escrow_instance.future_admin(), Key::from(_user));
}

#[test]
fn test_vesting_escrow_apply_transfer_ownership() {
    let (env, vesting_escrow_instance, owner, user1, token) = deploy();
    let _user = env.next_user();
    assert_eq!(
        vesting_escrow_instance.token(),
        Key::Hash(token.package_hash())
    );
    assert_eq!(vesting_escrow_instance.start_time(), 10000.into());
    assert_eq!(vesting_escrow_instance.end_time(), 10001.into());
    assert!(vesting_escrow_instance.can_disable());
    assert_eq!(vesting_escrow_instance.admin(), owner.into());
    assert_eq!(vesting_escrow_instance.lock(), 0);
    assert!(vesting_escrow_instance.fund_admins_enabled());
    assert!(!vesting_escrow_instance.fund_admins(owner));
    assert!(vesting_escrow_instance.fund_admins(user1));

    TestContract::new(
        &env,
        "vesting-escrow-session-code.wasm",
        "SessionCode",
        owner,
        runtime_args! {
            "entrypoint" => String::from(COMMIT_TRANSFER_OWNERSHIP),
            "package_hash" => Key::from(vesting_escrow_instance.package_hash()),
            "addr"=>Key::from(_user)
        },
        1000,
    );

    let ret: bool = env.query_account_named_key(owner, &[COMMIT_TRANSFER_OWNERSHIP.into()]);
    assert!(ret);
    assert_eq!(vesting_escrow_instance.future_admin(), Key::from(_user));

    TestContract::new(
        &env,
        "vesting-escrow-session-code.wasm",
        "SessionCode",
        owner,
        runtime_args! {
            "entrypoint" => String::from(APPLY_TRANSFER_OWNERSHIP),
            "package_hash" => Key::from(vesting_escrow_instance.package_hash()),
            "addr"=>Key::from(_user)
        },
        1000,
    );

    let ret: bool = env.query_account_named_key(owner, &[APPLY_TRANSFER_OWNERSHIP.into()]);
    assert!(ret);
    assert_eq!(vesting_escrow_instance.future_admin(), Key::from(_user));
    assert_eq!(vesting_escrow_instance.admin(), Key::from(_user));
}

#[test]
fn test_vesting_escrow_vested_supply() {
    let (env, vesting_escrow_instance, owner, user1, token) = deploy();
    let _user = env.next_user();
    assert_eq!(
        vesting_escrow_instance.token(),
        Key::Hash(token.package_hash())
    );
    assert_eq!(vesting_escrow_instance.start_time(), 10000.into());
    assert_eq!(vesting_escrow_instance.end_time(), 10001.into());
    assert!(vesting_escrow_instance.can_disable());
    assert_eq!(vesting_escrow_instance.admin(), owner.into());
    assert_eq!(vesting_escrow_instance.lock(), 0);
    assert!(vesting_escrow_instance.fund_admins_enabled());
    assert!(!vesting_escrow_instance.fund_admins(owner));
    assert!(vesting_escrow_instance.fund_admins(user1));

    let amount: U256 = 100.into();

    let value: U256 = 1000.into();
    token.call_contract(
        owner,
        "mint",
        runtime_args! {
            "to" => Key::Account(owner),
            "amount" => value + value
        },
        0,
    );
    token.call_contract(
        owner,
        "approve",
        runtime_args! {
            "spender" => Key::from(vesting_escrow_instance.package_hash()),
            "amount" => value + value
        },
        0,
    );
    vesting_escrow_instance.add_tokens(owner, amount);
    assert_eq!(vesting_escrow_instance.unallocated_supply(), amount);
    let user_1 = env.next_user();
    let user_2 = env.next_user();
    let user_3 = env.next_user();
    let user_4 = env.next_user();
    let _recipients: Vec<String> = vec![
        user_1.to_formatted_string(),
        user_2.to_formatted_string(),
        user_3.to_formatted_string(),
        user_4.to_formatted_string(),
    ];
    let _amounts: Vec<U256> = vec![1.into(), 2.into(), 3.into(), 4.into()];
    vesting_escrow_instance.fund(owner, _recipients, _amounts);
    assert_eq!(vesting_escrow_instance.initial_locked_supply(), 10.into());
    assert_eq!(vesting_escrow_instance.unallocated_supply(), 90.into());
    assert_eq!(vesting_escrow_instance.initial_locked(user_1), 1.into());
    assert_eq!(vesting_escrow_instance.initial_locked(user_2), 2.into());
    assert_eq!(vesting_escrow_instance.initial_locked(user_3), 3.into());
    assert_eq!(vesting_escrow_instance.initial_locked(user_4), 4.into());

    TestContract::new(
        &env,
        "vesting-escrow-session-code.wasm",
        "SessionCode",
        owner,
        runtime_args! {
            "entrypoint" => String::from(VESTED_SUPPLY),
            "package_hash" => Key::from(vesting_escrow_instance.package_hash()),
        },
        1000000,
    );

    let ret: U256 = env.query_account_named_key(owner, &[VESTED_SUPPLY.into()]);
    assert_eq!(ret, 10.into());
}

#[test]
fn test_vesting_escrow_locked_supply() {
    let (env, vesting_escrow_instance, owner, user1, token) = deploy();
    let _user = env.next_user();
    assert_eq!(
        vesting_escrow_instance.token(),
        Key::Hash(token.package_hash())
    );
    assert_eq!(vesting_escrow_instance.start_time(), 10000.into());
    assert_eq!(vesting_escrow_instance.end_time(), 10001.into());
    assert!(vesting_escrow_instance.can_disable());
    assert_eq!(vesting_escrow_instance.admin(), owner.into());
    assert_eq!(vesting_escrow_instance.lock(), 0);
    assert!(vesting_escrow_instance.fund_admins_enabled());
    assert!(!vesting_escrow_instance.fund_admins(owner));
    assert!(vesting_escrow_instance.fund_admins(user1));

    let amount: U256 = 1000.into();

    let value: U256 = 1000.into();
    token.call_contract(
        owner,
        "mint",
        runtime_args! {
            "to" => Key::Account(owner),
            "amount" => value + value
        },
        0,
    );
    token.call_contract(
        owner,
        "approve",
        runtime_args! {
            "spender" => Key::from(vesting_escrow_instance.package_hash()),
            "amount" => value + value
        },
        0,
    );
    vesting_escrow_instance.add_tokens(owner, amount);
    assert_eq!(vesting_escrow_instance.unallocated_supply(), amount);
    let user_1 = env.next_user();
    let user_2 = env.next_user();
    let user_3 = env.next_user();
    let user_4 = env.next_user();
    let _recipients: Vec<String> = vec![
        user_1.to_formatted_string(),
        user_2.to_formatted_string(),
        user_3.to_formatted_string(),
        user_4.to_formatted_string(),
    ];
    let _amounts: Vec<U256> = vec![2.into(), 3.into(), 4.into(), 5.into()];
    vesting_escrow_instance.fund(owner, _recipients, _amounts);
    assert_eq!(vesting_escrow_instance.initial_locked_supply(), 14.into());
    assert_eq!(vesting_escrow_instance.unallocated_supply(), 986.into());
    assert_eq!(vesting_escrow_instance.initial_locked(user_1), 2.into());
    assert_eq!(vesting_escrow_instance.initial_locked(user_2), 3.into());
    assert_eq!(vesting_escrow_instance.initial_locked(user_3), 4.into());
    assert_eq!(vesting_escrow_instance.initial_locked(user_4), 5.into());

    TestContract::new(
        &env,
        "vesting-escrow-session-code.wasm",
        "SessionCode",
        owner,
        runtime_args! {
            "entrypoint" => String::from(LOCKED_SUPPLY),
            "package_hash" => Key::from(vesting_escrow_instance.package_hash()),
        },
        1000000,
    );

    let ret: U256 = env.query_account_named_key(owner, &[LOCKED_SUPPLY.into()]);
    assert_eq!(ret, 0.into());
}

#[test]
fn test_vesting_escrow_vested_of() {
    let (env, vesting_escrow_instance, owner, user1, token) = deploy();
    let _user = env.next_user();
    assert_eq!(
        vesting_escrow_instance.token(),
        Key::Hash(token.package_hash())
    );
    assert_eq!(vesting_escrow_instance.start_time(), 10000.into());
    assert_eq!(vesting_escrow_instance.end_time(), 10001.into());
    assert!(vesting_escrow_instance.can_disable());
    assert_eq!(vesting_escrow_instance.admin(), owner.into());
    assert_eq!(vesting_escrow_instance.lock(), 0);
    assert!(vesting_escrow_instance.fund_admins_enabled());
    assert!(!vesting_escrow_instance.fund_admins(owner));
    assert!(vesting_escrow_instance.fund_admins(user1));

    let amount: U256 = 1000.into();

    let value: U256 = 1000.into();
    token.call_contract(
        owner,
        "mint",
        runtime_args! {
            "to" => Key::Account(owner),
            "amount" => value + value
        },
        0,
    );
    token.call_contract(
        owner,
        "approve",
        runtime_args! {
            "spender" => Key::from(vesting_escrow_instance.package_hash()),
            "amount" => value + value
        },
        0,
    );
    vesting_escrow_instance.add_tokens(owner, amount);
    assert_eq!(vesting_escrow_instance.unallocated_supply(), amount);
    let user_1 = env.next_user();
    let user_2 = env.next_user();
    let user_3 = env.next_user();
    let user_4 = env.next_user();
    let _recipients: Vec<String> = vec![
        user_1.to_formatted_string(),
        user_2.to_formatted_string(),
        user_3.to_formatted_string(),
        user_4.to_formatted_string(),
    ];
    let _amounts: Vec<U256> = vec![2.into(), 3.into(), 4.into(), 5.into()];
    vesting_escrow_instance.fund(owner, _recipients, _amounts);
    assert_eq!(vesting_escrow_instance.initial_locked_supply(), 14.into());
    assert_eq!(vesting_escrow_instance.unallocated_supply(), 986.into());
    assert_eq!(vesting_escrow_instance.initial_locked(user_1), 2.into());
    assert_eq!(vesting_escrow_instance.initial_locked(user_2), 3.into());
    assert_eq!(vesting_escrow_instance.initial_locked(user_3), 4.into());
    assert_eq!(vesting_escrow_instance.initial_locked(user_4), 5.into());

    TestContract::new(
        &env,
        "vesting-escrow-session-code.wasm",
        "SessionCode",
        owner,
        runtime_args! {
            "entrypoint" => String::from(VESTED_OF),
            "package_hash" => Key::from(vesting_escrow_instance.package_hash()),
            "_recipient" => Key::Account(user_1),
        },
        1000000,
    );

    let ret: U256 = env.query_account_named_key(owner, &[VESTED_OF.into()]);
    assert_eq!(ret, 2.into());
    TestContract::new(
        &env,
        "vesting-escrow-session-code.wasm",
        "SessionCode",
        owner,
        runtime_args! {
            "entrypoint" => String::from(VESTED_OF),
            "package_hash" => Key::from(vesting_escrow_instance.package_hash()),
            "_recipient" => Key::Account(user_2),
        },
        1000000,
    );

    let ret: U256 = env.query_account_named_key(owner, &[VESTED_OF.into()]);
    assert_eq!(ret, 3.into());
    TestContract::new(
        &env,
        "vesting-escrow-session-code.wasm",
        "SessionCode",
        owner,
        runtime_args! {
            "entrypoint" => String::from(VESTED_OF),
            "package_hash" => Key::from(vesting_escrow_instance.package_hash()),
            "_recipient" => Key::Account(user_3),
        },
        1000000,
    );

    let ret: U256 = env.query_account_named_key(owner, &[VESTED_OF.into()]);
    assert_eq!(ret, 4.into());
    TestContract::new(
        &env,
        "vesting-escrow-session-code.wasm",
        "SessionCode",
        owner,
        runtime_args! {
            "entrypoint" => String::from(VESTED_OF),
            "package_hash" => Key::from(vesting_escrow_instance.package_hash()),
            "_recipient" => Key::Account(user_4),
        },
        1000000,
    );

    let ret: U256 = env.query_account_named_key(owner, &[VESTED_OF.into()]);
    assert_eq!(ret, 5.into());
}

#[test]
fn test_vesting_escrow_balance_of() {
    let (env, vesting_escrow_instance, owner, user1, token) = deploy();
    let _user = env.next_user();
    assert_eq!(
        vesting_escrow_instance.token(),
        Key::Hash(token.package_hash())
    );
    assert_eq!(vesting_escrow_instance.start_time(), 10000.into());
    assert_eq!(vesting_escrow_instance.end_time(), 10001.into());
    assert!(vesting_escrow_instance.can_disable());
    assert_eq!(vesting_escrow_instance.admin(), owner.into());
    assert_eq!(vesting_escrow_instance.lock(), 0);
    assert!(vesting_escrow_instance.fund_admins_enabled());
    assert!(!vesting_escrow_instance.fund_admins(owner));
    assert!(vesting_escrow_instance.fund_admins(user1));

    let amount: U256 = 1000.into();

    let value: U256 = 1000.into();
    token.call_contract(
        owner,
        "mint",
        runtime_args! {
            "to" => Key::Account(owner),
            "amount" => value + value
        },
        0,
    );
    token.call_contract(
        owner,
        "approve",
        runtime_args! {
            "spender" => Key::from(vesting_escrow_instance.package_hash()),
            "amount" => value + value
        },
        0,
    );
    vesting_escrow_instance.add_tokens(owner, amount);
    assert_eq!(vesting_escrow_instance.unallocated_supply(), amount);
    let user_1 = env.next_user();
    let user_2 = env.next_user();
    let user_3 = env.next_user();
    let user_4 = env.next_user();
    let _recipients: Vec<String> = vec![
        user_1.to_formatted_string(),
        user_2.to_formatted_string(),
        user_3.to_formatted_string(),
        user_4.to_formatted_string(),
    ];
    let _amounts: Vec<U256> = vec![2.into(), 3.into(), 4.into(), 5.into()];
    vesting_escrow_instance.fund(owner, _recipients, _amounts);
    assert_eq!(vesting_escrow_instance.initial_locked_supply(), 14.into());
    assert_eq!(vesting_escrow_instance.unallocated_supply(), 986.into());
    assert_eq!(vesting_escrow_instance.initial_locked(user_1), 2.into());
    assert_eq!(vesting_escrow_instance.initial_locked(user_2), 3.into());
    assert_eq!(vesting_escrow_instance.initial_locked(user_3), 4.into());
    assert_eq!(vesting_escrow_instance.initial_locked(user_4), 5.into());

    TestContract::new(
        &env,
        "vesting-escrow-session-code.wasm",
        "SessionCode",
        owner,
        runtime_args! {
            "entrypoint" => String::from(BALANCE_OF),
            "package_hash" => Key::from(vesting_escrow_instance.package_hash()),
            "_recipient" => Key::Account(user_1),
        },
        1000000,
    );

    let ret: U256 = env.query_account_named_key(owner, &[BALANCE_OF.into()]);
    assert_eq!(ret, 2.into());
    TestContract::new(
        &env,
        "vesting-escrow-session-code.wasm",
        "SessionCode",
        owner,
        runtime_args! {
            "entrypoint" => String::from(BALANCE_OF),
            "package_hash" => Key::from(vesting_escrow_instance.package_hash()),
            "_recipient" => Key::Account(user_2),
        },
        1000000,
    );

    let ret: U256 = env.query_account_named_key(owner, &[BALANCE_OF.into()]);
    assert_eq!(ret, 3.into());
    TestContract::new(
        &env,
        "vesting-escrow-session-code.wasm",
        "SessionCode",
        owner,
        runtime_args! {
            "entrypoint" => String::from(BALANCE_OF),
            "package_hash" => Key::from(vesting_escrow_instance.package_hash()),
            "_recipient" => Key::Account(user_3),
        },
        1000000,
    );

    let ret: U256 = env.query_account_named_key(owner, &[BALANCE_OF.into()]);
    assert_eq!(ret, 4.into());
    TestContract::new(
        &env,
        "vesting-escrow-session-code.wasm",
        "SessionCode",
        owner,
        runtime_args! {
            "entrypoint" => String::from(BALANCE_OF),
            "package_hash" => Key::from(vesting_escrow_instance.package_hash()),
            "_recipient" => Key::Account(user_4),
        },
        1000000,
    );

    let ret: U256 = env.query_account_named_key(owner, &[BALANCE_OF.into()]);
    assert_eq!(ret, 5.into());
}

#[test]
fn test_vesting_escrow_locked_of() {
    let (env, vesting_escrow_instance, owner, user1, token) = deploy();
    let _user = env.next_user();
    assert_eq!(
        vesting_escrow_instance.token(),
        Key::Hash(token.package_hash())
    );
    assert_eq!(vesting_escrow_instance.start_time(), 10000.into());
    assert_eq!(vesting_escrow_instance.end_time(), 10001.into());
    assert!(vesting_escrow_instance.can_disable());
    assert_eq!(vesting_escrow_instance.admin(), owner.into());
    assert_eq!(vesting_escrow_instance.lock(), 0);
    assert!(vesting_escrow_instance.fund_admins_enabled());
    assert!(!vesting_escrow_instance.fund_admins(owner));
    assert!(vesting_escrow_instance.fund_admins(user1));

    let amount: U256 = 1000.into();

    let value: U256 = 1000.into();
    token.call_contract(
        owner,
        "mint",
        runtime_args! {
            "to" => Key::Account(owner),
            "amount" => value + value
        },
        0,
    );
    token.call_contract(
        owner,
        "approve",
        runtime_args! {
            "spender" => Key::from(vesting_escrow_instance.package_hash()),
            "amount" => value + value
        },
        0,
    );
    vesting_escrow_instance.add_tokens(owner, amount);
    assert_eq!(vesting_escrow_instance.unallocated_supply(), amount);
    let user_1 = env.next_user();
    let user_2 = env.next_user();
    let user_3 = env.next_user();
    let user_4 = env.next_user();
    let _recipients: Vec<String> = vec![
        user_1.to_formatted_string(),
        user_2.to_formatted_string(),
        user_3.to_formatted_string(),
        user_4.to_formatted_string(),
    ];
    let _amounts: Vec<U256> = vec![2.into(), 3.into(), 4.into(), 5.into()];
    vesting_escrow_instance.fund(owner, _recipients, _amounts);
    assert_eq!(vesting_escrow_instance.initial_locked_supply(), 14.into());
    assert_eq!(vesting_escrow_instance.unallocated_supply(), 986.into());
    assert_eq!(vesting_escrow_instance.initial_locked(user_1), 2.into());
    assert_eq!(vesting_escrow_instance.initial_locked(user_2), 3.into());
    assert_eq!(vesting_escrow_instance.initial_locked(user_3), 4.into());
    assert_eq!(vesting_escrow_instance.initial_locked(user_4), 5.into());

    TestContract::new(
        &env,
        "vesting-escrow-session-code.wasm",
        "SessionCode",
        owner,
        runtime_args! {
            "entrypoint" => String::from(LOCKED_OF),
            "package_hash" => Key::from(vesting_escrow_instance.package_hash()),
            "_recipient" => Key::Account(user_1),
        },
        1000000,
    );

    let ret: U256 = env.query_account_named_key(owner, &[LOCKED_OF.into()]);
    assert_eq!(ret, 0.into());
    TestContract::new(
        &env,
        "vesting-escrow-session-code.wasm",
        "SessionCode",
        owner,
        runtime_args! {
            "entrypoint" => String::from(LOCKED_OF),
            "package_hash" => Key::from(vesting_escrow_instance.package_hash()),
            "_recipient" => Key::Account(user_2),
        },
        1000000,
    );

    let ret: U256 = env.query_account_named_key(owner, &[LOCKED_OF.into()]);
    assert_eq!(ret, 0.into());
    TestContract::new(
        &env,
        "vesting-escrow-session-code.wasm",
        "SessionCode",
        owner,
        runtime_args! {
            "entrypoint" => String::from(LOCKED_OF),
            "package_hash" => Key::from(vesting_escrow_instance.package_hash()),
            "_recipient" => Key::Account(user_3),
        },
        1000000,
    );

    let ret: U256 = env.query_account_named_key(owner, &[LOCKED_OF.into()]);
    assert_eq!(ret, 0.into());
    TestContract::new(
        &env,
        "vesting-escrow-session-code.wasm",
        "SessionCode",
        owner,
        runtime_args! {
            "entrypoint" => String::from(LOCKED_OF),
            "package_hash" => Key::from(vesting_escrow_instance.package_hash()),
            "_recipient" => Key::Account(user_4),
        },
        1000000,
    );

    let ret: U256 = env.query_account_named_key(owner, &[LOCKED_OF.into()]);
    assert_eq!(ret, 0.into());
}
