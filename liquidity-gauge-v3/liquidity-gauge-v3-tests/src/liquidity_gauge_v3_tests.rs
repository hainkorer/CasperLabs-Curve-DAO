use crate::liquidity_gauge_v3_instance::LIQUIDITYGUAGEV3INSTANCEInstance;
use casper_types::{account::AccountHash, runtime_args, Key, RuntimeArgs, U128, U256};
use casperlabs_test_env::{TestContract, TestEnv};
use common::keys::*;
//Const
pub const TEN_E_NINE: u128 = 1000000000;
const NAME: &str = "LiquidityGaugeV3";
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
// Liquidity Guage V3

fn deploy() -> (TestEnv, AccountHash, TestContract) {
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

    let liquidity_gauge_v3_instance = LIQUIDITYGUAGEV3INSTANCEInstance::new_deploy(
        &env,
        NAME,
        owner,
        Key::Hash(erc20.package_hash()),
        Key::Hash(minter.package_hash()),
        Key::Account(owner),
    );
    // For Minting Purpose
    let to = Key::Hash(liquidity_gauge_v3_instance.package_hash());
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
    let addr1: Key = Key::Hash(liquidity_gauge_v3_instance.package_hash());
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
    (env, owner, liquidity_gauge_v3_instance)
}

#[test]
fn test_deploy() {
    let (_, _, _) = deploy();
}
#[test]
fn test_commit_transfer_ownership() {
    let (env, owner, contract) = deploy();
    let contract = LIQUIDITYGUAGEV3INSTANCEInstance::instance(contract);
    let addr = Key::from(env.next_user());
    contract.commit_transfer_ownership(owner, addr);
    assert_eq!(contract.future_admin(), addr);
}
#[test]
fn test_accept_transfer_ownership() {
    let (env, owner, contract) = deploy();
    let contract = LIQUIDITYGUAGEV3INSTANCEInstance::instance(contract);
    let addr = env.next_user();
    contract.commit_transfer_ownership(owner, Key::from(addr));
    assert_eq!(contract.future_admin(), Key::from(addr));
    assert_eq!(contract.admin(), owner.into());
    contract.accept_transfer_ownership(addr);
    assert_eq!(contract.admin(), addr.into());
}
#[test]
fn test_set_killed() {
    let (_, owner, contract) = deploy();
    let contract = LIQUIDITYGUAGEV3INSTANCEInstance::instance(contract);
    let is_killed: bool = true;
    contract.set_killed(owner, is_killed);
    assert_eq!(contract.is_killed(), is_killed);
}
#[test]
fn test_increase_allowance() {
    let (env, owner, contract) = deploy();
    let contract = LIQUIDITYGUAGEV3INSTANCEInstance::instance(contract);
    let spender = env.next_user();
    assert_eq!(contract.allowance(owner, spender), 0.into());
    let amount: U256 = 50000000.into();
    contract.increase_allowance(owner, Key::from(spender), amount);
    assert_eq!(contract.allowance(owner, spender), amount);
}
#[test]
fn test_decrease_allowance() {
    let (env, owner, contract) = deploy();
    let contract = LIQUIDITYGUAGEV3INSTANCEInstance::instance(contract);
    let spender = env.next_user();
    let approve_amount: U256 = 500000.into();
    contract.approve(owner, Key::from(spender), approve_amount);
    assert_eq!(contract.allowance(owner, spender), approve_amount);
    let amount: U256 = 100000.into();
    contract.decrease_allowance(owner, Key::from(spender), amount);
    assert_eq!(contract.allowance(owner, spender), 400000.into());
}
#[test]
fn test_approve() {
    let (env, owner, contract) = deploy();
    let contract = LIQUIDITYGUAGEV3INSTANCEInstance::instance(contract);
    let spender = env.next_user();
    let approve_amount: U256 = 500000.into();
    contract.approve(owner, Key::from(spender), approve_amount);
    assert_eq!(contract.allowance(owner, spender), approve_amount);
}
#[test]
fn test_decimals() {
    let (env, owner, contract) = deploy();

    TestContract::new(
        &env,
        "liquidity_gauge_v3_session_code.wasm",
        "SessionCode",
        owner,
        runtime_args! {
            "entrypoint" => String::from(DECIMALS),
            "package_hash" => Key::Hash(contract.package_hash())
        },
        0,
    );

    let ret: u8 = env.query_account_named_key(owner, &[DECIMALS.into()]);
    assert_eq!(ret, 9);
}
#[test]
fn test_integrate_checkpoint() {
    let (env, owner, contract) = deploy();

    TestContract::new(
        &env,
        "liquidity_gauge_v3_session_code.wasm",
        "SessionCode",
        owner,
        runtime_args! {
            "entrypoint" => String::from(INTEGRATE_CHECKPOINT),
            "package_hash" => Key::Hash(contract.package_hash())
        },
        0,
    );

    let ret: U256 = env.query_account_named_key(owner, &[INTEGRATE_CHECKPOINT.into()]);
    assert_eq!(ret, 100000.into());
}
#[test]
fn test_reward_contract() {
    let (env, owner, contract) = deploy();

    TestContract::new(
        &env,
        "liquidity_gauge_v3_session_code.wasm",
        "SessionCode",
        owner,
        runtime_args! {
            "entrypoint" => String::from(REWARD_CONTRACT),
            "package_hash" => Key::Hash(contract.package_hash())
        },
        0,
    );

    let ret: Key = env.query_account_named_key(owner, &[REWARD_CONTRACT.into()]);
    assert_eq!(
        ret,
        Key::from_formatted_str(
            "hash-0000000000000000000000000000000000000000000000000000000000000000",
        )
        .unwrap()
    );
}
#[test]
fn test_last_claim() {
    let (env, owner, contract) = deploy();

    TestContract::new(
        &env,
        "liquidity_gauge_v3_session_code.wasm",
        "SessionCode",
        owner,
        runtime_args! {
            "entrypoint" => String::from(LAST_CLAIM),
            "package_hash" => Key::Hash(contract.package_hash())
        },
        0,
    );

    let ret: U256 = env.query_account_named_key(owner, &[LAST_CLAIM.into()]);
    assert_eq!(ret, 0.into());
}
#[test]
fn test_claimed_reward() {
    let (env, owner, contract) = deploy();
    let addr = env.next_user();
    let token = env.next_user();
    TestContract::new(
        &env,
        "liquidity_gauge_v3_session_code.wasm",
        "SessionCode",
        owner,
        runtime_args! {
            "entrypoint" => String::from(CLAIMED_REWARD),
            "package_hash" => Key::Hash(contract.package_hash()),
            "addr"=>Key::from(addr),
            "token"=>Key::from(token)
        },
        0,
    );
    let ret: U256 = env.query_account_named_key(owner, &[CLAIMED_REWARD.into()]);
    assert_eq!(ret, 0.into());
}
#[test]
fn test_deposit() {
    let (_, owner, contract) = deploy();
    let contract = LIQUIDITYGUAGEV3INSTANCEInstance::instance(contract);
    let value: U256 = 1000.into();
    let addr: Key = Key::from_formatted_str(
        "hash-0000000000000000000000010000000000000000000000000000000000020000",
    )
    .unwrap();
    contract.deposit(owner, value, Some(addr), Some(false));
}
#[test]
fn test_withdraw() {
    let (_, owner, contract) = deploy();
    let contract = LIQUIDITYGUAGEV3INSTANCEInstance::instance(contract);
    let value: U256 = 1000.into();
    let addr: Key = Key::from_formatted_str(
        "hash-0000000000000000000000010000000000000000000000000000000000020000",
    )
    .unwrap();
    contract.deposit(owner, value, Some(addr), Some(false));
    contract.withdraw(owner, value, Some(false));
}
#[test]
fn test_transfer() {
    let (env, owner, contract) = deploy();
    let contract = LIQUIDITYGUAGEV3INSTANCEInstance::instance(contract);
    let value: U256 = 1000000.into();
    let recipient: Key = Key::from_formatted_str(
        "hash-0000000000000000000000010000000000000000000000000000000000020000",
    )
    .unwrap();
    let amount: U256 = 100000.into();
    contract.deposit(owner, value, Some(Key::Account(owner)), Some(false));
    TestContract::new(
        &env,
        "liquidity_gauge_v3_session_code.wasm",
        "SessionCode",
        owner,
        runtime_args! {
            "entrypoint" => String::from(TRANSFER),
            "package_hash" => Key::Hash(contract.package_hash()),
            "recipient"=>recipient,
            "amount"=>amount
        },
        0,
    );
    let ret: Result<(), u32> = env.query_account_named_key(owner, &[TRANSFER.into()]);
    match ret {
        Ok(()) => {}
        Err(e) => panic!("Transfer Failed ERROR:{}", e),
    }
}
#[test]
fn test_transfer_from() {
    let (env, owner, contract) = deploy();
    let contract = LIQUIDITYGUAGEV3INSTANCEInstance::instance(contract);
    let recipient: Key = Key::from_formatted_str(
        "hash-0000000000000000000000010000000000000000000000000000000000020000",
    )
    .unwrap();
    let spender = env.next_user();
    let amount: U256 = 100000.into();
    contract.deposit(owner, amount, Some(Key::from(owner)), Some(false));
    contract.approve(owner, Key::from(spender), amount);
    TestContract::new(
        &env,
        "liquidity_gauge_v3_session_code.wasm",
        "SessionCode",
        spender,
        runtime_args! {
            "entrypoint" => String::from(TRANSFER_FROM),
            "package_hash" => Key::Hash(contract.package_hash()),
            "owner"=>Key::from(owner),
            "recipient"=>recipient,
            "amount"=>amount
        },
        0,
    );
    let ret: Result<(), u32> = env.query_account_named_key(spender, &[TRANSFER_FROM.into()]);
    match ret {
        Ok(()) => {}
        Err(e) => panic!("Transfer From Failed ERROR:{}", e),
    }
}
#[test]
fn test_claimable_tokens() {
    let (env, owner, contract) = deploy();
    let contract = LIQUIDITYGUAGEV3INSTANCEInstance::instance(contract);
    let addr = env.next_user();
    TestContract::new(
        &env,
        "liquidity_gauge_v3_session_code.wasm",
        "SessionCode",
        owner,
        runtime_args! {
            "entrypoint" => String::from(CLAIMABLE_TOKENS),
            "package_hash" => Key::Hash(contract.package_hash()),
            "addr"=>Key::from(addr)
        },
        0,
    );
    let ret: U256 = env.query_account_named_key(owner, &[CLAIMABLE_TOKENS.into()]);
    assert_eq!(ret, 0.into());
}
#[test]
fn test_claimable_reward_write() {
    let (env, owner, contract) = deploy();
    let contract = LIQUIDITYGUAGEV3INSTANCEInstance::instance(contract);
    let addr = env.next_user();
    let token = env.next_user();
    TestContract::new(
        &env,
        "liquidity_gauge_v3_session_code.wasm",
        "SessionCode",
        owner,
        runtime_args! {
            "entrypoint" => String::from(CLAIMEABLE_REWARD_WRITE),
            "package_hash" => Key::Hash(contract.package_hash()),
            "addr"=>Key::from(addr),
            "token"=>Key::from(token)
        },
        0,
    );
    let ret: U256 = env.query_account_named_key(owner, &[CLAIMEABLE_REWARD_WRITE.into()]);
    assert_eq!(ret, 0.into());
}
#[test]
fn test_claimable_reward() {
    let (env, owner, contract) = deploy();
    let contract = LIQUIDITYGUAGEV3INSTANCEInstance::instance(contract);
    let addr = env.next_user();
    let token = env.next_user();
    TestContract::new(
        &env,
        "liquidity_gauge_v3_session_code.wasm",
        "SessionCode",
        owner,
        runtime_args! {
            "entrypoint" => String::from(CLAIMEABLE_REWARD),
            "package_hash" => Key::Hash(contract.package_hash()),
            "addr"=>Key::from(addr),
            "token"=>Key::from(token)
        },
        0,
    );
    let ret: U256 = env.query_account_named_key(owner, &[CLAIMEABLE_REWARD.into()]);
    assert_eq!(ret, 0.into());
}
#[test]
fn test_user_checkpoint() {
    let (env, owner, contract) = deploy();
    let contract = LIQUIDITYGUAGEV3INSTANCEInstance::instance(contract);
    TestContract::new(
        &env,
        "liquidity_gauge_v3_session_code.wasm",
        "SessionCode",
        owner,
        runtime_args! {
            "entrypoint" => String::from(USER_CHECKPOINT),
            "package_hash" => Key::Hash(contract.package_hash()),
            "addr"=>Key::from(owner),
        },
        0,
    );
    let ret: bool = env.query_account_named_key(owner, &[USER_CHECKPOINT.into()]);
    assert!(ret);
}
#[test]
fn test_set_rewards_receiver() {
    let (env, owner, contract) = deploy();
    let contract = LIQUIDITYGUAGEV3INSTANCEInstance::instance(contract);
    let receiver: Key = Key::from(env.next_user());
    contract.set_rewards_receiver(owner, receiver);
}
#[test]
fn test_claim_rewards() {
    let (env, owner, contract) = deploy();
    let contract = LIQUIDITYGUAGEV3INSTANCEInstance::instance(contract);
    let addr: Key = Key::from(env.next_user());
    let receiver: Key = Key::from_formatted_str(
        "hash-0000000000000000000000000000000000000000000000000000000000000000",
    )
    .unwrap();
    contract.claim_rewards(owner, Some(addr), Some(receiver))
}