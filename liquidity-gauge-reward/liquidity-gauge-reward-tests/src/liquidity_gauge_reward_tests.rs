use crate::liquidity_gauge_reward_instance::LIQUIDITYGAUGEREWARDInstance;
use casper_types::{account::AccountHash, runtime_args, Key, RuntimeArgs, U128, U256};
use common::keys::*;
use liquidity_gauge_reward_crate::data::*;
use test_env::{TestContract, TestEnv};

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

fn deploy_erc20_crv(env: &TestEnv, sender: AccountHash) -> TestContract {
    TestContract::new(
        env,
        "erc20_crv.wasm",
        "erc20-crv",
        sender,
        runtime_args! {
            "name" => "CRV",
            "symbol" => "ERC20CRV",
            "decimal" => 9 as u8,
            "supply" => U256::from(0)
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
    let instance = LIQUIDITYGAUGEREWARDInstance::new(
        &env,
        "Liquidity Gauge Reward",
        owner,
        Key::Hash(erc20.package_hash()),
        Key::Hash(minter.package_hash()),
        Key::Hash(erc20.package_hash()),
        Key::Hash(erc20.package_hash()),
        Key::Account(owner),
    );
    (env, owner, instance, erc20)
}

// #[test]
fn test_deploy() {
    let (_, _, _, _) = deploy();
}

#[test]
fn test_user_checkpoint() {
    let (env, owner, instance, _) = deploy();
    let addr: Key = Key::Account(owner);
    instance.user_checkpoint(owner, addr);
    // let ret: Key = instance.key_value(FUTURE_ADMIN.to_string());
    // assert_eq!(ret, addr, "Ownership not transferred");
}
