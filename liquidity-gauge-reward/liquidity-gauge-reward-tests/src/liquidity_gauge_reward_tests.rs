use crate::liquidity_gauge_reward_instance::LIQUIDITYGAUGEREWARDInstance;
use casper_types::{account::AccountHash, runtime_args, Key, RuntimeArgs, U256};
use casperlabs_test_env::{TestContract, TestEnv};

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

fn deploy_erc20_crv(env: &TestEnv, sender: AccountHash) -> TestContract {
    TestContract::new(
        env,
        "erc20_crv.wasm",
        "erc20-crv",
        sender,
        runtime_args! {
            "name" => "CRV",
            "symbol" => "ERC20CRV",
            "decimals" => 9_u8,
            "supply" => U256::from(0)
        },
        200000000000,
    )
}

fn deploy_curve_rewards(
    env: &TestEnv,
    sender: AccountHash,
    token: Key,
    reward: Key,
) -> TestContract {
    TestContract::new(
        env,
        "curve-rewards.wasm",
        "curve-rewards",
        sender,
        runtime_args! {
            "token" => token,
            "reward" => reward
        },
        0,
    )
}

fn deploy_voting_escrow(
    env: &TestEnv,
    sender: AccountHash,
    token_addr: Key,
    name: String,
    symbol: String,
    version: String,
) -> TestContract {
    TestContract::new(
        env,
        "voting-escrow.wasm",
        "voting-escrow",
        sender,
        runtime_args! {
            "token_addr" => token_addr,
            "name" => name,
            "symbol" => symbol,
            "version" => version,
        },
        0,
    )
}

fn deploy_gauge_controller(
    env: &TestEnv,
    sender: AccountHash,
    token: Key,
    voting_escrow: Key,
) -> TestContract {
    TestContract::new(
        env,
        "gauge-controller-token.wasm",
        "gauge-controller",
        sender,
        runtime_args! {
            "token" => token,
            "voting_escrow" => voting_escrow,
        },
        0,
    )
}

fn deploy_minter(env: &TestEnv, sender: AccountHash, controller: Key, token: Key) -> TestContract {
    TestContract::new(
        env,
        "minter-token.wasm",
        "minter",
        sender,
        runtime_args! {
            "controller" => controller,
            "token" => token,
        },
        0,
    )
}

fn deploy() -> (
    TestEnv,
    AccountHash,
    LIQUIDITYGAUGEREWARDInstance,
    TestContract,
) {
    let env = TestEnv::new();
    let owner = env.next_user();
    let erc20 = deploy_erc20(&env, owner);
    let erc20_crv = deploy_erc20_crv(&env, owner);
    let voting_escrow = deploy_voting_escrow(
        &env,
        owner,
        Key::Hash(erc20.package_hash()),
        "Voting Escrow".into(),
        "VT".into(),
        "1".into(),
    );
    let gauge_controller = deploy_gauge_controller(
        &env,
        owner,
        Key::Hash(erc20.package_hash()),
        Key::Hash(voting_escrow.package_hash()),
    );
    let minter = deploy_minter(
        &env,
        owner,
        Key::Hash(gauge_controller.package_hash()),
        Key::Hash(erc20_crv.package_hash()),
    );
    let curve_rewards = deploy_curve_rewards(
        &env,
        owner,
        Key::Hash(deploy_erc20(&env, owner).package_hash()),
        Key::Hash(deploy_erc20(&env, owner).package_hash()),
    );
    let instance = LIQUIDITYGAUGEREWARDInstance::new_deploy(
        &env,
        "Liquidity Gauge Reward",
        owner,
        Key::Hash(erc20.package_hash()),
        Key::Hash(minter.package_hash()),
        Key::Hash(curve_rewards.package_hash()),
        Key::Hash(erc20.package_hash()),
        Key::Account(owner),
    );
    (env, owner, instance, erc20)
}

#[test]
fn test_deploy() {
    let (_, _, _, _) = deploy();
}

#[test]
fn test_user_checkpoint() {
    let (_, owner, instance, _) = deploy();
    let addr: Key = Key::Account(owner);
    instance.user_checkpoint(owner, addr);
}

#[test]
fn test_claimable_tokens() {
    let (_env, owner, instance, _) = deploy();
    let addr: Key = Key::Account(owner);
    instance.claimable_tokens(owner, addr);
}

#[test]
fn test_claimable_reward() {
    let (_env, owner, instance, _) = deploy();
    let addr: Key = Key::Account(owner);
    instance.claimable_reward(owner, addr);
}

#[test]
fn test_set_approve_deposit() {
    let (_env, owner, instance, _) = deploy();
    let addr: Key = Key::Account(owner);
    let can_deposit: bool = true;
    instance.set_approve_deposit(owner, addr, can_deposit);
}

#[test]
fn test_deposit() {
    let (_env, owner, instance, erc20) = deploy();
    let value: U256 = 100.into();
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
    instance.deposit(owner, None, value);
}

#[test]
fn test_withdraw() {
    let (_env, owner, instance, erc20) = deploy();
    let claim_rewards: bool = true;
    let value: U256 = 100.into();
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
    instance.deposit(owner, None, value);
    instance.withdraw(owner, claim_rewards, value);
}

#[test]
fn test_claim_rewards() {
    let (_env, owner, instance, _) = deploy();
    instance.claim_rewards(owner, None);
}

#[test]
fn test_integrate_checkpoint() {
    let (_env, owner, instance, _) = deploy();
    instance.integrate_checkpoint(owner);
}

#[test]
fn test_kill_me() {
    let (_env, owner, instance, _) = deploy();
    instance.kill_me(owner);
}

#[test]
fn test_commit_transfer_ownership() {
    let (_env, owner, instance, _) = deploy();
    let addr: Key = Key::Account(owner);
    instance.commit_transfer_ownership(owner, addr);
}

#[test]
fn test_apply_transfer_ownership() {
    let (_env, owner, instance, _) = deploy();
    let addr: Key = Key::Account(owner);
    instance.commit_transfer_ownership(owner, addr);
    instance.apply_transfer_ownership(owner);
}

#[test]
fn test_toggle_external_rewards_claim() {
    let (_env, owner, instance, _) = deploy();
    let val: bool = true;
    instance.toggle_external_rewards_claim(owner, val);
}
