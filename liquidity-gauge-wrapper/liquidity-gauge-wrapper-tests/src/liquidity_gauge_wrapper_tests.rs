use crate::liquidity_gauge_wrapper_instance::LIQUIDITYGAUGEWRAPPERInstance;
use casper_types::{account::AccountHash, runtime_args, Key, RuntimeArgs, U128, U256};
use casperlabs_test_env::{TestContract, TestEnv};
use common::keys::*;
//Const
pub const TEN_E_NINE: u128 = 1000000000;
const NAME: &str = "LiquidityGuageWrapper";
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
            "decimal" => 9_u8,
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
// Liquidity Gauge V3
fn deploy_liquidity_gauge_v3(
    env: &TestEnv,
    sender: AccountHash,
    lp_addr: Key,
    minter: Key,
    admin: Key,
) -> TestContract {
    TestContract::new(
        env,
        "liquidity-gauge-v3.wasm",
        "liquidity-gauge-v3",
        sender,
        runtime_args! {
            "lp_addr" => lp_addr,
            "minter" => minter,
            "admin" => admin,
        },
        0,
    )
}
fn deploy() -> (TestEnv, AccountHash, TestContract) {
    let env = TestEnv::new();
    let owner = env.next_user();
    let erc20 = deploy_erc20(&env, owner);
    //let reward = deploy_reward(&env, owner);
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
    let deploy_liquidity_gauge_v3 = deploy_liquidity_gauge_v3(
        &env,
        owner,
        Key::Hash(erc20.package_hash()),
        Key::Hash(minter.package_hash()),
        Key::Account(owner),
    );
    let liquidity_gauge_wrapper_instance = LIQUIDITYGAUGEWRAPPERInstance::new_deploy(
        &env,
        NAME,
        owner,
        "Gauge Wrapper".to_string(),
        "LGW".to_string(),
        Key::Hash(deploy_liquidity_gauge_v3.package_hash()),
        Key::Account(owner),
    );
    // For Minting Purpose
    let to = Key::Hash(liquidity_gauge_wrapper_instance.package_hash());
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
    erc20_crv.call_contract(
        owner,
        "set_minter",
        runtime_args! {"_minter" => Key::Account(owner)},
        0,
    );
    // deploy_liquidity_gauge_v3.call_contract(
    //     owner,
    //     "set_rewards",
    //     runtime_args! {"to" => to , "value" => amount_1},
    //     0,
    // );
    erc20_crv.call_contract(
        owner,
        "mint",
        runtime_args! {"to" => to , "value" => amount_1},
        2000000000000000000,
    );

    let _name: String = "type".to_string();
    gauge_controller.call_contract(owner, "add_type", runtime_args! {"_name" => _name }, 0);
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
    gauge_controller.call_contract(owner, "add_type", runtime_args! {"_name" => _name_1 }, 0);
    let addr1: Key = Key::Hash(deploy_liquidity_gauge_v3.package_hash());
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
    (env, owner, liquidity_gauge_wrapper_instance)
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
        "liquidity-gauge-wrapper-session-code.wasm",
        SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(USER_CHECKPOINT),
            "package_hash" => package_hash,
            "addr" => addr,
        },
        300,
    );
}
#[test]
fn test_claimable_tokens() {
    let (env, owner, instance) = deploy();
    let package_hash = Key::Hash(instance.package_hash());
    let addr: Key = Key::Account(owner);
    TestContract::new(
        &env,
        "liquidity-gauge-wrapper-session-code.wasm",
        SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(CLAIMABLE_TOKENS),
            "package_hash" => package_hash,
            "addr" => addr,
        },
        300,
    );
}
#[test]
fn test_claim_tokens() {
    let (_, owner, instance) = deploy();
    let liquidity_gauge_wrapper_instance =
        LIQUIDITYGAUGEWRAPPERInstance::contract_instance(instance);
    liquidity_gauge_wrapper_instance.claim_tokens(owner, None);
}
#[test]
fn test_set_approve_deposit() {
    let (_, owner, instance) = deploy();
    let addr: Key = Key::Account(owner);
    let liquidity_gauge_wrapper_instance =
        LIQUIDITYGAUGEWRAPPERInstance::contract_instance(instance);
    liquidity_gauge_wrapper_instance.set_approve_deposit(owner, addr, true);
}
#[test]
fn test_deposit() {
    let (_, owner, instance) = deploy();
    let liquidity_gauge_wrapper_instance =
        LIQUIDITYGAUGEWRAPPERInstance::contract_instance(instance);
    liquidity_gauge_wrapper_instance.deposit(owner, U256::from(TEN_E_NINE * 10), None);
}
#[test]
fn test_withdraw() {
    let (_, owner, instance) = deploy();
    let addr: Key = Key::Account(owner);
    let liquidity_gauge_wrapper_instance =
        LIQUIDITYGAUGEWRAPPERInstance::contract_instance(instance);
    liquidity_gauge_wrapper_instance.deposit(owner, U256::from(TEN_E_NINE * 1000), None);
    liquidity_gauge_wrapper_instance.withdraw(owner, U256::from(TEN_E_NINE * 10), addr);
}
#[test]
fn test_allowance() {
    let (env, owner, instance) = deploy();
    let package_hash = Key::Hash(instance.package_hash());
    let user_1: Key = env.next_user().into();
    TestContract::new(
        &env,
        "liquidity-gauge-wrapper-session-code.wasm",
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
}
#[test]
fn test_transfer() {
    let (env, owner, instance) = deploy();
    let recipient: Key = env.next_user().into();
    let liquidity_gauge_wrapper_instance =
        LIQUIDITYGAUGEWRAPPERInstance::contract_instance(instance);
    liquidity_gauge_wrapper_instance.deposit(owner, U256::from(TEN_E_NINE * 1000), None);
    liquidity_gauge_wrapper_instance.transfer(owner, recipient, U256::from(TEN_E_NINE * 10));
}
#[test]
fn test_transfer_from() {
    let (env, owner, instance) = deploy();
    let recipient: Key = env.next_user().into();
    let liquidity_gauge_wrapper_instance =
        LIQUIDITYGAUGEWRAPPERInstance::contract_instance(instance);
    liquidity_gauge_wrapper_instance.deposit(owner, U256::from(TEN_E_NINE * 1000), None);
    liquidity_gauge_wrapper_instance.approve(
        owner,
        Key::Account(owner),
        U256::from(TEN_E_NINE * 100),
    );
    liquidity_gauge_wrapper_instance.transfer_from(owner, Key::Account(owner), recipient, 0.into());
}
#[test]
fn test_approve() {
    let (_, owner, instance) = deploy();
    let liquidity_gauge_wrapper_instance =
        LIQUIDITYGAUGEWRAPPERInstance::contract_instance(instance);
    liquidity_gauge_wrapper_instance.approve(
        owner,
        Key::Account(owner),
        U256::from(TEN_E_NINE * 100),
    );
}
#[test]
fn test_increase_allowance() {
    let (_, owner, instance) = deploy();
    let liquidity_gauge_wrapper_instance =
        LIQUIDITYGAUGEWRAPPERInstance::contract_instance(instance);
    liquidity_gauge_wrapper_instance.approve(
        owner,
        Key::Account(owner),
        U256::from(TEN_E_NINE * 100),
    );
    liquidity_gauge_wrapper_instance.increase_allowance(
        owner,
        Key::Account(owner),
        U256::from(TEN_E_NINE * 10),
    );
}
#[test]
fn test_decrease_allowance() {
    let (_, owner, instance) = deploy();
    let liquidity_gauge_wrapper_instance =
        LIQUIDITYGAUGEWRAPPERInstance::contract_instance(instance);
    liquidity_gauge_wrapper_instance.approve(
        owner,
        Key::Account(owner),
        U256::from(TEN_E_NINE * 100),
    );
    liquidity_gauge_wrapper_instance.decrease_allowance(
        owner,
        Key::Account(owner),
        U256::from(TEN_E_NINE * 10),
    );
}
#[test]
fn test_kill_me() {
    let (_, owner, instance) = deploy();
    let liquidity_gauge_wrapper_instance =
        LIQUIDITYGAUGEWRAPPERInstance::contract_instance(instance);
    liquidity_gauge_wrapper_instance.approve(
        owner,
        Key::Account(owner),
        U256::from(TEN_E_NINE * 100),
    );
    liquidity_gauge_wrapper_instance.kill_me(owner);
}
#[test]
fn test_commit_transfer_ownership() {
    let (_, owner, instance) = deploy();
    let liquidity_gauge_wrapper_instance =
        LIQUIDITYGAUGEWRAPPERInstance::contract_instance(instance);
    let addr: Key = Key::Account(owner);
    liquidity_gauge_wrapper_instance.commit_transfer_ownership(owner, addr);
}

#[test]
fn test_apply_transfer_ownership() {
    let (_, owner, instance) = deploy();
    let addr: Key = Key::Account(owner);
    let liquidity_gauge_wrapper_instance =
        LIQUIDITYGAUGEWRAPPERInstance::contract_instance(instance);
    liquidity_gauge_wrapper_instance.commit_transfer_ownership(owner, addr);
    liquidity_gauge_wrapper_instance.apply_transfer_ownership(owner);
}
