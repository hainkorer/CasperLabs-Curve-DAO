use crate::fee_distributor_instance::FEEDISTRIBUTORInstance;
use casper_types::{account::AccountHash, runtime_args, Key, RuntimeArgs, U256};
use casperlabs_test_env::{TestContract, TestEnv};
use common::keys::*;
pub const TEN_E_NINE: u128 = 1000000000;
pub const WEEK: U256 = U256([604800000, 0, 0, 0]);
const MILLI_SECONDS_IN_DAY: u64 = 86_400_000;
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
// CRV
fn deploy_erc20_crv(env: &TestEnv, sender: AccountHash,time_now:u64) -> TestContract {
    TestContract::new(
        env,
        "erc20_crv.wasm",
        "erc20-crv",
        sender,
        runtime_args! {
            "name" => "CRV",
            "symbol" => "ERC20CRV",
            "decimals" => 9_u8
        },
        time_now,
    )
}

fn deploy_voting_escrow(
    env: &TestEnv,
    sender: AccountHash,
    erc20_crv: &TestContract,
    time_now:u64
) -> TestContract {
    TestContract::new(
        env,
        "voting-escrow.wasm",
        "Voting Escrow",
        sender,
        runtime_args! {
            "token_addr" => Key::Hash(erc20_crv.package_hash()),
            "name" => String::from("VotingEscrow"),
            "symbol" => String::from("VE"),
            "version" => String::from("1"),
        },
        time_now,
    )
}

fn deploy() -> (
    TestEnv,
    AccountHash,
    FEEDISTRIBUTORInstance,
    TestContract,
    u64,
) {
    let env = TestEnv::new();
    let owner = env.next_user();
    let time_now: u64 = FEEDISTRIBUTORInstance::now();
    let unlock_time = U256::from(time_now.checked_add(MILLI_SECONDS_IN_DAY * 720).unwrap());
    let erc20 = deploy_erc20(&env, owner);
    let erc20_crv = deploy_erc20_crv(&env, owner,time_now);
    let voting_escrow = deploy_voting_escrow(&env, owner, &erc20_crv,time_now);
    erc20_crv.call_contract(
        owner,
        "approve",
        runtime_args! {"spender" => Key::Hash(voting_escrow.package_hash()) , "amount" => U256::from(2500*TEN_E_NINE)},
        time_now,
    );
    voting_escrow.call_contract(
        owner,
        "create_lock",
        runtime_args! {
        "value" => U256::from(2500*TEN_E_NINE),
        "unlock_time" => unlock_time
        },
        time_now,
    );
    let instance = FEEDISTRIBUTORInstance::new_deploy(
        &env,
        "Fee Distributor",
        owner,
        Key::Hash(voting_escrow.package_hash()),
        U256::from(time_now),
        Key::Hash(erc20.package_hash()),
        Key::Account(owner),
        Key::Account(owner),
        time_now
    );

    (env, owner, instance, erc20, time_now)
}

#[test]
fn test_deploy() {
    let (_env, owner, instance, _, time_now) = deploy();
    let time_now_u256: U256 = U256::from(time_now);
    assert_eq!(instance.admin(), Key::from(owner));
    let t: U256 = (time_now_u256 / WEEK) * WEEK;
    assert_eq!(instance.start_time(), t);
    assert_eq!(instance.last_token_time(), t);
}

#[test]
fn test_checkpoint_token() {
    let (_, owner, instance, _, time_now) = deploy();
    instance.checkpoint_token(owner, time_now);
}

#[test]
fn test_ve_for_at() {
    let (env, owner, instance, _, time_now) = deploy();
    let time_now_u256: U256 = U256::from(time_now);
    let timestamp: U256 = time_now_u256;
    TestContract::new(
        &env,
        TEST_SESSION_CODE_WASM,
        TEST_SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(VE_FOR_AT),
            "package_hash" => Key::Hash(instance.package_hash()),
            "user" => Key::from(owner),
            "timestamp" => timestamp
        },
        time_now,
    );
    let _ret: U256 = env.query_account_named_key(owner, &[VE_FOR_AT.into()]);
    assert!(_ret/TEN_E_NINE<=1181.into() && _ret/TEN_E_NINE>=1171.into(), "Invalid default ve value"); //depends on time
}

#[test]
fn test_checkpoint_total_supply() {
    let (_, owner, instance, _, time_now) = deploy();
    instance.checkpoint_total_supply(owner, time_now);
}

#[test]
fn test_claim() {
    let (env, owner, instance, _, time_now) = deploy();
    TestContract::new(
        &env,
        SESSION_CODE_WASM,
        SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(CLAIM),
            "package_hash" => Key::Hash(instance.package_hash()),
            "addr" => None::<Key>
        },
        time_now,
    );
    let ret: U256 = env.query_account_named_key(owner, &[CLAIM.into()]);
    assert_eq!(ret, 0.into(), "Invalid default claim value");
}

#[test]
fn test_claim_many() {
    let (env, owner, instance, _, time_now) = deploy();
    let receivers: Vec<String> = vec![
        env.next_user().to_formatted_string(),
        env.next_user().to_formatted_string(),
        env.next_user().to_formatted_string(),
    ];
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
        time_now,
    );
    let ret: bool = env.query_account_named_key(owner, &[CLAIM_MANY.into()]);
    assert!(ret, "Claim should come true");
}

#[test]
fn test_burn() {
    let (env, owner, instance, erc20, time_now) = deploy();
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
        time_now,
    );
    let ret: bool = env.query_account_named_key(owner, &[BURN.into()]);
    assert!(ret, "Claim should come true");
}

#[test]
fn test_commit_admin() {
    let (env, owner, instance, _, time_now) = deploy();
    let addr: Key = Key::Account(env.next_user());
    instance.commit_admin(owner, time_now, addr);
    assert_eq!(instance.future_admin(), addr);
}

#[test]
fn test_apply_admin() {
    let (env, owner, instance, _, time_now) = deploy();
    let addr: Key = Key::Account(env.next_user());
    instance.commit_admin(owner, time_now, addr);
    instance.apply_admin(owner, time_now);
    assert_eq!(instance.admin(), addr);
}

#[test]
fn test_toggle_allow_checkpoint_token() {
    let (_, owner, instance, _, time_now) = deploy();
    instance.toggle_allow_checkpoint_token(owner, time_now);
    let can_checkpoint_token: bool = instance.key_value("can_checkpoint_token".into());
    assert!(can_checkpoint_token, "Cannot checkpoint");
}

#[test]
fn test_kill_me() {
    let (_, owner, instance, erc20, time_now) = deploy();
    erc20.call_contract(
        owner,
        "mint",
        runtime_args! {
            "to" => Key::Hash(instance.package_hash()),
            "amount" => U256::from(10000)
        },
        time_now,
    );
    instance.kill_me(owner, time_now);
    let is_killed: bool = instance.key_value("is_killed".into());
    assert!(is_killed, "Contract not killed");
}

#[test]
fn test_recover_balance() {
    let (env, owner, instance, erc20, time_now) = deploy();
    let coin: Key = Key::Hash(erc20.package_hash());
    erc20.call_contract(
        owner,
        "mint",
        runtime_args! {
            "to" => Key::Hash(instance.package_hash()),
            "amount" => U256::from(10000)
        },
        time_now,
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
        time_now,
    );
    let ret: bool = env.query_account_named_key(owner, &[RECOVER_BALANCE.into()]);
    assert!(ret, "Balance recovered should be true");
}
