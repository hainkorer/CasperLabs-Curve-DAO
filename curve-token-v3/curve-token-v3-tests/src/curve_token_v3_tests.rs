use crate::curve_token_v3_instance::{now, CURVETOKENV3Instance};
use casper_types::{account::AccountHash, runtime_args, Key, RuntimeArgs, U256};
use casperlabs_test_env::{TestContract, TestEnv};
use common::keys::*;

const NAME: &str = "CRVTokenV3";
const SYMBOL: &str = "CRV3";
pub const TEN_E_NINE: u128 = 1000000000;
fn call(env: &TestEnv, owner: AccountHash, runtime_args: RuntimeArgs) {
    TestContract::new(
        env,
        "curve-token-v3-session-code.wasm",
        "curve-token-v3-session-code",
        owner,
        runtime_args,
        now(),
    );
}
fn test_call(env: &TestEnv, owner: AccountHash, runtime_args: RuntimeArgs) {
    TestContract::new(
        env,
        TEST_SESSION_CODE_WASM,
        TEST_SESSION_CODE_NAME,
        owner,
        runtime_args,
        now(),
    );
}
fn deploy_token_erc20(env: &TestEnv, owner: AccountHash) -> TestContract {
    TestContract::new(
        env,
        "erc20-token.wasm",
        "erc20",
        owner,
        runtime_args! {
            "name" => "Token",
            "symbol" => "TK",
            "decimals" => 9_u8,
            "initial_supply" => U256::from(TEN_E_NINE * 1000000000000000000)
        },
        now(),
    )
}
fn deploy_reward(env: &TestEnv, owner: AccountHash) -> TestContract {
    TestContract::new(
        env,
        "erc20-token.wasm",
        "erc20",
        owner,
        runtime_args! {
            "name" => "Reward",
            "symbol" => "RD",
            "decimals" => 9_u8,
            "initial_supply" => U256::from(TEN_E_NINE * 1000000000000000000000)
        },
        now(),
    )
}
fn deploy_curve_rewards(
    env: &TestEnv,
    owner: AccountHash,
    token: Key,
    reward: Key,
) -> TestContract {
    TestContract::new(
        env,
        "curve-rewards.wasm",
        "CURVEREWARDS",
        owner,
        runtime_args! {
            "token" => token,
            "reward" => reward,
        },
        now(),
    )
}
fn deploy() -> (TestEnv, CURVETOKENV3Instance, AccountHash, TestContract) {
    let env = TestEnv::new();
    let owner = env.next_user();
    let token_erc20 = deploy_token_erc20(&env, owner);
    let reward = deploy_reward(&env, owner);
    let curve_reward = deploy_curve_rewards(
        &env,
        owner,
        Key::Hash(token_erc20.package_hash()),
        Key::Hash(reward.package_hash()),
    );
    let token: TestContract =
        CURVETOKENV3Instance::new_deploy(&env, NAME, owner, NAME.to_string(), SYMBOL.to_string());
    (
        env,
        CURVETOKENV3Instance::instance(token),
        owner,
        curve_reward,
    )
}

#[test]
fn deployment() {
    let (_, _, _, _) = deploy();
}
#[test]
fn decimals() {
    let (env, token, owner, _) = deploy();
    test_call(
        &env,
        owner,
        runtime_args! {
            "entrypoint" => "decimals",
            "package_hash" => Key::from(token.package_hash())
        },
    );
    let decimals: U256 = env.query_account_named_key(owner, &["decimals".into()]);
    assert_eq!(decimals, 9.into());
}
#[test]
fn set_minter() {
    let (env, token, owner, _) = deploy();
    let _minter_arg: Key = Key::Account(env.next_user());
    token.set_minter(owner, _minter_arg, now());
    let ret: Key = token.query("minter");
    assert_eq!(ret, _minter_arg);
}
#[test]
fn mint() {
    let (env, token, owner, _) = deploy();
    let _to_arg: Key = Key::from_formatted_str(
        "hash-0000000000000000000000010000000000000000000000000000000000020000",
    )
    .unwrap();
    let _value_arg: U256 = 2000000000.into();
    call(
        &env,
        owner,
        runtime_args! {
            "entrypoint" => "mint",
            "package_hash" => Key::from(token.package_hash()),
            "to" => _to_arg,
            "amount" => _value_arg
        },
    );
    let status: bool = env.query_account_named_key(owner, &["mint".into()]);
    assert!(status);
}
#[test]
fn transfer() {
    let (env, token, owner, _) = deploy();
    let own: Key = Key::Account(owner);
    let _to_arg: Key = Key::Account(env.next_user());
    let _value_arg: U256 = 2000000000.into();
    call(
        &env,
        owner,
        runtime_args! {
            "entrypoint" => "mint",
            "package_hash" => Key::from(token.package_hash()),
            "to" => own,
            "amount" => _value_arg
        },
    );
    call(
        &env,
        owner,
        runtime_args! {
            "entrypoint" => "transfer",
            "package_hash" => Key::from(token.package_hash()),
            "recipient" => _to_arg,
            "amount" => _value_arg
        },
    );
    let status: Result<(), u32> = env.query_account_named_key(owner, &["transfer".into()]);
    match status {
        Ok(()) => {}
        Err(e) => panic!("Transfer Failed ERROR:{}", e),
    }
}
#[test]
fn transfer_from() {
    let (env, token, owner, _) = deploy();
    let to = env.next_user();
    let _value_arg: U256 = 2000000000.into();
    call(
        &env,
        owner,
        runtime_args! {
            "entrypoint" => "mint",
            "package_hash" => Key::from(token.package_hash()),
            "to" => Key::Account(owner),
            "amount" => _value_arg
        },
    );
    token.approve(owner, Key::Account(to), _value_arg, now());
    call(
        &env,
        to,
        runtime_args! {
            "entrypoint" => "transfer_from",
            "package_hash" => Key::from(token.package_hash()),
            "owner" => Key::Account(owner),
            "recipient" => Key::Account(to),
            "amount" => _value_arg
        },
    );
    let status: Result<(), u32> = env.query_account_named_key(to, &["transfer_from".into()]);
    match status {
        Ok(()) => {}
        Err(e) => panic!("Transfer from Failed ERROR:{}", e),
    }
}
#[test]
fn increase_allowance() {
    let (env, token, owner, _) = deploy();
    let spender = env.next_user();
    let _value_arg: U256 = 2000000000.into();
    call(
        &env,
        owner,
        runtime_args! {
            "entrypoint" => "mint",
            "package_hash" => Key::from(token.package_hash()),
            "to" => Key::Account(owner),
            "amount" => _value_arg
        },
    );
    call(
        &env,
        owner,
        runtime_args! {
            "entrypoint" => "increase_allowance",
            "package_hash" => Key::from(token.package_hash()),
            "spender" => Key::Account(spender),
            "amount" => _value_arg
        },
    );
    let status: Result<(), u32> =
        env.query_account_named_key(owner, &["increase_allowance".into()]);
    match status {
        Ok(()) => {}
        Err(e) => panic!("increase_allowance Failed ERROR:{}", e),
    }
}
#[test]
fn decrease_allowance() {
    let (env, token, owner, _) = deploy();
    let spender = env.next_user();
    let _value_arg: U256 = 2000000000.into();
    call(
        &env,
        owner,
        runtime_args! {
            "entrypoint" => "mint",
            "package_hash" => Key::from(token.package_hash()),
            "to" => Key::Account(owner),
            "amount" => _value_arg
        },
    );
    call(
        &env,
        owner,
        runtime_args! {
            "entrypoint" => "increase_allowance",
            "package_hash" => Key::from(token.package_hash()),
            "spender" => Key::Account(spender),
            "amount" => _value_arg
        },
    );
    call(
        &env,
        owner,
        runtime_args! {
            "entrypoint" => "decrease_allowance",
            "package_hash" => Key::from(token.package_hash()),
            "spender" => Key::Account(spender),
            "amount" => _value_arg
        },
    );
    let status: Result<(), u32> =
        env.query_account_named_key(owner, &["decrease_allowance".into()]);
    match status {
        Ok(()) => {}
        Err(e) => panic!("decrease_allowance Failed ERROR:{}", e),
    }
}
#[test]
fn burn_from() {
    let (env, token, owner, _) = deploy();
    let _value_arg: U256 = 2000000000.into();
    call(
        &env,
        owner,
        runtime_args! {
            "entrypoint" => "mint",
            "package_hash" => Key::from(token.package_hash()),
            "to" => Key::Account(owner),
            "amount" => _value_arg
        },
    );
    call(
        &env,
        owner,
        runtime_args! {
            "entrypoint" => "burn_from",
            "package_hash" => Key::from(token.package_hash()),
            "from" => Key::Account(owner),
            "amount" => _value_arg
        },
    );
    let status: bool = env.query_account_named_key(owner, &["burn_from".into()]);
    assert!(status);
}
