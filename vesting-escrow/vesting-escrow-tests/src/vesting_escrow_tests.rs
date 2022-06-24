use casper_types::{account::AccountHash, runtime_args, Key, RuntimeArgs, U256};
use casperlabs_test_env::{TestContract, TestEnv};

use crate::vesting_escrow_instance::{self, VESTINGESCROWInstance};

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
    TestContract, // VESTINGESCROWInstance,
                  // VESTINGESCROWInstance,
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
    let _end_time: U256 = 10000.into();
    let _can_disable: bool = true;
    let user1 = env.next_user();
    let _fund_admins: Vec<String> = vec![
        user1.to_formatted_string(),
        env.next_user().to_formatted_string(),
        env.next_user().to_formatted_string(),
        env.next_user().to_formatted_string(),
    ];
    let token: TestContract = VESTINGESCROWInstance::new(
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
fn test_deploy() {
    let (env, vesting_escrow_instance, _owner, user1, token) = deploy();
    let _user = env.next_user();
    assert_eq!(
        vesting_escrow_instance.token(),
        Key::Hash(token.package_hash())
    );
    assert_eq!(vesting_escrow_instance.start_time(), 10000.into());
    assert_eq!(vesting_escrow_instance.end_time(), 10000.into());
    assert_eq!(vesting_escrow_instance.can_disable(), true);
    assert_eq!(vesting_escrow_instance.admin(), _owner.into());
    assert_eq!(vesting_escrow_instance.lock(), 0);
    assert_eq!(vesting_escrow_instance.fund_admins_enabled(), true);
    assert_eq!(vesting_escrow_instance.fund_admins(_owner), false);
    assert_eq!(vesting_escrow_instance.fund_admins(user1), true);
}

#[test]
fn test_disable_fund_admins() {
    let (env, vesting_escrow_instance, _owner, user1, token) = deploy();
    let _user = env.next_user();
    assert_eq!(
        vesting_escrow_instance.token(),
        Key::Hash(token.package_hash())
    );
    assert_eq!(vesting_escrow_instance.start_time(), 10000.into());
    assert_eq!(vesting_escrow_instance.end_time(), 10000.into());
    assert_eq!(vesting_escrow_instance.can_disable(), true);
    assert_eq!(vesting_escrow_instance.admin(), _owner.into());
    assert_eq!(vesting_escrow_instance.lock(), 0);
    assert_eq!(vesting_escrow_instance.fund_admins_enabled(), true);
    assert_eq!(vesting_escrow_instance.fund_admins(_owner), false);
    assert_eq!(vesting_escrow_instance.fund_admins(user1), true);
    vesting_escrow_instance.disable_fund_admins(_owner);
    assert_eq!(vesting_escrow_instance.fund_admins_enabled(), false);
    assert_eq!(vesting_escrow_instance.can_disable(), true);
}

#[test]
fn test_disable_can_disable() {
    let (env, vesting_escrow_instance, _owner, user1, token) = deploy();
    let _user = env.next_user();
    assert_eq!(
        vesting_escrow_instance.token(),
        Key::Hash(token.package_hash())
    );
    assert_eq!(vesting_escrow_instance.start_time(), 10000.into());
    assert_eq!(vesting_escrow_instance.end_time(), 10000.into());
    assert_eq!(vesting_escrow_instance.can_disable(), true);
    assert_eq!(vesting_escrow_instance.admin(), _owner.into());
    assert_eq!(vesting_escrow_instance.lock(), 0);
    assert_eq!(vesting_escrow_instance.fund_admins_enabled(), true);
    assert_eq!(vesting_escrow_instance.fund_admins(_owner), false);
    assert_eq!(vesting_escrow_instance.fund_admins(user1), true);
    vesting_escrow_instance.disable_can_disable(_owner);
    assert_eq!(vesting_escrow_instance.fund_admins_enabled(), true);
    assert_eq!(vesting_escrow_instance.can_disable(), false);
}

#[test]
fn test_toggle_disable() {
    let (env, vesting_escrow_instance, _owner, user1, token) = deploy();
    let _user = env.next_user();
    assert_eq!(
        vesting_escrow_instance.token(),
        Key::Hash(token.package_hash())
    );
    assert_eq!(vesting_escrow_instance.start_time(), 10000.into());
    assert_eq!(vesting_escrow_instance.end_time(), 10000.into());
    assert_eq!(vesting_escrow_instance.can_disable(), true);
    assert_eq!(vesting_escrow_instance.admin(), _owner.into());
    assert_eq!(vesting_escrow_instance.lock(), 0);
    assert_eq!(vesting_escrow_instance.fund_admins_enabled(), true);
    assert_eq!(vesting_escrow_instance.fund_admins(_owner), false);
    vesting_escrow_instance.toggle_disable(_owner, _user);
    assert_eq!(vesting_escrow_instance.disabled_at(_owner), 0.into());
    assert_eq!(vesting_escrow_instance.disabled_at(_user), 1000.into());
}

#[test]
fn test_toggle_disable_after_toggle_disable() {
    let (env, vesting_escrow_instance, _owner, user1, token) = deploy();
    let _user = env.next_user();
    assert_eq!(
        vesting_escrow_instance.token(),
        Key::Hash(token.package_hash())
    );
    assert_eq!(vesting_escrow_instance.start_time(), 10000.into());
    assert_eq!(vesting_escrow_instance.end_time(), 10000.into());
    assert_eq!(vesting_escrow_instance.can_disable(), true);
    assert_eq!(vesting_escrow_instance.admin(), _owner.into());
    assert_eq!(vesting_escrow_instance.lock(), 0);
    assert_eq!(vesting_escrow_instance.fund_admins_enabled(), true);
    assert_eq!(vesting_escrow_instance.fund_admins(_owner), false);
    vesting_escrow_instance.toggle_disable(_owner, _user);
    assert_eq!(vesting_escrow_instance.disabled_at(_owner), 0.into());
    assert_eq!(vesting_escrow_instance.disabled_at(_user), 1000.into());
    vesting_escrow_instance.toggle_disable(_owner, _user);
    assert_eq!(vesting_escrow_instance.disabled_at(_owner), 0.into());
    assert_eq!(vesting_escrow_instance.disabled_at(_user), 0.into());
}

#[test]
#[should_panic]
fn test_toggle_disable_by_user() {
    let (env, vesting_escrow_instance, _owner, user1, token) = deploy();
    let _user = env.next_user();
    assert_eq!(
        vesting_escrow_instance.token(),
        Key::Hash(token.package_hash())
    );
    assert_eq!(vesting_escrow_instance.start_time(), 10000.into());
    assert_eq!(vesting_escrow_instance.end_time(), 10000.into());
    assert_eq!(vesting_escrow_instance.can_disable(), true);
    assert_eq!(vesting_escrow_instance.admin(), _owner.into());
    assert_eq!(vesting_escrow_instance.lock(), 0);
    assert_eq!(vesting_escrow_instance.fund_admins_enabled(), true);
    assert_eq!(vesting_escrow_instance.fund_admins(_owner), false);
    assert_eq!(vesting_escrow_instance.fund_admins(user1), true);
    vesting_escrow_instance.disable_can_disable(_owner);
    assert_eq!(vesting_escrow_instance.fund_admins_enabled(), true);
    assert_eq!(vesting_escrow_instance.can_disable(), false);
    vesting_escrow_instance.toggle_disable(_user, _user);
}

#[test]
#[should_panic]
fn test_toggle_disable_when_disabled() {
    let (env, vesting_escrow_instance, _owner, user1, token) = deploy();
    let _user = env.next_user();
    assert_eq!(
        vesting_escrow_instance.token(),
        Key::Hash(token.package_hash())
    );
    assert_eq!(vesting_escrow_instance.start_time(), 10000.into());
    assert_eq!(vesting_escrow_instance.end_time(), 10000.into());
    assert_eq!(vesting_escrow_instance.can_disable(), true);
    assert_eq!(vesting_escrow_instance.admin(), _owner.into());
    assert_eq!(vesting_escrow_instance.lock(), 0);
    assert_eq!(vesting_escrow_instance.fund_admins_enabled(), true);
    assert_eq!(vesting_escrow_instance.fund_admins(_owner), false);
    assert_eq!(vesting_escrow_instance.fund_admins(user1), true);
    vesting_escrow_instance.disable_can_disable(_owner);
    assert_eq!(vesting_escrow_instance.fund_admins_enabled(), true);
    assert_eq!(vesting_escrow_instance.can_disable(), false);
    vesting_escrow_instance.toggle_disable(_owner, _user);
}

#[test]
fn test_add_tokens() {
    let (env, vesting_escrow_instance, owner, user1, token) = deploy();
    let _user = env.next_user();
    assert_eq!(
        vesting_escrow_instance.token(),
        Key::Hash(token.package_hash())
    );
    assert_eq!(vesting_escrow_instance.start_time(), 10000.into());
    assert_eq!(vesting_escrow_instance.end_time(), 10000.into());
    assert_eq!(vesting_escrow_instance.can_disable(), true);
    assert_eq!(vesting_escrow_instance.admin(), owner.into());
    assert_eq!(vesting_escrow_instance.lock(), 0);
    assert_eq!(vesting_escrow_instance.fund_admins_enabled(), true);
    assert_eq!(vesting_escrow_instance.fund_admins(owner), false);
    assert_eq!(vesting_escrow_instance.fund_admins(user1), true);
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
fn test_add_tokens_by_user() {
    let (env, vesting_escrow_instance, owner, user1, token) = deploy();
    let _user = env.next_user();
    assert_eq!(
        vesting_escrow_instance.token(),
        Key::Hash(token.package_hash())
    );
    assert_eq!(vesting_escrow_instance.start_time(), 10000.into());
    assert_eq!(vesting_escrow_instance.end_time(), 10000.into());
    assert_eq!(vesting_escrow_instance.can_disable(), true);
    assert_eq!(vesting_escrow_instance.admin(), owner.into());
    assert_eq!(vesting_escrow_instance.lock(), 0);
    assert_eq!(vesting_escrow_instance.fund_admins_enabled(), true);
    assert_eq!(vesting_escrow_instance.fund_admins(owner), false);
    assert_eq!(vesting_escrow_instance.fund_admins(user1), true);
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
fn test_fund() {
    let (env, vesting_escrow_instance, owner, user1, token) = deploy();
    let _user = env.next_user();
    assert_eq!(
        vesting_escrow_instance.token(),
        Key::Hash(token.package_hash())
    );
    assert_eq!(vesting_escrow_instance.start_time(), 10000.into());
    assert_eq!(vesting_escrow_instance.end_time(), 10000.into());
    assert_eq!(vesting_escrow_instance.can_disable(), true);
    assert_eq!(vesting_escrow_instance.admin(), owner.into());
    assert_eq!(vesting_escrow_instance.lock(), 0);
    assert_eq!(vesting_escrow_instance.fund_admins_enabled(), true);
    assert_eq!(vesting_escrow_instance.fund_admins(owner), false);
    assert_eq!(vesting_escrow_instance.fund_admins(user1), true);
    
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
    let user_1=env.next_user();
    let user_2=env.next_user();
    let user_3=env.next_user();
    let user_4=env.next_user();
    let _recipients: Vec<String> = vec![
        user_1.to_formatted_string(),
        user_2.to_formatted_string(),
        user_3.to_formatted_string(),
        user_4.to_formatted_string(),
    ];
    let _amounts: Vec<U256> = vec![
        1.into(),
        2.into(),
        3.into(),
        4.into(),
    ];
    vesting_escrow_instance.fund(owner, _recipients,_amounts);
    assert_eq!(vesting_escrow_instance.initial_locked_supply(), 10.into());
    assert_eq!(vesting_escrow_instance.unallocated_supply(), 90.into());
    assert_eq!(vesting_escrow_instance.initial_locked(user_1), 1.into());
    assert_eq!(vesting_escrow_instance.initial_locked(user_2), 2.into());
    assert_eq!(vesting_escrow_instance.initial_locked(user_3), 3.into());
    assert_eq!(vesting_escrow_instance.initial_locked(user_4), 4.into());
  
}

#[test]
#[should_panic]
fn test_fund_by_user() {
    let (env, vesting_escrow_instance, owner, user1, token) = deploy();
    let _user = env.next_user();
    assert_eq!(
        vesting_escrow_instance.token(),
        Key::Hash(token.package_hash())
    );
    assert_eq!(vesting_escrow_instance.start_time(), 10000.into());
    assert_eq!(vesting_escrow_instance.end_time(), 10000.into());
    assert_eq!(vesting_escrow_instance.can_disable(), true);
    assert_eq!(vesting_escrow_instance.admin(), owner.into());
    assert_eq!(vesting_escrow_instance.lock(), 0);
    assert_eq!(vesting_escrow_instance.fund_admins_enabled(), true);
    assert_eq!(vesting_escrow_instance.fund_admins(owner), false);
    assert_eq!(vesting_escrow_instance.fund_admins(user1), true);
    
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
    let user_1=env.next_user();
    let user_2=env.next_user();
    let user_3=env.next_user();
    let user_4=env.next_user();
    let _recipients: Vec<String> = vec![
        user_1.to_formatted_string(),
        user_2.to_formatted_string(),
        user_3.to_formatted_string(),
        user_4.to_formatted_string(),
    ];
    let _amounts: Vec<U256> = vec![
        1.into(),
        2.into(),
        3.into(),
        4.into(),
    ];
    vesting_escrow_instance.fund(_user, _recipients,_amounts);
    assert_eq!(vesting_escrow_instance.initial_locked_supply(), 10.into());
    assert_eq!(vesting_escrow_instance.unallocated_supply(), 90.into());
    assert_eq!(vesting_escrow_instance.initial_locked(user_1), 1.into());
    assert_eq!(vesting_escrow_instance.initial_locked(user_2), 2.into());
    assert_eq!(vesting_escrow_instance.initial_locked(user_3), 3.into());
    assert_eq!(vesting_escrow_instance.initial_locked(user_4), 4.into());
  
}
