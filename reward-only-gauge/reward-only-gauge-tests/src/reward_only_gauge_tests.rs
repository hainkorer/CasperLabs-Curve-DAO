use casper_types::{account::AccountHash, bytesrepr::Bytes, runtime_args, Key, RuntimeArgs, U256};
use casperlabs_test_env::{TestContract, TestEnv};

use crate::reward_only_gauge_instance::REWARDONLYGAUGEInstance;
use casper_types_derive::{CLTyped, FromBytes, ToBytes};
use common::keys::*;

#[derive(Clone, Copy, CLTyped, ToBytes, FromBytes)]
pub struct ClaimDataStruct {
    pub claimable_amount: U256,
    pub claimed_amount: U256,
}

const NAME: &str = "REWARDONLYGAUGE";

const TOKEN_NAME: &str = "ERC20";
const TOKEN_SYMBOL: &str = "ERC";
const DECIMALS: u8 = 9;
const INIT_TOTAL_SUPPLY: u64 = 0;
pub const TEN_E_NINE: u128 = 1000000000;
fn deploy_erc20(env: &TestEnv, owner: AccountHash, name: &str,symbol: &str,decimals: u8) -> TestContract {
    TestContract::new(
        env,
        "erc20-token.wasm",
        "LP token",
        owner,
        runtime_args! {
            "name" => name,
            "symbol" => symbol,
            "decimals" => decimals,
            "initial_supply" => U256::from(TEN_E_NINE * 100000000000000000000)
        },
    0,
    )
}
fn deploy() -> (
    TestEnv,
    REWARDONLYGAUGEInstance,
    TestContract,
    AccountHash,
    TestContract,
    u64
) {
    let env = TestEnv::new();
    let owner = env.next_user();
    let time_now: u64 = REWARDONLYGAUGEInstance::now();
    let lp_token: TestContract = deploy_erc20(&env, owner,TOKEN_NAME,TOKEN_SYMBOL,DECIMALS);
    let curve_rewards: TestContract = REWARDONLYGAUGEInstance::curve_rewards(
        &env,
        "CURVEREWARDS",
        owner,
        Key::Hash(lp_token.package_hash()),
        Key::Hash(lp_token.package_hash()),
    );
    let reward_only_gauge: TestContract = REWARDONLYGAUGEInstance::new_deploy(
        &env,
        NAME,
        owner,
        Key::from(owner),
        Key::Hash(lp_token.package_hash()),
        
    );
  
    (
        env,
        REWARDONLYGAUGEInstance::instance(reward_only_gauge),
        lp_token,
        owner,
        curve_rewards,
        time_now
    )
}

#[test]
fn test_deploy() {
    let (env, reward_only_gauge, lp_token, owner, _curve_rewards,_) = deploy();
    let user = env.next_user();
    assert_eq!(reward_only_gauge.name(), "Curve.fi ERC RewardGauge Deposit");
    assert_eq!(reward_only_gauge.symbol(), "ERC-gauge");
    assert_eq!(reward_only_gauge.decimals(), 9);
    assert_eq!(reward_only_gauge.total_supply(), 0.into());
    assert_eq!(reward_only_gauge.balance_of(owner), 0.into());
    assert_eq!(reward_only_gauge.balance_of(user), 0.into());
    assert_eq!(reward_only_gauge.admin(), owner.into());
    assert_eq!(
        reward_only_gauge.lp_token(),
        Key::Hash(lp_token.package_hash())
    );
}

#[test]
fn test_set_rewards_receiver() {
    let (env, reward_only_gauge, lp_token, owner, _curve_rewards,time_now) = deploy();
    let user = env.next_user();
    assert_eq!(reward_only_gauge.name(), "Curve.fi ERC RewardGauge Deposit");
    assert_eq!(reward_only_gauge.symbol(), "ERC-gauge");
    assert_eq!(reward_only_gauge.decimals(), 9);
    assert_eq!(reward_only_gauge.total_supply(), 0.into());
    assert_eq!(reward_only_gauge.balance_of(owner), 0.into());
    assert_eq!(reward_only_gauge.balance_of(user), 0.into());
    assert_eq!(reward_only_gauge.admin(), owner.into());
    assert_eq!(
        reward_only_gauge.lp_token(),
        Key::Hash(lp_token.package_hash())
    );

    reward_only_gauge.set_rewards_receiver(owner,time_now, user);
    assert_eq!(reward_only_gauge.rewards_receiver(owner), Key::from(user));
}

#[test]
fn test_commit_transfer_ownership() {
    let (env, reward_only_gauge, lp_token, owner, _curve_rewards,time_now) = deploy();
    let user = env.next_user();
    assert_eq!(reward_only_gauge.name(), "Curve.fi ERC RewardGauge Deposit");
    assert_eq!(reward_only_gauge.symbol(), "ERC-gauge");
    assert_eq!(reward_only_gauge.decimals(), 9);
    assert_eq!(reward_only_gauge.total_supply(), 0.into());
    assert_eq!(reward_only_gauge.balance_of(owner), 0.into());
    assert_eq!(reward_only_gauge.balance_of(user), 0.into());
    assert_eq!(reward_only_gauge.admin(), owner.into());
    assert_eq!(
        reward_only_gauge.lp_token(),
        Key::Hash(lp_token.package_hash())
    );

    reward_only_gauge.commit_transfer_ownership(owner,time_now, user);
    assert_eq!(reward_only_gauge.admin(), owner.into());
    assert_eq!(reward_only_gauge.future_admin(), user.into());
}

#[test]
fn test_accept_transfer_ownership() {
    let (env, reward_only_gauge, lp_token, owner, _curve_rewards,time_now) = deploy();
    let user = env.next_user();
    assert_eq!(reward_only_gauge.name(), "Curve.fi ERC RewardGauge Deposit");
    assert_eq!(reward_only_gauge.symbol(), "ERC-gauge");
    assert_eq!(reward_only_gauge.decimals(), 9);
    assert_eq!(reward_only_gauge.total_supply(), 0.into());
    assert_eq!(reward_only_gauge.balance_of(owner), 0.into());
    assert_eq!(reward_only_gauge.balance_of(user), 0.into());
    assert_eq!(reward_only_gauge.admin(), owner.into());
    assert_eq!(
        reward_only_gauge.lp_token(),
        Key::Hash(lp_token.package_hash())
    );

    reward_only_gauge.commit_transfer_ownership(owner,time_now, user);
    assert_eq!(reward_only_gauge.admin(), owner.into());
    assert_eq!(reward_only_gauge.future_admin(), user.into());
    reward_only_gauge.accept_transfer_ownership(user,time_now);
    assert_eq!(reward_only_gauge.admin(), user.into());
}
#[test]
fn test_reward_contract() {
    let (env, reward_only_gauge, lp_token, owner, _curve_rewards,time_now) = deploy();
    let user = env.next_user();

    assert_eq!(reward_only_gauge.name(), "Curve.fi ERC RewardGauge Deposit");
    assert_eq!(reward_only_gauge.symbol(), "ERC-gauge");
    assert_eq!(reward_only_gauge.decimals(), 9);
    assert_eq!(reward_only_gauge.total_supply(), 0.into());
    assert_eq!(reward_only_gauge.balance_of(owner), 0.into());
    assert_eq!(reward_only_gauge.balance_of(user), 0.into());
    assert_eq!(reward_only_gauge.admin(), owner.into());
    assert_eq!(
        reward_only_gauge.lp_token(),
        Key::Hash(lp_token.package_hash())
    );
    assert_eq!(reward_only_gauge.allowance(owner, user), 0.into());

    TestContract::new(
        &env,
        "reward-only-gauge-session-code.wasm",
        "SessionCode",
        owner,
        runtime_args! {
            "entrypoint" => String::from(REWARD_CONTRACT),
            "package_hash" => Key::from(reward_only_gauge.contract_package_hash()),
        },
        time_now,
    );

    let ret: Key = env.query_account_named_key(owner, &[REWARD_CONTRACT.into()]);
    assert_eq!(
        ret,
        Key::from_formatted_str(
            "hash-0000000000000000000000000000000000000000000000000000000000000000"
        )
        .unwrap()
    );
}

#[test]
fn test_last_claim() {
    let (env, reward_only_gauge, lp_token, owner, _curve_rewards,time_now) = deploy();
    let user = env.next_user();

    assert_eq!(reward_only_gauge.name(), "Curve.fi ERC RewardGauge Deposit");
    assert_eq!(reward_only_gauge.symbol(), "ERC-gauge");
    assert_eq!(reward_only_gauge.decimals(), 9);
    assert_eq!(reward_only_gauge.total_supply(), 0.into());
    assert_eq!(reward_only_gauge.balance_of(owner), 0.into());
    assert_eq!(reward_only_gauge.balance_of(user), 0.into());
    assert_eq!(reward_only_gauge.admin(), owner.into());
    assert_eq!(
        reward_only_gauge.lp_token(),
        Key::Hash(lp_token.package_hash())
    );
    assert_eq!(reward_only_gauge.allowance(owner, user), 0.into());

    TestContract::new(
        &env,
        "reward-only-gauge-session-code.wasm",
        "SessionCode",
        owner,
        runtime_args! {
            "entrypoint" => String::from(LAST_CLAIM),
            "package_hash" => Key::from(reward_only_gauge.contract_package_hash()),
        },
        time_now,
    );

    let ret: U256 = env.query_account_named_key(owner, &[LAST_CLAIM.into()]);
    assert_eq!(ret, 0.into());
}

#[test]
fn test_claimed_reward() {
    let (env, reward_only_gauge, lp_token, owner, _curve_rewards,time_now) = deploy();
    let user = env.next_user();

    assert_eq!(reward_only_gauge.name(), "Curve.fi ERC RewardGauge Deposit");
    assert_eq!(reward_only_gauge.symbol(), "ERC-gauge");
    assert_eq!(reward_only_gauge.decimals(), 9);
    assert_eq!(reward_only_gauge.total_supply(), 0.into());
    assert_eq!(reward_only_gauge.balance_of(owner), 0.into());
    assert_eq!(reward_only_gauge.balance_of(user), 0.into());
    assert_eq!(reward_only_gauge.admin(), owner.into());
    assert_eq!(
        reward_only_gauge.lp_token(),
        Key::Hash(lp_token.package_hash())
    );
    assert_eq!(reward_only_gauge.allowance(owner, user), 0.into());

    TestContract::new(
        &env,
        "reward-only-gauge-session-code.wasm",
        "SessionCode",
        owner,
        runtime_args! {
            "entrypoint" => String::from(CLAIMED_REWARD),
            "package_hash" => Key::from(reward_only_gauge.contract_package_hash()),
            "addr"=>Key::from(user),
            "token"=>Key::from(user)
        },
        time_now,
    );

    let ret: U256 = env.query_account_named_key(owner, &[CLAIMED_REWARD.into()]);
    assert_eq!(ret, 0.into());
}

#[test]
fn test_approve() {
    let (env, reward_only_gauge, lp_token, owner, _curve_rewards,time_now) = deploy();
    let user = env.next_user();
    assert_eq!(reward_only_gauge.name(), "Curve.fi ERC RewardGauge Deposit");
    assert_eq!(reward_only_gauge.symbol(), "ERC-gauge");
    assert_eq!(reward_only_gauge.decimals(), 9);
    assert_eq!(reward_only_gauge.total_supply(), 0.into());
    assert_eq!(reward_only_gauge.balance_of(owner), 0.into());
    assert_eq!(reward_only_gauge.balance_of(user), 0.into());
    assert_eq!(reward_only_gauge.admin(), owner.into());
    assert_eq!(
        reward_only_gauge.lp_token(),
        Key::Hash(lp_token.package_hash())
    );

    let amount = 10.into();
    reward_only_gauge.approve(owner, time_now,user, amount);
    assert_eq!(
        reward_only_gauge.balance_of(owner),
        INIT_TOTAL_SUPPLY.into()
    );
    assert_eq!(reward_only_gauge.balance_of(user), 0.into());
    assert_eq!(reward_only_gauge.allowance(owner, user), amount);
    assert_eq!(reward_only_gauge.allowance(user, owner), 0.into());
}

#[test]
fn test_increase_allowance() {
    let (env, reward_only_gauge, lp_token, owner, _curve_rewards,time_now) = deploy();
    let user = env.next_user();

    assert_eq!(reward_only_gauge.name(), "Curve.fi ERC RewardGauge Deposit");
    assert_eq!(reward_only_gauge.symbol(), "ERC-gauge");
    assert_eq!(reward_only_gauge.decimals(), 9);
    assert_eq!(reward_only_gauge.total_supply(), 0.into());
    assert_eq!(reward_only_gauge.balance_of(owner), 0.into());
    assert_eq!(reward_only_gauge.balance_of(user), 0.into());
    assert_eq!(reward_only_gauge.admin(), owner.into());
    assert_eq!(
        reward_only_gauge.lp_token(),
        Key::Hash(lp_token.package_hash())
    );
    assert_eq!(reward_only_gauge.allowance(owner, user), 0.into());
    let amount: U256 = 100.into();

    TestContract::new(
        &env,
        "reward-only-gauge-session-code.wasm",
        "SessionCode",
        owner,
        runtime_args! {
            "entrypoint" => String::from(INCREASE_ALLOWANCE),
            "package_hash" => Key::from(reward_only_gauge.contract_package_hash()),
            "spender"=>Key::from(user),
            "amount"=>amount
        },
        time_now,
    );

    let ret: Result<(), u32> = env.query_account_named_key(owner, &[INCREASE_ALLOWANCE.into()]);
    match ret {
        Ok(()) => {}
        Err(e) => panic!("Increase Allowance Failed ERROR:{}", e),
    }
    assert_eq!(reward_only_gauge.allowance(owner, user), 100.into());
}

#[test]
fn test_decrease_allowance() {
    let (env, reward_only_gauge, lp_token, owner, _curve_rewards,time_now) = deploy();
    let user = env.next_user();

    assert_eq!(reward_only_gauge.name(), "Curve.fi ERC RewardGauge Deposit");
    assert_eq!(reward_only_gauge.symbol(), "ERC-gauge");
    assert_eq!(reward_only_gauge.decimals(), 9);
    assert_eq!(reward_only_gauge.total_supply(), 0.into());
    assert_eq!(reward_only_gauge.balance_of(owner), 0.into());
    assert_eq!(reward_only_gauge.balance_of(user), 0.into());
    assert_eq!(reward_only_gauge.admin(), owner.into());
    assert_eq!(
        reward_only_gauge.lp_token(),
        Key::Hash(lp_token.package_hash())
    );
    assert_eq!(reward_only_gauge.allowance(owner, user), 0.into());
    let amount: U256 = 100.into();
    TestContract::new(
        &env,
        "reward-only-gauge-session-code.wasm",
        "SessionCode",
        owner,
        runtime_args! {
            "entrypoint" => String::from(INCREASE_ALLOWANCE),
            "package_hash" => Key::from(reward_only_gauge.contract_package_hash()),
            "spender"=>Key::from(user),
            "amount"=>amount
        },
        time_now,
    );
    let ret: Result<(), u32> = env.query_account_named_key(owner, &[INCREASE_ALLOWANCE.into()]);
    match ret {
        Ok(()) => {}
        Err(e) => panic!("Increase Allowance Failed ERROR:{}", e),
    }
    assert_eq!(reward_only_gauge.allowance(owner, user), 100.into());
    let amount2: U256 = 10.into();
    TestContract::new(
        &env,
        "reward-only-gauge-session-code.wasm",
        "SessionCode",
        owner,
        runtime_args! {
            "entrypoint" => String::from(DECREASE_ALLOWANCE),
            "package_hash" => Key::from(reward_only_gauge.contract_package_hash()),
            "spender"=>Key::from(user),
            "amount"=>amount2
        },
        time_now,
    );

    let ret: Result<(), u32> = env.query_account_named_key(owner, &[INCREASE_ALLOWANCE.into()]);
    match ret {
        Ok(()) => {}
        Err(e) => panic!("Decrease Allowance Failed ERROR:{}", e),
    }
    assert_eq!(reward_only_gauge.allowance(owner, user), 90.into());
}

#[test]
fn test_set_rewards() {
    let (env, reward_only_gauge, lp_token, owner, _curve_rewards,time_now) = deploy();
    let user = env.next_user();

    assert_eq!(reward_only_gauge.name(), "Curve.fi ERC RewardGauge Deposit");
    assert_eq!(reward_only_gauge.symbol(), "ERC-gauge");
    assert_eq!(reward_only_gauge.decimals(), 9);
    assert_eq!(reward_only_gauge.total_supply(), 0.into());
    assert_eq!(reward_only_gauge.balance_of(owner), 0.into());
    assert_eq!(reward_only_gauge.balance_of(user), 0.into());
    assert_eq!(reward_only_gauge.admin(), owner.into());
    assert_eq!(
        reward_only_gauge.lp_token(),
        Key::Hash(lp_token.package_hash())
    );
    assert_eq!(reward_only_gauge.allowance(owner, user), 0.into());

    let claim_sig: Bytes = Bytes::from("get_reward".as_bytes());
    let lp_token1: TestContract = deploy_erc20(
        &env,
        owner,
        "Lp_token1",
        TOKEN_SYMBOL,
        DECIMALS
    );
    let lp_token2: TestContract = deploy_erc20(
        &env,
        owner,
        "Lp_token2",
        TOKEN_SYMBOL,
        DECIMALS
    );
    let lp_token3: TestContract = deploy_erc20(
        &env,
        owner,
        "Lp_token3",
        TOKEN_SYMBOL,
        DECIMALS
    );
    let lp_token4: TestContract = deploy_erc20(
        &env,
        owner,
        "Lp_token4",
        TOKEN_SYMBOL,
        DECIMALS
    );
    let lp_token5: TestContract = deploy_erc20(
        &env,
        owner,
        "Lp_token5",
        TOKEN_SYMBOL,
        DECIMALS
    );
    let lp_token6: TestContract =deploy_erc20(
        &env,
        owner,
        "Lp_token6",
        TOKEN_SYMBOL,
        DECIMALS
    );
    let lp_token7: TestContract = deploy_erc20(
        &env,
        owner,
        "Lp_token7",
        TOKEN_SYMBOL,
        DECIMALS
    );
    let lp_token8: TestContract = deploy_erc20(
        &env,
        owner,
        "Lp_token8",
        TOKEN_SYMBOL,
        DECIMALS
    );

    let reward_tokens: Vec<String> = vec![
        Key::Hash(lp_token1.package_hash()).to_formatted_string(),
        Key::Hash(lp_token2.package_hash()).to_formatted_string(),
        Key::Hash(lp_token3.package_hash()).to_formatted_string(),
        Key::Hash(lp_token4.package_hash()).to_formatted_string(),
        Key::Hash(lp_token5.package_hash()).to_formatted_string(),
        Key::Hash(lp_token6.package_hash()).to_formatted_string(),
        Key::Hash(lp_token7.package_hash()).to_formatted_string(),
        Key::Hash(lp_token8.package_hash()).to_formatted_string(),
    ];

    reward_only_gauge.set_rewards(
        owner,
        time_now,
        Key::Hash(_curve_rewards.package_hash()),
        claim_sig.clone(),
        reward_tokens,
    );
    assert_eq!(reward_only_gauge.claim_sig(), claim_sig);
    assert_eq!(
        reward_only_gauge.reward_tokens(0.into()),
        Key::Hash(lp_token1.package_hash())
    );
    assert_eq!(
        reward_only_gauge.reward_tokens(1.into()),
        Key::Hash(lp_token2.package_hash())
    );
    assert_eq!(
        reward_only_gauge.reward_tokens(2.into()),
        Key::Hash(lp_token3.package_hash())
    );
    assert_eq!(
        reward_only_gauge.reward_tokens(3.into()),
        Key::Hash(lp_token4.package_hash())
    );
    assert_eq!(
        reward_only_gauge.reward_tokens(4.into()),
        Key::Hash(lp_token5.package_hash())
    );
    assert_eq!(
        reward_only_gauge.reward_tokens(5.into()),
        Key::Hash(lp_token6.package_hash())
    );
    assert_eq!(
        reward_only_gauge.reward_tokens(6.into()),
        Key::Hash(lp_token7.package_hash())
    );
    assert_eq!(
        reward_only_gauge.reward_tokens(7.into()),
        Key::Hash(lp_token8.package_hash())
    );
    assert_eq!(reward_only_gauge.name(), "Curve.fi ERC RewardGauge Deposit");
    assert_eq!(reward_only_gauge.symbol(), "ERC-gauge");
    assert_eq!(reward_only_gauge.decimals(), 9);
    assert_eq!(reward_only_gauge.total_supply(), 0.into());
    assert_eq!(reward_only_gauge.balance_of(owner), 0.into());
    assert_eq!(reward_only_gauge.balance_of(user), 0.into());
}

#[test]
fn test_claim_rewards() {
    let (env, reward_only_gauge, lp_token, owner, _curve_rewards,time_now) = deploy();
    let user = env.next_user();

    assert_eq!(reward_only_gauge.name(), "Curve.fi ERC RewardGauge Deposit");
    assert_eq!(reward_only_gauge.symbol(), "ERC-gauge");
    assert_eq!(reward_only_gauge.decimals(), 9);
    assert_eq!(reward_only_gauge.total_supply(), 0.into());
    assert_eq!(reward_only_gauge.balance_of(owner), 0.into());
    assert_eq!(reward_only_gauge.balance_of(user), 0.into());
    assert_eq!(reward_only_gauge.admin(), owner.into());
    assert_eq!(
        reward_only_gauge.lp_token(),
        Key::Hash(lp_token.package_hash())
    );
    assert_eq!(reward_only_gauge.allowance(owner, user), 0.into());

    let claim_sig: Bytes = Bytes::from("get_reward".as_bytes());
    let lp_token1: TestContract = deploy_erc20(
        &env,
        owner,
        "Lp_token1",
        TOKEN_SYMBOL,
        DECIMALS
    );
    let lp_token2: TestContract =deploy_erc20(
        &env,
        owner,
        "Lp_token2",
        TOKEN_SYMBOL,
        DECIMALS
    );
    let lp_token3: TestContract = deploy_erc20(
        &env,
        owner,
        "Lp_token3",
        TOKEN_SYMBOL,
        DECIMALS
    );
    let lp_token4: TestContract = deploy_erc20(
        &env,
        owner,
        "Lp_token4",
        TOKEN_SYMBOL,
        DECIMALS
    );
    let lp_token5: TestContract =deploy_erc20(
        &env,
        owner,
        "Lp_token5",
        TOKEN_SYMBOL,
        DECIMALS
    );
    let lp_token6: TestContract = deploy_erc20(
        &env,
        owner,
        "Lp_token6",
        TOKEN_SYMBOL,
        DECIMALS
    );
    let lp_token7: TestContract =deploy_erc20(
        &env,
        owner,
        "Lp_token7",
        TOKEN_SYMBOL,
        DECIMALS
    );
    let lp_token8: TestContract = deploy_erc20(
        &env,
        owner,
        "Lp_token8",
        TOKEN_SYMBOL,
        DECIMALS
    );

    let reward_tokens: Vec<String> = vec![
        Key::Hash(lp_token1.package_hash()).to_formatted_string(),
        Key::Hash(lp_token2.package_hash()).to_formatted_string(),
        Key::Hash(lp_token3.package_hash()).to_formatted_string(),
        Key::Hash(lp_token4.package_hash()).to_formatted_string(),
        Key::Hash(lp_token5.package_hash()).to_formatted_string(),
        Key::Hash(lp_token6.package_hash()).to_formatted_string(),
        Key::Hash(lp_token7.package_hash()).to_formatted_string(),
        Key::Hash(lp_token8.package_hash()).to_formatted_string(),
    ];

    reward_only_gauge.set_rewards(
        owner,
        time_now,
        Key::Hash(_curve_rewards.package_hash()),
        claim_sig.clone(),
        reward_tokens,
    );
    assert_eq!(reward_only_gauge.claim_sig(), claim_sig);
    assert_eq!(
        reward_only_gauge.reward_tokens(0.into()),
        Key::Hash(lp_token1.package_hash())
    );
    assert_eq!(
        reward_only_gauge.reward_tokens(1.into()),
        Key::Hash(lp_token2.package_hash())
    );
    assert_eq!(
        reward_only_gauge.reward_tokens(2.into()),
        Key::Hash(lp_token3.package_hash())
    );
    assert_eq!(
        reward_only_gauge.reward_tokens(3.into()),
        Key::Hash(lp_token4.package_hash())
    );
    assert_eq!(
        reward_only_gauge.reward_tokens(4.into()),
        Key::Hash(lp_token5.package_hash())
    );
    assert_eq!(
        reward_only_gauge.reward_tokens(5.into()),
        Key::Hash(lp_token6.package_hash())
    );
    assert_eq!(
        reward_only_gauge.reward_tokens(6.into()),
        Key::Hash(lp_token7.package_hash())
    );
    assert_eq!(
        reward_only_gauge.reward_tokens(7.into()),
        Key::Hash(lp_token8.package_hash())
    );
    reward_only_gauge.claim_rewards(owner,time_now, Some(Key::from(owner)), Some(Key::from(user)));

    assert_eq!(reward_only_gauge.name(), "Curve.fi ERC RewardGauge Deposit");
    assert_eq!(reward_only_gauge.symbol(), "ERC-gauge");
    assert_eq!(reward_only_gauge.decimals(), 9);
    assert_eq!(reward_only_gauge.total_supply(), 0.into());
    assert_eq!(reward_only_gauge.balance_of(owner), 0.into());
    assert_eq!(reward_only_gauge.balance_of(user), 0.into());
}

#[test]
fn test_claimable_reward_write() {
    let (env, reward_only_gauge, lp_token, owner, _curve_rewards,time_now) = deploy();
    let user = env.next_user();

    assert_eq!(reward_only_gauge.name(), "Curve.fi ERC RewardGauge Deposit");
    assert_eq!(reward_only_gauge.symbol(), "ERC-gauge");
    assert_eq!(reward_only_gauge.decimals(), 9);
    assert_eq!(reward_only_gauge.total_supply(), 0.into());
    assert_eq!(reward_only_gauge.balance_of(owner), 0.into());
    assert_eq!(reward_only_gauge.balance_of(user), 0.into());
    assert_eq!(reward_only_gauge.admin(), owner.into());
    assert_eq!(
        reward_only_gauge.lp_token(),
        Key::Hash(lp_token.package_hash())
    );
    assert_eq!(reward_only_gauge.allowance(owner, user), 0.into());

    let claim_sig: Bytes = Bytes::from("get_reward".as_bytes());
    let lp_token1: TestContract = deploy_erc20(
        &env,
        owner,
        "Lp_token1",
        TOKEN_SYMBOL,
        DECIMALS
    );
    let lp_token2: TestContract = deploy_erc20(
        &env,
        owner,
        "Lp_token2",
        TOKEN_SYMBOL,
        DECIMALS
    );
    let lp_token3: TestContract =deploy_erc20(
        &env,
        owner,
        "Lp_token3",
        TOKEN_SYMBOL,
        DECIMALS
    );
    let lp_token4: TestContract = deploy_erc20(
        &env,
        owner,
        "Lp_token4",
        TOKEN_SYMBOL,
        DECIMALS
    );
    let lp_token5: TestContract =deploy_erc20(
        &env,
        owner,
        "Lp_token5",
        TOKEN_SYMBOL,
        DECIMALS
    );
    let lp_token6: TestContract =deploy_erc20(
        &env,
        owner,
        "Lp_token6",
        TOKEN_SYMBOL,
        DECIMALS
    );
    let lp_token7: TestContract = deploy_erc20(
        &env,
        owner,
        "Lp_token7",
        TOKEN_SYMBOL,
        DECIMALS
    );
    let lp_token8: TestContract = deploy_erc20(
        &env,
        owner,
        "Lp_token8",
        TOKEN_SYMBOL,
        DECIMALS
    );

    let reward_tokens: Vec<String> = vec![
        Key::Hash(lp_token1.package_hash()).to_formatted_string(),
        Key::Hash(lp_token2.package_hash()).to_formatted_string(),
        Key::Hash(lp_token3.package_hash()).to_formatted_string(),
        Key::Hash(lp_token4.package_hash()).to_formatted_string(),
        Key::Hash(lp_token5.package_hash()).to_formatted_string(),
        Key::Hash(lp_token6.package_hash()).to_formatted_string(),
        Key::Hash(lp_token7.package_hash()).to_formatted_string(),
        Key::Hash(lp_token8.package_hash()).to_formatted_string(),
    ];

    reward_only_gauge.set_rewards(
        owner,
        time_now,
        Key::Hash(_curve_rewards.package_hash()),
        claim_sig.clone(),
        reward_tokens,
    );
    assert_eq!(reward_only_gauge.claim_sig(), claim_sig);
    assert_eq!(
        reward_only_gauge.reward_tokens(0.into()),
        Key::Hash(lp_token1.package_hash())
    );
    assert_eq!(
        reward_only_gauge.reward_tokens(1.into()),
        Key::Hash(lp_token2.package_hash())
    );
    assert_eq!(
        reward_only_gauge.reward_tokens(2.into()),
        Key::Hash(lp_token3.package_hash())
    );
    assert_eq!(
        reward_only_gauge.reward_tokens(3.into()),
        Key::Hash(lp_token4.package_hash())
    );
    assert_eq!(
        reward_only_gauge.reward_tokens(4.into()),
        Key::Hash(lp_token5.package_hash())
    );
    assert_eq!(
        reward_only_gauge.reward_tokens(5.into()),
        Key::Hash(lp_token6.package_hash())
    );
    assert_eq!(
        reward_only_gauge.reward_tokens(6.into()),
        Key::Hash(lp_token7.package_hash())
    );
    assert_eq!(
        reward_only_gauge.reward_tokens(7.into()),
        Key::Hash(lp_token8.package_hash())
    );
    reward_only_gauge.claim_rewards(owner,time_now, Some(Key::from(owner)), Some(Key::from(user)));

    assert_eq!(reward_only_gauge.name(), "Curve.fi ERC RewardGauge Deposit");
    assert_eq!(reward_only_gauge.symbol(), "ERC-gauge");
    assert_eq!(reward_only_gauge.decimals(), 9);
    assert_eq!(reward_only_gauge.total_supply(), 0.into());
    assert_eq!(reward_only_gauge.balance_of(owner), 0.into());
    assert_eq!(reward_only_gauge.balance_of(user), 0.into());

    TestContract::new(
        &env,
        "reward-only-gauge-session-code.wasm",
        "SessionCode",
        owner,
        runtime_args! {
            "entrypoint" => String::from(CLAIMABLE_REWARD_WRITE),
            "package_hash" => Key::from(reward_only_gauge.contract_package_hash()),
            "addr"=>Key::from(user),
            "token"=>Key::Hash(lp_token.package_hash())
        },
        time_now,
    );

    let ret: U256 = env.query_account_named_key(owner, &[CLAIMABLE_REWARD_WRITE.into()]);
    assert_eq!(ret, 0.into());
}

#[test]
fn test_claimable_reward() {
    let (env, reward_only_gauge, lp_token, owner, _curve_rewards,time_now) = deploy();
    let user = env.next_user();

    assert_eq!(reward_only_gauge.name(), "Curve.fi ERC RewardGauge Deposit");
    assert_eq!(reward_only_gauge.symbol(), "ERC-gauge");
    assert_eq!(reward_only_gauge.decimals(), 9);
    assert_eq!(reward_only_gauge.total_supply(), 0.into());
    assert_eq!(reward_only_gauge.balance_of(owner), 0.into());
    assert_eq!(reward_only_gauge.balance_of(user), 0.into());
    assert_eq!(reward_only_gauge.admin(), owner.into());
    assert_eq!(
        reward_only_gauge.lp_token(),
        Key::Hash(lp_token.package_hash())
    );
    assert_eq!(reward_only_gauge.allowance(owner, user), 0.into());

    let claim_sig: Bytes = Bytes::from("get_reward".as_bytes());
    let lp_token1: TestContract = deploy_erc20(
        &env,
        owner,
        "Lp_token1",
        TOKEN_SYMBOL,
        DECIMALS
    );
    let lp_token2: TestContract =deploy_erc20(
        &env,
        owner,
        "Lp_token2",
        TOKEN_SYMBOL,
        DECIMALS
    );
    let lp_token3: TestContract = deploy_erc20(
        &env,
        owner,
        "Lp_token3",
        TOKEN_SYMBOL,
        DECIMALS
    );
    let lp_token4: TestContract = deploy_erc20(
        &env,
        owner,
        "Lp_token4",
        TOKEN_SYMBOL,
        DECIMALS
    );
    let lp_token5: TestContract =deploy_erc20(
        &env,
        owner,
        "Lp_token5",
        TOKEN_SYMBOL,
        DECIMALS
    );
    let lp_token6: TestContract =deploy_erc20(
        &env,
        owner,
        "Lp_token6",
        TOKEN_SYMBOL,
        DECIMALS
    );
    let lp_token7: TestContract = deploy_erc20(
        &env,
        owner,
        "Lp_token7",
        TOKEN_SYMBOL,
        DECIMALS
    );
    let lp_token8: TestContract = deploy_erc20(
        &env,
        owner,
        "Lp_token8",
        TOKEN_SYMBOL,
        DECIMALS
    );

    let reward_tokens: Vec<String> = vec![
        Key::Hash(lp_token1.package_hash()).to_formatted_string(),
        Key::Hash(lp_token2.package_hash()).to_formatted_string(),
        Key::Hash(lp_token3.package_hash()).to_formatted_string(),
        Key::Hash(lp_token4.package_hash()).to_formatted_string(),
        Key::Hash(lp_token5.package_hash()).to_formatted_string(),
        Key::Hash(lp_token6.package_hash()).to_formatted_string(),
        Key::Hash(lp_token7.package_hash()).to_formatted_string(),
        Key::Hash(lp_token8.package_hash()).to_formatted_string(),
    ];

    reward_only_gauge.set_rewards(
        owner,
        time_now,
        Key::Hash(_curve_rewards.package_hash()),
        claim_sig.clone(),
        reward_tokens,
    );
    assert_eq!(reward_only_gauge.claim_sig(), claim_sig);
    assert_eq!(
        reward_only_gauge.reward_tokens(0.into()),
        Key::Hash(lp_token1.package_hash())
    );
    assert_eq!(
        reward_only_gauge.reward_tokens(1.into()),
        Key::Hash(lp_token2.package_hash())
    );
    assert_eq!(
        reward_only_gauge.reward_tokens(2.into()),
        Key::Hash(lp_token3.package_hash())
    );
    assert_eq!(
        reward_only_gauge.reward_tokens(3.into()),
        Key::Hash(lp_token4.package_hash())
    );
    assert_eq!(
        reward_only_gauge.reward_tokens(4.into()),
        Key::Hash(lp_token5.package_hash())
    );
    assert_eq!(
        reward_only_gauge.reward_tokens(5.into()),
        Key::Hash(lp_token6.package_hash())
    );
    assert_eq!(
        reward_only_gauge.reward_tokens(6.into()),
        Key::Hash(lp_token7.package_hash())
    );
    assert_eq!(
        reward_only_gauge.reward_tokens(7.into()),
        Key::Hash(lp_token8.package_hash())
    );
    reward_only_gauge.claim_rewards(owner, time_now,Some(Key::from(owner)), Some(Key::from(user)));

    assert_eq!(reward_only_gauge.name(), "Curve.fi ERC RewardGauge Deposit");
    assert_eq!(reward_only_gauge.symbol(), "ERC-gauge");
    assert_eq!(reward_only_gauge.decimals(), 9);
    assert_eq!(reward_only_gauge.total_supply(), 0.into());
    assert_eq!(reward_only_gauge.balance_of(owner), 0.into());
    assert_eq!(reward_only_gauge.balance_of(user), 0.into());

    TestContract::new(
        &env,
        "reward-only-gauge-session-code.wasm",
        "SessionCode",
        owner,
        runtime_args! {
            "entrypoint" => String::from(CLAIMABLE_REWARD),
            "package_hash" => Key::from(reward_only_gauge.contract_package_hash()),
            "addr"=>Key::from(user),
            "token"=>Key::Hash(lp_token.package_hash())
        },
        time_now,
    );

    let ret: U256 = env.query_account_named_key(owner, &[CLAIMABLE_REWARD.into()]);
    assert_eq!(ret, 0.into());
}

#[test]
fn test_deposit() {
    let (env, reward_only_gauge, lp_token, owner, _curve_rewards,time_now) = deploy();
    let user = env.next_user();

    assert_eq!(reward_only_gauge.name(), "Curve.fi ERC RewardGauge Deposit");
    assert_eq!(reward_only_gauge.symbol(), "ERC-gauge");
    assert_eq!(reward_only_gauge.decimals(), 9);
    assert_eq!(reward_only_gauge.total_supply(), 0.into());
    assert_eq!(reward_only_gauge.balance_of(owner), 0.into());
    assert_eq!(reward_only_gauge.balance_of(user), 0.into());
    assert_eq!(reward_only_gauge.admin(), owner.into());
    assert_eq!(
        reward_only_gauge.lp_token(),
        Key::Hash(lp_token.package_hash())
    );
    assert_eq!(reward_only_gauge.allowance(owner, user), 0.into());

    let deposit: U256 = 10.into();
    lp_token.call_contract(
        owner,
        "approve",
        runtime_args! {
                    "spender"=>Key::from(reward_only_gauge.contract_package_hash()),
                "amount"=>deposit
        },
        time_now,
    );

    reward_only_gauge.deposit(owner,time_now, deposit, Some(Key::from(owner)), Some(false));

    assert_eq!(
        reward_only_gauge.lp_token(),
        Key::Hash(lp_token.package_hash())
    );
    assert_eq!(reward_only_gauge.name(), "Curve.fi ERC RewardGauge Deposit");
    assert_eq!(reward_only_gauge.symbol(), "ERC-gauge");
    assert_eq!(reward_only_gauge.decimals(), 9);
    assert_eq!(reward_only_gauge.total_supply(), 10.into());
    assert_eq!(reward_only_gauge.balance_of(owner), 10.into());
    assert_eq!(reward_only_gauge.balance_of(user), 0.into());
}

#[test]
fn test_withdraw() {
    let (env, reward_only_gauge, lp_token, owner, _curve_rewards,time_now) = deploy();
    let user = env.next_user();

    assert_eq!(reward_only_gauge.name(), "Curve.fi ERC RewardGauge Deposit");
    assert_eq!(reward_only_gauge.symbol(), "ERC-gauge");
    assert_eq!(reward_only_gauge.decimals(), 9);
    assert_eq!(reward_only_gauge.total_supply(), 0.into());
    assert_eq!(reward_only_gauge.balance_of(owner), 0.into());
    assert_eq!(reward_only_gauge.balance_of(user), 0.into());
    assert_eq!(reward_only_gauge.admin(), owner.into());
    assert_eq!(
        reward_only_gauge.lp_token(),
        Key::Hash(lp_token.package_hash())
    );
    assert_eq!(reward_only_gauge.allowance(owner, user), 0.into());

    let deposit: U256 = 10.into();
    lp_token.call_contract(
        owner,
        "approve",
        runtime_args! {
                    "spender"=>Key::from(reward_only_gauge.contract_package_hash()),
                "amount"=>deposit
        },
        time_now,
    );

    reward_only_gauge.deposit(owner, time_now,deposit, Some(Key::from(owner)), Some(false));

    assert_eq!(
        reward_only_gauge.lp_token(),
        Key::Hash(lp_token.package_hash())
    );
    assert_eq!(reward_only_gauge.name(), "Curve.fi ERC RewardGauge Deposit");
    assert_eq!(reward_only_gauge.symbol(), "ERC-gauge");
    assert_eq!(reward_only_gauge.decimals(), 9);
    assert_eq!(reward_only_gauge.total_supply(), 10.into());
    assert_eq!(reward_only_gauge.balance_of(owner), 10.into());
    assert_eq!(reward_only_gauge.balance_of(user), 0.into());
    reward_only_gauge.withdraw(owner, time_now,deposit / 2, Some(false));
    assert_eq!(reward_only_gauge.name(), "Curve.fi ERC RewardGauge Deposit");
    assert_eq!(reward_only_gauge.symbol(), "ERC-gauge");
    assert_eq!(reward_only_gauge.decimals(), 9);
    assert_eq!(reward_only_gauge.total_supply(), 5.into());
    assert_eq!(reward_only_gauge.balance_of(owner), 5.into());
    assert_eq!(reward_only_gauge.balance_of(user), 0.into());
}

#[test]
fn test_transfer() {
    let (env, reward_only_gauge, lp_token, owner, _curve_rewards,time_now) = deploy();
    let user = env.next_user();

    assert_eq!(reward_only_gauge.name(), "Curve.fi ERC RewardGauge Deposit");
    assert_eq!(reward_only_gauge.symbol(), "ERC-gauge");
    assert_eq!(reward_only_gauge.decimals(), 9);
    assert_eq!(reward_only_gauge.total_supply(), 0.into());
    assert_eq!(reward_only_gauge.balance_of(owner), 0.into());
    assert_eq!(reward_only_gauge.balance_of(user), 0.into());
    assert_eq!(reward_only_gauge.admin(), owner.into());
    assert_eq!(
        reward_only_gauge.lp_token(),
        Key::Hash(lp_token.package_hash())
    );
    assert_eq!(reward_only_gauge.allowance(owner, user), 0.into());

    let deposit: U256 = 10.into();
    lp_token.call_contract(
        owner,
        "approve",
        runtime_args! {
                    "spender"=>Key::from(reward_only_gauge.contract_package_hash()),
                "amount"=>deposit
        },
        time_now,
    );

    reward_only_gauge.deposit(owner, time_now,deposit, Some(Key::from(owner)), Some(false));

    assert_eq!(
        reward_only_gauge.lp_token(),
        Key::Hash(lp_token.package_hash())
    );
    assert_eq!(reward_only_gauge.name(), "Curve.fi ERC RewardGauge Deposit");
    assert_eq!(reward_only_gauge.symbol(), "ERC-gauge");
    assert_eq!(reward_only_gauge.decimals(), 9);
    assert_eq!(reward_only_gauge.total_supply(), 10.into());
    assert_eq!(reward_only_gauge.balance_of(owner), 10.into());
    assert_eq!(reward_only_gauge.balance_of(user), 0.into());

    let amount: U256 = 5.into();

    TestContract::new(
        &env,
        "reward-only-gauge-session-code.wasm",
        "SessionCode",
        owner,
        runtime_args! {
            "entrypoint" => String::from(TRANSFER),
            "package_hash" => Key::from(reward_only_gauge.contract_package_hash()),
            "recipient"=>Key::from(user),
            "amount"=>amount
        },
        time_now,
    );

    let ret: Result<(), u32> = env.query_account_named_key(owner, &[TRANSFER.into()]);
    match ret {
        Ok(()) => {}
        Err(e) => panic!("Transfer Failed ERROR:{}", e),
    }
    assert_eq!(reward_only_gauge.allowance(owner, user), 0.into());
    assert_eq!(reward_only_gauge.name(), "Curve.fi ERC RewardGauge Deposit");
    assert_eq!(reward_only_gauge.symbol(), "ERC-gauge");
    assert_eq!(reward_only_gauge.decimals(), 9);
    assert_eq!(reward_only_gauge.total_supply(), 10.into());
    assert_eq!(reward_only_gauge.balance_of(owner), 5.into());
    assert_eq!(reward_only_gauge.balance_of(user), 5.into());
}

#[test]
fn test_transfer_from() {
    let (env, reward_only_gauge, lp_token, owner, _curve_rewards,time_now) = deploy();
    let user = env.next_user();

    assert_eq!(reward_only_gauge.name(), "Curve.fi ERC RewardGauge Deposit");
    assert_eq!(reward_only_gauge.symbol(), "ERC-gauge");
    assert_eq!(reward_only_gauge.decimals(), 9);
    assert_eq!(reward_only_gauge.total_supply(), 0.into());
    assert_eq!(reward_only_gauge.balance_of(owner), 0.into());
    assert_eq!(reward_only_gauge.balance_of(user), 0.into());
    assert_eq!(reward_only_gauge.admin(), owner.into());
    assert_eq!(
        reward_only_gauge.lp_token(),
        Key::Hash(lp_token.package_hash())
    );
    assert_eq!(reward_only_gauge.allowance(owner, user), 0.into());

    let deposit: U256 = 10.into();
    lp_token.call_contract(
        owner,
        "approve",
        runtime_args! {
                    "spender"=>Key::from(reward_only_gauge.contract_package_hash()),
                "amount"=>deposit
        },
        time_now,
    );

    reward_only_gauge.deposit(owner, time_now,deposit, Some(Key::from(owner)), Some(false));

    assert_eq!(
        reward_only_gauge.lp_token(),
        Key::Hash(lp_token.package_hash())
    );
    assert_eq!(reward_only_gauge.name(), "Curve.fi ERC RewardGauge Deposit");
    assert_eq!(reward_only_gauge.symbol(), "ERC-gauge");
    assert_eq!(reward_only_gauge.decimals(), 9);
    assert_eq!(reward_only_gauge.total_supply(), 10.into());
    assert_eq!(reward_only_gauge.balance_of(owner), 10.into());
    assert_eq!(reward_only_gauge.balance_of(user), 0.into());

    let amount: U256 = 100.into();

    TestContract::new(
        &env,
        "reward-only-gauge-session-code.wasm",
        "SessionCode",
        owner,
        runtime_args! {
            "entrypoint" => String::from(INCREASE_ALLOWANCE),
            "package_hash" => Key::from(reward_only_gauge.contract_package_hash()),
            "spender"=>Key::from(user),
            "amount"=>amount
        },
        time_now,
    );

    let ret: Result<(), u32> = env.query_account_named_key(owner, &[INCREASE_ALLOWANCE.into()]);
    match ret {
        Ok(()) => {}
        Err(e) => panic!("Increase Allowance Failed ERROR:{}", e),
    }
    assert_eq!(reward_only_gauge.allowance(owner, user), 100.into());

    let amount: U256 = 5.into();

    TestContract::new(
        &env,
        "reward-only-gauge-session-code.wasm",
        "SessionCode",
        user,
        runtime_args! {
            "entrypoint" => String::from(TRANSFER_FROM),
            "package_hash" => Key::from(reward_only_gauge.contract_package_hash()),
            "owner"=>Key::from(owner),
            "recipient"=>Key::from(user),
            "amount"=>amount
        },
        time_now,
    );

    let ret: Result<(), u32> = env.query_account_named_key(user, &[TRANSFER_FROM.into()]);
    match ret {
        Ok(()) => {}
        Err(e) => panic!("Transfer From Failed ERROR:{}", e),
    }
    assert_eq!(reward_only_gauge.allowance(owner, user), 95.into());
    assert_eq!(reward_only_gauge.name(), "Curve.fi ERC RewardGauge Deposit");
    assert_eq!(reward_only_gauge.symbol(), "ERC-gauge");
    assert_eq!(reward_only_gauge.decimals(), 9);
    assert_eq!(reward_only_gauge.total_supply(), 10.into());
    assert_eq!(reward_only_gauge.balance_of(owner), 5.into());
    assert_eq!(reward_only_gauge.balance_of(user), 5.into());
}
