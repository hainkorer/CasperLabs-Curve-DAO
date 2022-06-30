use casper_types::{account::AccountHash, runtime_args, Key, RuntimeArgs, U256};
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
const DECIMALS: u8 = 8;
const INIT_TOTAL_SUPPLY: u64 = 0;

fn deploy() -> (TestEnv, REWARDONLYGAUGEInstance, TestContract, AccountHash) {
    let env = TestEnv::new();
    let owner = env.next_user();
    let lp_token: TestContract = REWARDONLYGAUGEInstance::erc20_crv(
        &env,
        owner,
        TOKEN_NAME,
        TOKEN_SYMBOL,
        DECIMALS,
        INIT_TOTAL_SUPPLY.into(),
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
    )
}

#[test]
fn test_deploy() {
    let (env, reward_only_gauge, lp_token, owner) = deploy();
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
    let (env, reward_only_gauge, lp_token, owner) = deploy();
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

    reward_only_gauge.set_rewards_receiver(owner, user);
    assert_eq!(reward_only_gauge.rewards_receiver(owner), Key::from(user));
}

#[test]
fn test_commit_transfer_ownership() {
    let (env, reward_only_gauge, lp_token, owner) = deploy();
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

    reward_only_gauge.commit_transfer_ownership(owner, user);
    assert_eq!(reward_only_gauge.admin(), owner.into());
    assert_eq!(reward_only_gauge.future_admin(), user.into());
    // assert_eq!(reward_only_gauge.rewards_receiver(owner), Key::from(user));
}

#[test]
fn test_accept_transfer_ownership() {
    let (env, reward_only_gauge, lp_token, owner) = deploy();
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

    reward_only_gauge.commit_transfer_ownership(owner, user);
    assert_eq!(reward_only_gauge.admin(), owner.into());
    assert_eq!(reward_only_gauge.future_admin(), user.into());
    reward_only_gauge.accept_transfer_ownership(user);
    assert_eq!(reward_only_gauge.admin(), user.into());
}
#[test]
fn test_reward_contract() {
    let (env, reward_only_gauge, lp_token, owner) = deploy();
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
        0,
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
    let (env, reward_only_gauge, lp_token, owner) = deploy();
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
        0,
    );

    let ret: U256 = env.query_account_named_key(owner, &[LAST_CLAIM.into()]);
    assert_eq!(ret, 0.into());
}

#[test]
fn test_claimed_reward() {
    let (env, reward_only_gauge, lp_token, owner) = deploy();
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
            "_addr"=>Key::from(user),
            "_token"=>Key::from(user)
        },
        0,
    );

    let ret: U256 = env.query_account_named_key(owner, &[CLAIMED_REWARD.into()]);
    assert_eq!(ret, 0.into());
}

#[test]
fn test_claimable_reward() {
    let (env, reward_only_gauge, lp_token, owner) = deploy();
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
            "entrypoint" => String::from(CLAIMABLE_REWARD),
            "package_hash" => Key::from(reward_only_gauge.contract_package_hash()),
            "_addr"=>Key::from(user),
            "_token"=>Key::from(user)
        },
        0,
    );

    let ret: U256 = env.query_account_named_key(owner, &[CLAIMABLE_REWARD.into()]);
    assert_eq!(ret, 0.into());
}

#[test]
fn test_approve() {
    let (env, reward_only_gauge, lp_token, owner) = deploy();
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
    reward_only_gauge.approve(owner, user, amount);
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
    let (env, reward_only_gauge, lp_token, owner) = deploy();
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
        0,
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
    let (env, reward_only_gauge, lp_token, owner) = deploy();
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
        0,
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
        0,
    );

    let ret: Result<(), u32> = env.query_account_named_key(owner, &[INCREASE_ALLOWANCE.into()]);
    match ret {
        Ok(()) => {}
        Err(e) => panic!("Decrease Allowance Failed ERROR:{}", e),
    }
    assert_eq!(reward_only_gauge.allowance(owner, user), 90.into());
}
