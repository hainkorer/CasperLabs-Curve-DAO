use crate::curve_rewards_instance::CURVEREWARDSInstance;
use casper_types::{account::AccountHash, runtime_args, Key, RuntimeArgs, U256};
use common::keys::*;
use test_env::{TestContract, TestEnv};
//Const
pub const TEN_E_NINE: u128 = 1000000000;
fn deploy_token(env: &TestEnv, owner: AccountHash) -> TestContract {
    TestContract::new(
        &env,
        "erc20-token.wasm",
        "erc2020",
        owner,
        runtime_args! {
            "name" => "Token",
            "symbol" => "TK",
            "decimals" => 9 as u8,
            "initial_supply" => U256::from(TEN_E_NINE * 1000000000000000000)
        },
        0,
    )
}
fn deploy_reward(env: &TestEnv, owner: AccountHash) -> TestContract {
    TestContract::new(
        &env,
        "erc20-token.wasm",
        "erc2020",
        owner,
        runtime_args! {
            "name" => "Reward",
            "symbol" => "RD",
            "decimals" => 9 as u8,
            "initial_supply" => U256::from(TEN_E_NINE * 1000000000000000000000)
        },
        0,
    )
}
fn deploy() -> (TestEnv, AccountHash, TestContract) {
    let env = TestEnv::new();
    let owner = env.next_user();
    let token = deploy_token(&env, owner);
    let reward = deploy_reward(&env, owner);
    let curve_rewards_instance = CURVEREWARDSInstance::new(
        &env,
        "CURVEREWARDS",
        owner,
        Key::Hash(token.package_hash()),
        Key::Hash(reward.package_hash()),
    );
    let curve_rewards_package_hash = Key::Hash(curve_rewards_instance.package_hash());
    // For Minting Purpose
    let to: Key = Key::from(curve_rewards_package_hash);
    let amount: U256 = U256::from(TEN_E_NINE * 100000000000000000000);
    token.call_contract(
        owner,
        "mint",
        runtime_args! {"to" => to , "amount" => amount},
        0,
    );
    token.call_contract(
        owner,
        "approve",
        runtime_args! {"spender" => to , "amount" => amount},
        0,
    );
    reward.call_contract(
        owner,
        "mint",
        runtime_args! {"to" => to , "amount" => amount},
        0,
    );
    reward.call_contract(
        owner,
        "approve",
        runtime_args! {"spender" => to , "amount" => amount},
        0,
    );
    (env, owner, curve_rewards_instance)
}

#[test]
fn test_deploy() {
    let (_, _, _) = deploy();
}
#[test]
fn last_time_reward_applicable() {
    let (env, owner, instance) = deploy();
    let package_hash = Key::Hash(instance.package_hash());
    //let curve_rewards_instance = CURVEREWARDSInstance::contract_instance(instance);
    TestContract::new(
        &env,
        "curve-rewards-session-code.wasm",
        SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(LAST_TIME_REWARD_APPLICABLE),
            "package_hash" => package_hash,
        },
        200,
    );
    let ret: U256 = env.query_account_named_key(owner, &[LAST_TIME_REWARD_APPLICABLE.into()]);
    println!("{:?}", ret);
    //proxy.last_time_reward_applicable(owner);
}
#[test]
fn reward_per_token() {
    let (env, owner, instance) = deploy();
    let package_hash = Key::Hash(instance.package_hash());
    let curve_rewards_instance = CURVEREWARDSInstance::contract_instance(instance);
    let amount: U256 = U256::from(TEN_E_NINE * 20);
    curve_rewards_instance.stake(owner, amount);
    curve_rewards_instance.notify_reward_amount(owner, U256::from(TEN_E_NINE * 15));
    TestContract::new(
        &env,
        "curve-rewards-session-code.wasm",
        SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(REWARD_PER_TOKEN),
            "package_hash" => package_hash,
        },
        200,
    );
    let ret: U256 = env.query_account_named_key(owner, &[REWARD_PER_TOKEN.into()]);
    // proxy.reward_per_token(owner);
    // let v1: U256 = proxy.result();
    println!("{:?}", ret);
}

#[test]
fn stake() {
    let (_, owner, instance) = deploy();
    let curve_rewards_instance = CURVEREWARDSInstance::contract_instance(instance);
    let amount: U256 = U256::from(TEN_E_NINE * 20);
    curve_rewards_instance.stake(owner, amount);
}
#[test]
fn withdraw() {
    let (_, owner, instance) = deploy();
    let curve_rewards_instance = CURVEREWARDSInstance::contract_instance(instance);
    let amount: U256 = U256::from(TEN_E_NINE * 20);
    curve_rewards_instance.stake(owner, amount);
    let withdraw_amount: U256 = U256::from(TEN_E_NINE * 10);
    curve_rewards_instance.withdraw(owner, withdraw_amount);
}
#[test]
fn get_reward() {
    let (_, owner, instance) = deploy();
    let curve_rewards_instance = CURVEREWARDSInstance::contract_instance(instance);
    curve_rewards_instance.get_reward(owner);
}

