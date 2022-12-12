use crate::curve_token_v3_instance::{address_to_str, now, CURVETOKENV3Instance};
use casper_types::{account::AccountHash, runtime_args, Key, RuntimeArgs, U256};
use casperlabs_test_env::{TestContract, TestEnv};
use common::keys::*;
use curve_erc20_crate::Address;

const NAME: &str = "CRVTokenV3";
const SYMBOL: &str = "CRV3";
pub const TEN_E_NINE: u128 = 1000000000;

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
        "curve-erc20.wasm",
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
        "curve-erc20.wasm",
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
fn deploy() -> (TestEnv, TestContract, AccountHash, TestContract) {
    let env = TestEnv::new();
    let owner = env.next_user();
    let token_erc20 = deploy_token_erc20(&env, owner);
    let reward = deploy_reward(&env, owner);
    let curve_rewards = deploy_curve_rewards(
        &env,
        owner,
        Key::Hash(token_erc20.package_hash()),
        Key::Hash(reward.package_hash()),
    );
    let token: TestContract =
        CURVETOKENV3Instance::new_deploy(&env, NAME, owner, NAME.to_string(), SYMBOL.to_string());
    (env, token, owner, curve_rewards)
}

#[test]
fn deployment() {
    let (_, curve_token_v3, owner, _) = deploy();
    assert_eq!(
        NAME,
        curve_token_v3.query_named_key::<String>("name".into())
    );
    assert_eq!(
        SYMBOL,
        curve_token_v3.query_named_key::<String>("symbol".into())
    );
    assert_eq!(
        Key::from(owner),
        curve_token_v3.query_named_key::<Key>("minter".into())
    );
    assert_eq!(9, curve_token_v3.query_named_key::<u8>("decimals".into()));
    assert_eq!(
        U256::from(0),
        curve_token_v3.query_named_key::<U256>("total_supply".into())
    );
    let balances: U256 = curve_token_v3.query(BALANCES, address_to_str(&Address::Account(owner)));
    assert_eq!(U256::from(0), balances);
}
#[test]
fn decimals() {
    let (env, curve_token_v3, owner, _) = deploy();
    let curve_token_v3 = CURVETOKENV3Instance::instance(curve_token_v3);
    test_call(
        &env,
        owner,
        runtime_args! {
            "entrypoint" => "decimals",
            "package_hash" => Key::from(curve_token_v3.package_hash())
        },
    );
    let decimals: U256 = env.query_account_named_key(owner, &["decimals".into()]);
    assert_eq!(decimals, 9.into());
}
#[test]
fn set_minter() {
    let (env, curve_token_v3, owner, _) = deploy();
    let _minter_arg: Key = Key::Account(env.next_user());
    curve_token_v3.call_contract(
        owner,
        "set_minter",
        runtime_args! {
            "minter" => _minter_arg
        },
        0,
    );
    let ret: Key = curve_token_v3.query_named_key::<Key>("minter".into());
    assert_eq!(ret, _minter_arg);
}
#[test]
fn mint() {
    let (env, curve_token_v3, owner, _) = deploy();
    // let curve_token_v3_instance=CURVETOKENV3Instance::instance(curve_token_v3);
    let _to_arg: Address = Address::from(env.next_user());
    let _value_arg: U256 = 2000000000.into();
    curve_token_v3.call_contract(
        owner,
        "mint",
        runtime_args! {
            "to" => _to_arg,
            "amount"=>_value_arg
        },
        0,
    );
    assert_eq!(
        U256::from(2000000000),
        curve_token_v3.query_named_key::<U256>("total_supply".into())
    );
    let balances: U256 = curve_token_v3.query(BALANCES, address_to_str(&_to_arg));
    assert_eq!(U256::from(2000000000), balances);
}
#[test]
fn transfer() {
    let (env, curve_token_v3, owner, _) = deploy();
    let _owner: Address = Address::Account(owner);
    let _to_arg: Address = Address::Account(env.next_user());
    let _value_arg: U256 = 2000000000.into();
    curve_token_v3.call_contract(
        owner,
        "mint",
        runtime_args! {
            "to" => _owner,
            "amount"=>_value_arg
        },
        0,
    );
    let mut balances: U256 = curve_token_v3.query(BALANCES, address_to_str(&_owner));
    assert_eq!(U256::from(2000000000), balances);
    let _transfer_amount: U256 = 1000000000.into();
    curve_token_v3.call_contract(
        owner,
        "transfer",
        runtime_args! {
            "recipient" => _to_arg,
            "amount" => _transfer_amount

        },
        0,
    );
    balances = curve_token_v3.query(BALANCES, address_to_str(&_to_arg));
    assert_eq!(U256::from(1000000000), balances);
    balances = curve_token_v3.query(BALANCES, address_to_str(&_owner));
    assert_eq!(U256::from(1000000000), balances);
}
#[test]
fn transfer_from() {
    let (env, curve_token_v3, owner, _) = deploy();
    let _owner = Address::Account(owner);
    let to = env.next_user();
    let _value_arg: U256 = 2000000000.into();
    curve_token_v3.call_contract(
        owner,
        "mint",
        runtime_args! {
            "to" => Address::Account(owner),
            "amount"=>_value_arg
        },
        0,
    );
    curve_token_v3.call_contract(
        owner,
        "approve",
        runtime_args! {
            "spender" => Address::Account(to),
            "amount" => _value_arg

        },
        0,
    );
    curve_token_v3.call_contract(
        to,
        "transfer_from",
        runtime_args! {
            "owner" =>Address::Account(owner),
            "recipient" => Key::Account(to),
            "amount" => _value_arg

        },
        0,
    );
    let balances: U256 = curve_token_v3.query(BALANCES, address_to_str(&_owner));
    assert_eq!(U256::from(0), balances);
    let curve_token_v3_instance = CURVETOKENV3Instance::instance(curve_token_v3);
    test_call(
        &env,
        owner,
        runtime_args! {
            "entrypoint" => "allowance",
            "package_hash" => Key::from(curve_token_v3_instance.package_hash()),
            "owner" => Address::Account(owner),
            "spender" => Address::Account(to),

        },
    );
    let allowance: U256 = env.query_account_named_key(owner, &[ALLOWANCE.into()]);
    assert_eq!(U256::from(0), allowance);
}
#[test]
fn increase_allowance() {
    let (env, curve_token_v3, owner, _) = deploy();
    let spender = env.next_user();
    let _value_arg: U256 = 2000000000.into();
    curve_token_v3.call_contract(
        owner,
        "mint",
        runtime_args! {
            "to" => Address::Account(owner),
            "amount"=>_value_arg
        },
        0,
    );
    curve_token_v3.call_contract(
        owner,
        "increase_allowance",
        runtime_args! {
            "spender" => Address::Account(spender),
            "amount" => _value_arg

        },
        0,
    );
    let curve_token_v3_instance = CURVETOKENV3Instance::instance(curve_token_v3);
    test_call(
        &env,
        owner,
        runtime_args! {
            "entrypoint" => "allowance",
            "package_hash" => Key::from(curve_token_v3_instance.package_hash()),
            "owner" => Address::Account(owner),
            "spender" => Address::Account(spender),

        },
    );
    let allowance: U256 = env.query_account_named_key(owner, &[ALLOWANCE.into()]);
    assert_eq!(U256::from(2000000000), allowance);
}
#[test]
fn decrease_allowance() {
    let (env, curve_token_v3, owner, _) = deploy();
    let spender = env.next_user();
    let _value_arg: U256 = 2000000000.into();
    let _decrease_amount: U256 = 1000000000.into();
    curve_token_v3.call_contract(
        owner,
        "mint",
        runtime_args! {
            "to" => Address::Account(owner),
            "amount"=>_value_arg
        },
        0,
    );
    curve_token_v3.call_contract(
        owner,
        "increase_allowance",
        runtime_args! {
            "spender" => Address::Account(spender),
            "amount" => _value_arg

        },
        0,
    );
    curve_token_v3.call_contract(
        owner,
        "decrease_allowance",
        runtime_args! {
            "spender" => Address::Account(spender),
            "amount" => _decrease_amount

        },
        0,
    );
    let curve_token_v3_instance = CURVETOKENV3Instance::instance(curve_token_v3);
    test_call(
        &env,
        owner,
        runtime_args! {
            "entrypoint" => "allowance",
            "package_hash" => Key::from(curve_token_v3_instance.package_hash()),
            "owner" => Address::Account(owner),
            "spender" => Address::Account(spender),

        },
    );
    let allowance: U256 = env.query_account_named_key(owner, &[ALLOWANCE.into()]);
    assert_eq!(U256::from(1000000000), allowance);
}
#[test]
fn burn_from() {
    let (_env, curve_token_v3, owner, _) = deploy();
    let _value_arg: U256 = 2000000000.into();
    let _burn_amount: U256 = 1000000000.into();
    curve_token_v3.call_contract(
        owner,
        "mint",
        runtime_args! {
            "to" => Address::Account(owner),
            "amount"=>_value_arg
        },
        0,
    );
    curve_token_v3.call_contract(
        owner,
        "burn_from",
        runtime_args! {
            "from" => Address::Account(owner),
            "amount" => _burn_amount

        },
        0,
    );
    let balances: U256 = curve_token_v3.query(BALANCES, address_to_str(&Address::Account(owner)));
    assert_eq!(U256::from(1000000000), balances);
    assert_eq!(
        U256::from(1000000000),
        curve_token_v3.query_named_key::<U256>("total_supply".into())
    );
}
#[test]
fn test_set_name() {
    let (_, curve_token_v3, owner, curve_reward) = deploy();
    let curve_rewards_package_hash = Key::Hash(curve_reward.package_hash());
    curve_token_v3.call_contract(
        owner,
        "set_minter",
        runtime_args! {
            "minter" => curve_rewards_package_hash
        },
        0,
    );
    curve_token_v3.call_contract(
        owner,
        "set_name",
        runtime_args! {
            "name" => "curve-token-v3".to_string(),
            "symbol" => "crvtok3".to_string()

        },
        0,
    );
    assert_eq!(
        "curve-token-v3".to_string(),
        curve_token_v3.query_named_key::<String>("name".into())
    );
    assert_eq!(
        "crvtok3".to_string(),
        curve_token_v3.query_named_key::<String>("symbol".into())
    );
}
