use crate::fee_distributor_instance::FEEDISTRIBUTORInstance;
use casper_types::{account::AccountHash, runtime_args, Key, RuntimeArgs, U256};
use casperlabs_test_env::{TestContract, TestEnv};
use common::keys::*;
use fee_distributor_crate::data::*;

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

fn deploy_voting_escrow(env: &TestEnv, sender: AccountHash, erc20: &TestContract) -> TestContract {
    TestContract::new(
        env,
        "voting-escrow.wasm",
        "Voting Escrow",
        sender,
        runtime_args! {
            "token_addr" => Key::Hash(erc20.package_hash()),
            "name" => String::from("VotingEscrow"),
            "symbol" => String::from("VE"),
            "version" => String::from("1"),
        },
        0,
    )
}

fn deploy() -> (TestEnv, AccountHash, FEEDISTRIBUTORInstance, TestContract) {
    let env = TestEnv::new();
    let owner = env.next_user();

    let erc20 = deploy_erc20(&env, owner);
    let voting_escrow = deploy_voting_escrow(&env, owner, &erc20);

    let instance = FEEDISTRIBUTORInstance::new(
        &env,
        "Fee Distributor",
        owner,
        Key::Hash(voting_escrow.package_hash()),
        0.into(),
        Key::Hash(erc20.package_hash()),
        Key::Account(owner),
        Key::Account(owner),
    );

    (env, owner, instance, erc20)
}

#[test]
fn test_deploy() {
    let (_, _, _, _) = deploy();
}

#[test]
fn test_checkpoint_token() {
    let (_, owner, instance, _) = deploy();
    instance.checkpoint_token(owner);
}

#[test]
fn test_ve_for_at() {
    let (env, owner, instance, _) = deploy();
    let user: Key = Key::Account(env.next_user());
    let timestamp: U256 = 123.into();
    TestContract::new(
        &env,
        SESSION_CODE_WASM,
        SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(VE_FOR_AT),
            "package_hash" => Key::Hash(instance.package_hash()),
            "user" => user,
            "timestamp" => timestamp
        },
        0,
    );
    let ret: U256 = env.query_account_named_key(owner, &[VE_FOR_AT.into()]);
    assert_eq!(ret, 0.into(), "Invalid default ve value");
}

#[test]
fn test_ve_for_at_js_client() {
    let (env, owner, instance, _) = deploy();
    let user: Key = Key::Account(env.next_user());
    let timestamp: U256 = 123.into();
    instance.ve_for_at_js_client(owner, user, timestamp);
    let ret: U256 = instance.key_value(RESULT.to_string());
    assert_eq!(ret, 0.into(), "Invalid default ve value");
}

#[test]
fn test_checkpoint_total_supply() {
    let (_, owner, instance, _) = deploy();
    instance.checkpoint_total_supply(owner);
}

#[test]
fn test_claim() {
    let (env, owner, instance, _) = deploy();
    let addr: Key = Key::Account(env.next_user());
    TestContract::new(
        &env,
        SESSION_CODE_WASM,
        SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(CLAIM),
            "package_hash" => Key::Hash(instance.package_hash()),
            "addr" => addr
        },
        0,
    );
    let ret: U256 = env.query_account_named_key(owner, &[CLAIM.into()]);
    assert_eq!(ret, 0.into(), "Invalid default claim value");
}

#[test]
fn test_claim_js_client() {
    let (env, owner, instance, _) = deploy();
    let addr: Key = Key::Account(env.next_user());
    instance.claim_js_client(owner, addr);
    let ret: U256 = instance.key_value(RESULT.to_string());
    assert_eq!(ret, 0.into(), "Invalid default claim value");
}

#[test]
fn test_claim_many() {
    let (env, owner, instance, _) = deploy();
    let mut receivers: Vec<Key> = Vec::new();
    receivers.push(Key::Account(env.next_user()));
    receivers.push(Key::Account(env.next_user()));
    receivers.push(Key::Account(env.next_user()));
    TestContract::new(
        &env,
        SESSION_CODE_WASM,
        SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(CLAIM_MANY),
            "package_hash" => Key::Hash(instance.package_hash()),
            "receivers" => receivers
        },
        0,
    );
    let ret: bool = env.query_account_named_key(owner, &[CLAIM_MANY.into()]);
    assert_eq!(ret, true, "Claim should come true");
}

#[test]
fn test_claim_many_js_client() {
    let (env, owner, instance, _) = deploy();
    let mut receivers: Vec<Key> = Vec::new();
    receivers.push(Key::Account(env.next_user()));
    receivers.push(Key::Account(env.next_user()));
    receivers.push(Key::Account(env.next_user()));
    instance.claim_many_js_client(owner, receivers);
    let ret: bool = instance.key_value(RESULT.to_string());
    assert_eq!(ret, true, "Claim should come true");
}

#[test]
fn test_burn() {
    let (env, owner, instance, erc20) = deploy();
    let coin: Key = Key::Hash(erc20.package_hash());
    TestContract::new(
        &env,
        SESSION_CODE_WASM,
        SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(BURN),
            "package_hash" => Key::Hash(instance.package_hash()),
            "coin" => coin
        },
        0,
    );
    let ret: bool = env.query_account_named_key(owner, &[BURN.into()]);
    assert_eq!(ret, true, "Claim should come true");
}

#[test]
fn test_burn_js_client() {
    let (_, owner, instance, erc20) = deploy();
    let coin: Key = Key::Hash(erc20.package_hash());
    instance.burn_js_client(owner, coin);
    let ret: bool = instance.key_value(RESULT.to_string());
    assert_eq!(ret, true, "Burn should come true");
}

#[test]
fn test_commit_admin() {
    let (env, owner, instance, _) = deploy();
    let addr: Key = Key::Account(env.next_user());
    instance.commit_admin(owner, addr);
}

#[test]
fn test_apply_admin() {
    let (env, owner, instance, _) = deploy();
    let addr: Key = Key::Account(env.next_user());
    instance.commit_admin(owner, addr);
    instance.apply_admin(owner);
}

#[test]
fn test_toggle_allow_checkpoint_token() {
    let (_, owner, instance, _) = deploy();
    instance.toggle_allow_checkpoint_token(owner);
}

#[test]
fn test_kill_me() {
    let (_, owner, instance, erc20) = deploy();
    erc20.call_contract(
        owner,
        "mint",
        runtime_args! {
            "to" => Key::Hash(instance.package_hash()),
            "amount" => U256::from(10000)
        },
        0,
    );
    instance.kill_me(owner);
}

#[test]
fn test_recover_balance() {
    let (env, owner, instance, erc20) = deploy();
    let coin: Key = Key::Hash(erc20.package_hash());
    erc20.call_contract(
        owner,
        "mint",
        runtime_args! {
            "to" => Key::Hash(instance.package_hash()),
            "amount" => U256::from(10000)
        },
        0,
    );
    TestContract::new(
        &env,
        SESSION_CODE_WASM,
        SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(RECOVER_BALANCE),
            "package_hash" => Key::Hash(instance.package_hash()),
            "coin" => coin
        },
        0,
    );
    let ret: bool = env.query_account_named_key(owner, &[RECOVER_BALANCE.into()]);
    assert_eq!(ret, true, "Balance recovered should be true");
}

#[test]
fn test_recover_balance_js_client() {
    let (_, owner, instance, erc20) = deploy();
    let coin: Key = Key::Hash(erc20.package_hash());
    erc20.call_contract(
        owner,
        "mint",
        runtime_args! {
            "to" => Key::Hash(instance.package_hash()),
            "amount" => U256::from(10000)
        },
        0,
    );
    instance.recover_balance_js_client(owner, coin);
    let ret: bool = instance.key_value(RESULT.to_string());
    assert_eq!(ret, true, "Balance recovered should be true");
}
