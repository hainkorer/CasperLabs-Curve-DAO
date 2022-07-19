use crate::liquidity_gauge_reward_wrapper_instance::LIQUIDITYGAUGEREWARDWRAPPERInstance;
use casper_types::{account::AccountHash, runtime_args, Key, RuntimeArgs, U128, U256};
use casperlabs_test_env::{TestContract, TestEnv};
use common::keys::*;
//Const
pub const TEN_E_NINE: u128 = 1000000000;
const NAME: &str = "LiquidityGuageRewardWrapper";
//ERC20
fn deploy_erc20(env: &TestEnv, owner: AccountHash) -> TestContract {
    TestContract::new(
        env,
        "erc20-token.wasm",
        "rewarded_token",
        owner,
        runtime_args! {
            "name" => "rewarded_token",
            "symbol" => "ERA",
            "decimals" => 9_u8,
            "initial_supply" => U256::from(TEN_E_NINE * 100000000000000000000)
        },
        0,
    )
}
// CRV
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
            "supply" => U256::from(TEN_E_NINE * 10000000000000000)
        },
        200000000000,
    )
}
// Voting Escrow
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
//gauge_controller
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
//Reward
fn deploy_reward(env: &TestEnv, owner: AccountHash) -> TestContract {
    TestContract::new(
        env,
        "erc20-token.wasm",
        "reward token",
        owner,
        runtime_args! {
            "name" => "reward token",
            "symbol" => "RT",
            "decimals" => 9_u8,
            "initial_supply" => U256::from(TEN_E_NINE * 100000000000000000000)
        },
        0,
    )
}
//Minter
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
//Curve Rewards
fn deploy_curve_rewards(
    env: &TestEnv,
    owner: AccountHash,
    token: Key,
    reward: Key,
) -> TestContract {
    TestContract::new(
        env,
        "curve-rewards.wasm",
        "Curve Rewards",
        owner,
        runtime_args! {
            "token" => token,
            "reward" => reward
        },
        0,
    )
}
// Liquidity Guage Reward
fn deploy_liquidity_gauge_reward(
    env: &TestEnv,
    owner: AccountHash,
    minter: Key,
    token: Key,
    reward: Key,
) -> TestContract {
    let cruve_rewards = deploy_curve_rewards(env, owner, token, reward);
    TestContract::new(
        env,
        "liquidity-gauge-reward.wasm",
        "Liquidity Guage Reward",
        owner,
        runtime_args! {
            "lp_addr" => token,
            "minter" => minter,
            "reward_contract" => Key::Hash(cruve_rewards.package_hash()),
            "rewarded_token" => token,
            "admin" => Key::Account(owner)
        },
        0,
    )
}
fn deploy() -> (TestEnv, AccountHash, TestContract) {
    let env = TestEnv::new();
    let owner = env.next_user();
    let erc20 = deploy_erc20(&env, owner);
    let reward = deploy_reward(&env, owner);
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
    let deploy_liquidity_gauge_reward = deploy_liquidity_gauge_reward(
        &env,
        owner,
        Key::Hash(minter.package_hash()),
        Key::Hash(erc20.package_hash()),
        Key::Hash(reward.package_hash()),
    );
    let liquidity_gauge_reward_wrapper_instance = LIQUIDITYGAUGEREWARDWRAPPERInstance::new_deploy(
        &env,
        NAME,
        owner,
        "Reward Wrapper".to_string(),
        "LGRW".to_string(),
        Key::Hash(deploy_liquidity_gauge_reward.package_hash()),
        Key::Account(owner),
    );
    // For Minting Purpose
    let to = Key::Hash(liquidity_gauge_reward_wrapper_instance.package_hash());
    let amount: U256 = U256::from(TEN_E_NINE * 100000000000000000000);
    let amount_1: U256 = U256::from(TEN_E_NINE * 100);
    erc20.call_contract(
        owner,
        "mint",
        runtime_args! {"to" => to , "amount" => amount},
        0,
    );
    erc20.call_contract(
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
    erc20_crv.call_contract(
        owner,
        "set_minter",
        runtime_args! {"minter" => Key::Account(owner)},
        0,
    );
    erc20_crv.call_contract(
        owner,
        "mint",
        runtime_args! {"to" => to , "amount" => amount_1},
        2000000000000000000,
    );

    let _name: String = "type".to_string();
    gauge_controller.call_contract(owner, "add_type", runtime_args! {"name" => _name }, 0);
    let addr: Key = Key::Account(owner);
    let gauge_type: U128 = 0.into();
    gauge_controller.call_contract(
        owner,
        "add_gauge",
        runtime_args! {
            "addr" => addr,
            "gauge_type" => gauge_type,
            "weight"=>None::<U256>
        },
        0,
    );
    let _name_1: String = "type1".to_string();
    gauge_controller.call_contract(owner, "add_type", runtime_args! {"name" => _name_1 }, 0);
    let addr1: Key = Key::Hash(deploy_liquidity_gauge_reward.package_hash());
    let gauge_type_1: U128 = 1.into();
    gauge_controller.call_contract(
        owner,
        "add_gauge",
        runtime_args! {
            "addr" => addr1,
            "gauge_type" => gauge_type_1,
            "weight"=>None::<U256>
        },
        0,
    );
    (env, owner, liquidity_gauge_reward_wrapper_instance)
}

#[test]
fn test_deploy() {
    let (_, _, _) = deploy();
}
#[test]
fn test_user_checkpoint() {
    let (env, owner, instance) = deploy();
    let package_hash = Key::Hash(instance.package_hash());
    let addr: Key = Key::Account(owner);
    TestContract::new(
        &env,
        "liquidity-gauge-reward-wrapper-session-code.wasm",
        SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(USER_CHECKPOINT),
            "package_hash" => package_hash,
            "addr" => addr,
        },
        300,
    );
    let ret: bool = env.query_account_named_key(owner, &[USER_CHECKPOINT.into()]);
    assert!(ret, "{} {}", true, "Invalid result");
}
#[test]
fn test_claimable_tokens() {
    let (env, owner, instance) = deploy();
    let package_hash = Key::Hash(instance.package_hash());
    let liquidity_gauge_reward_wrapper_instance =
        LIQUIDITYGAUGEREWARDWRAPPERInstance::contract_instance(instance);
    liquidity_gauge_reward_wrapper_instance.deposit(owner, U256::from(TEN_E_NINE * 1000), None);
    let addr: Key = Key::Account(owner);
    TestContract::new(
        &env,
        "liquidity-gauge-reward-wrapper-session-code.wasm",
        SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(CLAIMABLE_TOKENS),
            "package_hash" => package_hash,
            "addr" => addr,
        },
        300,
    );
    // let ret: U256 = env.query_account_named_key(owner, &[CLAIMABLE_TOKENS.into()]);
    // assert_eq!(ret, U256::from(TEN_E_NINE * 1000), "Invalid result");
}
#[test]
fn test_claimable_reward() {
    let (env, owner, instance) = deploy();
    let package_hash = Key::Hash(instance.package_hash());
    let liquidity_gauge_reward_wrapper_instance =
        LIQUIDITYGAUGEREWARDWRAPPERInstance::contract_instance(instance);
    let addr: Key = Key::Account(owner);
    liquidity_gauge_reward_wrapper_instance.deposit(owner, U256::from(TEN_E_NINE * 1000), None);
    TestContract::new(
        &env,
        "liquidity-gauge-reward-wrapper-session-code.wasm",
        SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(CLAIMABLE_REWARD),
            "package_hash" => package_hash,
            "addr" => addr,
        },
        300,
    );
    let ret: U256 = env.query_account_named_key(owner, &[CLAIMABLE_REWARD.into()]);
    assert_eq!(ret, 0.into(), "Invalid result");
}
#[test]
fn test_claim_tokens() {
    let (env, owner, instance) = deploy();
    let package_hash = Key::Hash(instance.package_hash());
    let liquidity_gauge_reward_wrapper_instance =
        LIQUIDITYGAUGEREWARDWRAPPERInstance::contract_instance(instance);
    liquidity_gauge_reward_wrapper_instance.deposit(owner, U256::from(TEN_E_NINE * 1000), None);
    liquidity_gauge_reward_wrapper_instance.claim_tokens(owner, None);
    TestContract::new(
        &env,
        "liquidity-gauge-reward-wrapper-session-code.wasm",
        SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(BALANCE_OF),
            "package_hash" => package_hash,
            "owner" => Key::Account(owner)
        },
        300,
    );
    let ret: U256 = env.query_account_named_key(owner, &[BALANCE_OF.into()]);
    assert_eq!(ret, U256::from(TEN_E_NINE * 1000), "Invalid result");
}
#[test]
fn test_set_approve_deposit() {
    let (env, owner, instance) = deploy();
    let package_hash = Key::Hash(instance.package_hash());
    let addr: Key = Key::Account(owner);
    let liquidity_gauge_reward_wrapper_instance =
        LIQUIDITYGAUGEREWARDWRAPPERInstance::contract_instance(instance);
    liquidity_gauge_reward_wrapper_instance.set_approve_deposit(owner, addr, true);
    TestContract::new(
        &env,
        "liquidity-gauge-reward-wrapper-session-code.wasm",
        SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(APPROVED_TO_DEPOSIT),
            "package_hash" => package_hash,
            "owner" => Key::Account(owner) ,
            "spender" => Key::Account(owner)
        },
        300,
    );
    let ret: bool = env.query_account_named_key(owner, &[APPROVED_TO_DEPOSIT.into()]);
    assert!(ret, "{} {}", true, "Invalid result");
}
#[test]
fn test_deposit() {
    let (env, owner, instance) = deploy();
    let package_hash = Key::Hash(instance.package_hash());
    let liquidity_gauge_reward_wrapper_instance =
        LIQUIDITYGAUGEREWARDWRAPPERInstance::contract_instance(instance);
    liquidity_gauge_reward_wrapper_instance.deposit(owner, U256::from(TEN_E_NINE * 1000), None);
    TestContract::new(
        &env,
        "liquidity-gauge-reward-wrapper-session-code.wasm",
        SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(BALANCE_OF),
            "package_hash" => package_hash,
            "owner" => Key::Account(owner)
        },
        300,
    );
    let ret: U256 = env.query_account_named_key(owner, &[BALANCE_OF.into()]);
    assert_eq!(ret, U256::from(TEN_E_NINE * 1000), "Invalid result");
}
#[test]
fn test_withdraw() {
    let (env, owner, instance) = deploy();
    let package_hash = Key::Hash(instance.package_hash());
    let addr: Key = Key::Account(owner);
    let liquidity_gauge_reward_wrapper_instance =
        LIQUIDITYGAUGEREWARDWRAPPERInstance::contract_instance(instance);
    liquidity_gauge_reward_wrapper_instance.deposit(owner, U256::from(TEN_E_NINE * 1000), None);
    liquidity_gauge_reward_wrapper_instance.withdraw(owner, U256::from(TEN_E_NINE * 10), addr);
    TestContract::new(
        &env,
        "liquidity-gauge-reward-wrapper-session-code.wasm",
        SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(BALANCE_OF),
            "package_hash" => package_hash,
            "owner" => Key::Account(owner)
        },
        300,
    );
    let v: u128 = 990000000000_u128;
    let ret: U256 = env.query_account_named_key(owner, &[BALANCE_OF.into()]);
    assert_eq!(ret, v.into(), "Invalid result");
}
#[test]
fn test_allowance() {
    let (env, owner, instance) = deploy();
    let package_hash = Key::Hash(instance.package_hash());
    let user_1: Key = env.next_user().into();
    let liquidity_gauge_reward_wrapper_instance =
        LIQUIDITYGAUGEREWARDWRAPPERInstance::contract_instance(instance);
    liquidity_gauge_reward_wrapper_instance.deposit(owner, U256::from(TEN_E_NINE * 1000), None);
    liquidity_gauge_reward_wrapper_instance.approve(owner, user_1, U256::from(TEN_E_NINE * 100));
    TestContract::new(
        &env,
        "liquidity-gauge-reward-wrapper-session-code.wasm",
        SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(ALLOWANCE),
            "package_hash" => package_hash,
            "owner" => Key::Account(owner),
            "spender" => user_1
        },
        300,
    );
    let v: u128 = 100000000000_u128;
    let ret: U256 = env.query_account_named_key(owner, &[ALLOWANCE.into()]);
    assert_eq!(ret, v.into(), "Invalid result");
}
#[test]
fn test_transfer() {
    let (env, owner, instance) = deploy();
    let package_hash = Key::Hash(instance.package_hash());
    let recipient: Key = env.next_user().into();
    let liquidity_gauge_reward_wrapper_instance =
        LIQUIDITYGAUGEREWARDWRAPPERInstance::contract_instance(instance);
    liquidity_gauge_reward_wrapper_instance.deposit(owner, U256::from(TEN_E_NINE * 1000), None);
    liquidity_gauge_reward_wrapper_instance.transfer(owner, recipient, U256::from(TEN_E_NINE * 10));
    TestContract::new(
        &env,
        "liquidity-gauge-reward-wrapper-session-code.wasm",
        SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(BALANCE_OF),
            "package_hash" => package_hash,
            "owner" => Key::Account(owner)
        },
        300,
    );
    let v: u128 = 990000000000_u128;
    let ret: U256 = env.query_account_named_key(owner, &[BALANCE_OF.into()]);
    assert_eq!(ret, v.into(), "Invalid result");
}
#[test]
fn test_transfer_from() {
    let (env, owner, instance) = deploy();
    let package_hash = Key::Hash(instance.package_hash());
    let recipient: Key = env.next_user().into();
    let liquidity_gauge_reward_wrapper_instance =
        LIQUIDITYGAUGEREWARDWRAPPERInstance::contract_instance(instance);
    liquidity_gauge_reward_wrapper_instance.deposit(owner, U256::from(TEN_E_NINE * 1000), None);
    liquidity_gauge_reward_wrapper_instance.approve(
        owner,
        Key::Account(owner),
        U256::from(TEN_E_NINE * 100),
    );
    liquidity_gauge_reward_wrapper_instance.transfer_from(
        owner,
        Key::Account(owner),
        recipient,
        U256::from(TEN_E_NINE * 10),
    );
    TestContract::new(
        &env,
        "liquidity-gauge-reward-wrapper-session-code.wasm",
        SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(ALLOWANCE),
            "package_hash" => package_hash,
            "owner" => Key::Account(owner),
            "spender" => Key::Account(owner)
        },
        300,
    );
    let v: u128 = 90000000000_u128;
    let ret: U256 = env.query_account_named_key(owner, &[ALLOWANCE.into()]);
    assert_eq!(ret, v.into(), "Invalid result");
}
#[test]
fn test_approve() {
    let (env, owner, instance) = deploy();
    let package_hash = Key::Hash(instance.package_hash());
    let liquidity_gauge_reward_wrapper_instance =
        LIQUIDITYGAUGEREWARDWRAPPERInstance::contract_instance(instance);
    liquidity_gauge_reward_wrapper_instance.approve(
        owner,
        Key::Account(owner),
        U256::from(TEN_E_NINE * 100),
    );
    TestContract::new(
        &env,
        "liquidity-gauge-reward-wrapper-session-code.wasm",
        SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(ALLOWANCE),
            "package_hash" => package_hash,
            "owner" => Key::Account(owner),
            "spender" => Key::Account(owner)
        },
        300,
    );
    let v: u128 = 100000000000_u128;
    let ret: U256 = env.query_account_named_key(owner, &[ALLOWANCE.into()]);
    assert_eq!(ret, v.into(), "Invalid result");
}
#[test]
fn test_increase_allowance() {
    let (env, owner, instance) = deploy();
    let package_hash = Key::Hash(instance.package_hash());
    let liquidity_gauge_reward_wrapper_instance =
        LIQUIDITYGAUGEREWARDWRAPPERInstance::contract_instance(instance);
    liquidity_gauge_reward_wrapper_instance.approve(
        owner,
        Key::Account(owner),
        U256::from(TEN_E_NINE * 100),
    );
    liquidity_gauge_reward_wrapper_instance.increase_allowance(
        owner,
        Key::Account(owner),
        U256::from(TEN_E_NINE * 10),
    );
    TestContract::new(
        &env,
        "liquidity-gauge-reward-wrapper-session-code.wasm",
        SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(ALLOWANCE),
            "package_hash" => package_hash,
            "owner" => Key::Account(owner),
            "spender" => Key::Account(owner)
        },
        300,
    );
    let v: u128 = 110000000000_u128;
    let ret: U256 = env.query_account_named_key(owner, &[ALLOWANCE.into()]);
    assert_eq!(ret, v.into(), "Invalid result");
}
#[test]
fn test_decrease_allowance() {
    let (env, owner, instance) = deploy();
    let package_hash = Key::Hash(instance.package_hash());
    let liquidity_gauge_reward_wrapper_instance =
        LIQUIDITYGAUGEREWARDWRAPPERInstance::contract_instance(instance);
    liquidity_gauge_reward_wrapper_instance.approve(
        owner,
        Key::Account(owner),
        U256::from(TEN_E_NINE * 100),
    );
    liquidity_gauge_reward_wrapper_instance.decrease_allowance(
        owner,
        Key::Account(owner),
        U256::from(TEN_E_NINE * 10),
    );
    TestContract::new(
        &env,
        "liquidity-gauge-reward-wrapper-session-code.wasm",
        SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(ALLOWANCE),
            "package_hash" => package_hash,
            "owner" => Key::Account(owner),
            "spender" => Key::Account(owner)
        },
        300,
    );
    let v: u128 = 90000000000_u128;
    let ret: U256 = env.query_account_named_key(owner, &[ALLOWANCE.into()]);
    assert_eq!(ret, v.into(), "Invalid result");
}
#[test]
fn test_kill_me() {
    let (env, owner, instance) = deploy();
    let package_hash = Key::Hash(instance.package_hash());
    let liquidity_gauge_reward_wrapper_instance =
        LIQUIDITYGAUGEREWARDWRAPPERInstance::contract_instance(instance);
    liquidity_gauge_reward_wrapper_instance.approve(
        owner,
        Key::Account(owner),
        U256::from(TEN_E_NINE * 100),
    );
    liquidity_gauge_reward_wrapper_instance.kill_me(owner);
    TestContract::new(
        &env,
        "liquidity-gauge-reward-wrapper-session-code.wasm",
        SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(IS_KILLED),
            "package_hash" => package_hash,
        },
        300,
    );
    let ret: bool = env.query_account_named_key(owner, &[IS_KILLED.into()]);
    assert!(ret, "{} {}", true, "Invalid result");
}
#[test]
fn test_commit_transfer_ownership() {
    let (env, owner, instance) = deploy();
    let package_hash = Key::Hash(instance.package_hash());
    let liquidity_gauge_reward_wrapper_instance =
        LIQUIDITYGAUGEREWARDWRAPPERInstance::contract_instance(instance);
    let addr: Key = Key::Account(owner);
    liquidity_gauge_reward_wrapper_instance.commit_transfer_ownership(owner, addr);
    TestContract::new(
        &env,
        "liquidity-gauge-reward-wrapper-session-code.wasm",
        SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(FUTURE_ADMIN),
            "package_hash" => package_hash,
        },
        300,
    );
    let ret: Key = env.query_account_named_key(owner, &[FUTURE_ADMIN.into()]);
    assert_eq!(ret, addr, "Invalid result");
}

#[test]
fn test_apply_transfer_ownership() {
    let (env, owner, instance) = deploy();
    let package_hash = Key::Hash(instance.package_hash());
    let addr: Key = Key::Account(owner);
    let liquidity_gauge_reward_wrapper_instance =
        LIQUIDITYGAUGEREWARDWRAPPERInstance::contract_instance(instance);
    liquidity_gauge_reward_wrapper_instance.commit_transfer_ownership(owner, addr);
    liquidity_gauge_reward_wrapper_instance.apply_transfer_ownership(owner);
    TestContract::new(
        &env,
        "liquidity-gauge-reward-wrapper-session-code.wasm",
        SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(ADMIN),
            "package_hash" => package_hash,
        },
        300,
    );
    let ret: Key = env.query_account_named_key(owner, &[ADMIN.into()]);
    assert_eq!(ret, addr, "Invalid result");
}
#[should_panic]
#[test]
fn test_apply_transfer_ownership_panic() {
    let (_, owner, instance) = deploy();
    let liquidity_gauge_reward_wrapper_instance =
        LIQUIDITYGAUGEREWARDWRAPPERInstance::contract_instance(instance);
    liquidity_gauge_reward_wrapper_instance.apply_transfer_ownership(owner);
}
#[should_panic]
#[test]
fn test_decrease_allowance_panic() {
    let (_, owner, instance) = deploy();
    let liquidity_gauge_reward_wrapper_instance =
        LIQUIDITYGAUGEREWARDWRAPPERInstance::contract_instance(instance);
    liquidity_gauge_reward_wrapper_instance.decrease_allowance(
        owner,
        Key::Account(owner),
        U256::from(TEN_E_NINE * 10),
    );
}
#[should_panic]
#[test]
fn test_transfer_from_panic() {
    let (env, owner, instance) = deploy();
    let recipient: Key = env.next_user().into();
    let liquidity_gauge_reward_wrapper_instance =
        LIQUIDITYGAUGEREWARDWRAPPERInstance::contract_instance(instance);
    liquidity_gauge_reward_wrapper_instance.transfer_from(
        owner,
        Key::Account(owner),
        recipient,
        100000000.into(),
    );
}
#[should_panic]
#[test]
fn test_transfer_panic() {
    let (env, owner, instance) = deploy();
    let recipient: Key = env.next_user().into();
    let liquidity_gauge_reward_wrapper_instance =
        LIQUIDITYGAUGEREWARDWRAPPERInstance::contract_instance(instance);
    liquidity_gauge_reward_wrapper_instance.transfer(owner, recipient, U256::from(TEN_E_NINE * 10));
}
#[should_panic]
#[test]
fn test_withdraw_panic() {
    let (_, owner, instance) = deploy();
    let addr: Key = Key::Account(owner);
    let liquidity_gauge_reward_wrapper_instance =
        LIQUIDITYGAUGEREWARDWRAPPERInstance::contract_instance(instance);
    liquidity_gauge_reward_wrapper_instance.withdraw(owner, U256::from(TEN_E_NINE * 10), addr);
}
