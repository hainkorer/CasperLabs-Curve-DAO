use casper_types::{account::AccountHash, runtime_args, Key, RuntimeArgs, U128, U256};
use casperlabs_test_env::{TestContract, TestEnv};

use crate::minter_instance::{add_gauge, MINTERInstance};

const NAME: &str = "MINTER";
const TOKEN_NAME: &str = "ERC20";
const TOKEN_SYMBOL: &str = "ERC";
const DECIMALS: u8 = 8;
const INIT_TOTAL_SUPPLY: u64 = 0;

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

fn deploy() -> (
    TestEnv,
    MINTERInstance,
    AccountHash,
    TestContract,
    TestContract,
    TestContract,
    TestContract,
    TestContract,
) {
    let env = TestEnv::new();
    let owner = env.next_user();
    let _token: TestContract = MINTERInstance::deploy_erc20(
        &env,
        owner,
        TOKEN_NAME,
        TOKEN_SYMBOL,
        DECIMALS,
        INIT_TOTAL_SUPPLY.into(),
    );
    let erc20_crv = MINTERInstance::deploy_erc20_crv(&env, owner);

    let voting_escrow = MINTERInstance::deploy_voting_escrow(
        &env,
        "Voting Escrow",
        owner,
        Key::Hash(erc20_crv.package_hash()),
        "VotingEscrow".into(),
        "VE".into(),
        "1".into(),
    );

    let gauge_controller: TestContract = MINTERInstance::deploy_gauge_controller(
        &env,
        "gauge_controller",
        owner,
        Key::Hash(erc20_crv.package_hash()),
        Key::Hash(voting_escrow.package_hash()),
    );

    let minter: TestContract = MINTERInstance::new_deploy(
        &env,
        NAME,
        owner,
        Key::Hash(erc20_crv.package_hash()),
        Key::Hash(gauge_controller.package_hash()),
    );
    let curve_rewards = deploy_curve_rewards(
        &env,
        owner,
        Key::Hash(deploy_erc20(&env, owner).package_hash()),
        Key::Hash(deploy_erc20(&env, owner).package_hash()),
    );

    let liquidity_gauge_reward = MINTERInstance::deploy_liquidity_gauge_reward(
        &env,
        "Liquidity Gauge Reward",
        owner,
        Key::Hash(_token.package_hash()),
        Key::Hash(minter.package_hash()),
        Key::Hash(curve_rewards.package_hash()),
        Key::Hash(_token.package_hash()),
        Key::Account(owner),
    );
    // let test_contract: TestContract =
    //     MINTERInstance::proxy(&env, Key::Hash(token.contract_hash()), owner);
    // let test_contract2: TestContract =
    //     MINTERInstance::proxy2(&env, Key::Hash(token.contract_hash()), owner);
    (
        env,
        MINTERInstance::instance(minter),
        owner,
        _token,
        voting_escrow,
        gauge_controller,
        liquidity_gauge_reward,
        erc20_crv, // MINTERInstance::instance(test_contract),
                   // MINTERInstance::instance(test_contract2),
    )
}

#[test]
fn test_deploy() {
    let (
        env,
        minter,
        _owner,
        _token,
        _voting_escrow,
        gauge_controller,
        _liquidity_gauge_reward,
        erc20_crv,
    ) = deploy();
    let _user = env.next_user();
    assert_eq!(minter.token(), Key::Hash(erc20_crv.package_hash()));
    assert_eq!(
        minter.controller(),
        Key::Hash(gauge_controller.package_hash())
    );
}

#[test]
fn test_minter_mint() {
    let (
        env,
        minter,
        owner,
        token,
        _voting_escrow,
        gauge_controller,
        liquidity_gauge_reward,
        _erc20_crv,
    ) = deploy();
    let _user = env.next_user();
    minter.toggle_approve_mint(owner, Key::from(owner));
    let name: String = "type".to_string();
    gauge_controller.call_contract(
        owner,
        "add_type",
        runtime_args! {
            "_name" => name,
        },
        0,
    );
    let gauge_type: U128 = 0.into();
    let weight = U256::from(1000000);
    add_gauge(
        &gauge_controller,
        owner,
        Key::Hash(liquidity_gauge_reward.package_hash()),
        gauge_type,
        Some(weight),
    );

    let curve_rewards = deploy_curve_rewards(
        &env,
        owner,
        Key::Hash(deploy_erc20(&env, owner).package_hash()),
        Key::Hash(deploy_erc20(&env, owner).package_hash()),
    );
    let liquidity_gauge_reward_1 = MINTERInstance::deploy_liquidity_gauge_reward(
        &env,
        "Liquidity Gauge Reward 1",
        owner,
        Key::Hash(token.package_hash()),
        Key::from(minter.contract_package_hash()),
        Key::Hash(curve_rewards.package_hash()),
        Key::Hash(token.package_hash()),
        Key::Account(owner),
    );
    let name: String = "type1".to_string();
    gauge_controller.call_contract(
        owner,
        "add_type",
        runtime_args! {
            "_name" => name,
        },
        0,
    );
    let gauge_type: U128 = 1.into();
    add_gauge(
        &gauge_controller,
        owner,
        Key::Hash(liquidity_gauge_reward_1.package_hash()),
        gauge_type,
        Some(weight),
    );
    minter.mint(owner, Key::Hash(liquidity_gauge_reward_1.package_hash()));
}

#[test]
fn test_minter_mint_many() {
    let (
        env,
        minter,
        owner,
        token,
        _voting_escrow,
        gauge_controller,
        liquidity_gauge_reward,
        _erc20_crv,
    ) = deploy();

    let name: String = "type".to_string();
    gauge_controller.call_contract(
        owner,
        "add_type",
        runtime_args! {
            "_name" => name,
        },
        0,
    );
    let gauge_type: U128 = 0.into();
    let weight = U256::from(1000000);
    add_gauge(
        &gauge_controller,
        owner,
        Key::Hash(liquidity_gauge_reward.package_hash()),
        gauge_type,
        Some(weight),
    );

    let curve_rewards = deploy_curve_rewards(
        &env,
        owner,
        Key::Hash(deploy_erc20(&env, owner).package_hash()),
        Key::Hash(deploy_erc20(&env, owner).package_hash()),
    );
    let liquidity_gauge_reward_1 = MINTERInstance::deploy_liquidity_gauge_reward(
        &env,
        "Liquidity Gauge Reward 1",
        owner,
        Key::Hash(token.package_hash()),
        Key::from(minter.contract_package_hash()),
        Key::Hash(curve_rewards.package_hash()),
        Key::Hash(token.package_hash()),
        Key::Account(owner),
    );
    let name: String = "type1".to_string();
    gauge_controller.call_contract(
        owner,
        "add_type",
        runtime_args! {
            "_name" => name,
        },
        0,
    );
    let gauge_type: U128 = 1.into();
    add_gauge(
        &gauge_controller,
        owner,
        Key::Hash(liquidity_gauge_reward_1.package_hash()),
        gauge_type,
        Some(weight),
    );
    let gauge_addrs: Vec<String> = vec![
        // Key::Hash(liquidity_gauge_reward.package_hash()).to_formatted_string(),
        Key::Hash(liquidity_gauge_reward_1.package_hash()).to_formatted_string(),
    ];
    minter.mint_many(owner, gauge_addrs);
}

#[test]
fn test_minter_mint_for() {
    let (
        env,
        minter,
        owner,
        token,
        _voting_escrow,
        gauge_controller,
        liquidity_gauge_reward,
        _erc20_crv,
    ) = deploy();
    minter.toggle_approve_mint(owner, Key::from(owner));
    let name: String = "type".to_string();
    gauge_controller.call_contract(
        owner,
        "add_type",
        runtime_args! {
            "_name" => name,
        },
        0,
    );
    let gauge_type: U128 = 0.into();
    let weight = U256::from(1000000);
    add_gauge(
        &gauge_controller,
        owner,
        Key::Hash(liquidity_gauge_reward.package_hash()),
        gauge_type,
        Some(weight),
    );

    let curve_rewards = deploy_curve_rewards(
        &env,
        owner,
        Key::Hash(deploy_erc20(&env, owner).package_hash()),
        Key::Hash(deploy_erc20(&env, owner).package_hash()),
    );
    let liquidity_gauge_reward_1 = MINTERInstance::deploy_liquidity_gauge_reward(
        &env,
        "Liquidity Gauge Reward 1",
        owner,
        Key::Hash(token.package_hash()),
        Key::from(minter.contract_package_hash()),
        Key::Hash(curve_rewards.package_hash()),
        Key::Hash(token.package_hash()),
        Key::Account(owner),
    );
    let name: String = "type1".to_string();
    gauge_controller.call_contract(
        owner,
        "add_type",
        runtime_args! {
            "_name" => name,
        },
        0,
    );
    let gauge_type: U128 = 1.into();
    add_gauge(
        &gauge_controller,
        owner,
        Key::Hash(liquidity_gauge_reward_1.package_hash()),
        gauge_type,
        Some(weight),
    );

    minter.mint_for(
        owner,
        Key::Hash(liquidity_gauge_reward_1.package_hash()),
        Key::from(owner),
    );
}

#[test]
fn test_minter_toggle_approve_mint() {
    let (
        _env,
        minter,
        owner,
        token,
        _voting_escrow,
        _gauge_controller,
        _liquidity_gauge_reward,
        _erc20_crv,
    ) = deploy();

    minter.toggle_approve_mint(owner, Key::Hash(token.package_hash()));
}
