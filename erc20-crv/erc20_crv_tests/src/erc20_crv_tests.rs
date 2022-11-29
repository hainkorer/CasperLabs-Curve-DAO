use crate::erc20_crv_instance::ERC20CRVInstance;
use casper_types::{account::AccountHash, runtime_args, Key, RuntimeArgs, U256};
use casperlabs_test_env::{TestContract, TestEnv};
use common::keys::*;
use erc20_crv::data::*;
pub const TEN_E_NINE: u128 = 1000000000;
const MILLI_SECONDS_IN_DAY: u64 = 86_400_000;
fn deploy() -> (TestEnv, AccountHash, ERC20CRVInstance, u64) {
    let env = TestEnv::new();
    let owner = env.next_user();
    let time_now: u64 = ERC20CRVInstance::now();
    let instance = ERC20CRVInstance::new_deploy(
        &env,
        "ERC20CRV",
        owner,
        "ERC20CRV".to_string(),
        "erc20_crv".to_string(),
        9_u8,
        time_now,
    );
    (env, owner, instance, time_now)
}

#[test]
fn test_deploy() {
    let (env, owner, contract, time_now) = deploy();
    assert_eq!(contract.get_init_supply(), 1303030303000000000_i64.into());
    assert_eq!(contract.get_admin(), Key::from(owner));
    TestContract::new(
        &env,
        TEST_SESSION_CODE_WASM,
        TEST_SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(BALANCE_OF),
            "package_hash" => Key::Hash(contract.package_hash()),
            "owner"=>Key::from(owner)
        },
        time_now,
    );

    let ret: U256 = env.query_account_named_key(owner, &[BALANCE_OF.into()]);
    assert_eq!(ret, 1303030303000000000_i64.into());
    let start_epoch_time: U256 = U256::from(time_now) + INFLATION_DELAY - RATE_REDUCTION_TIME;
    assert_eq!(contract.get_start_epoch_time(), start_epoch_time);
}

#[test]
fn burn() {
    let (env, owner, contract, time_now) = deploy();
    TestContract::new(
        &env,
        TEST_SESSION_CODE_WASM,
        TEST_SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(TOTAL_SUPPLY),
            "package_hash" => Key::Hash(contract.package_hash())
        },
        time_now,
    );
    let mut ret: U256 = env.query_account_named_key(owner, &[TOTAL_SUPPLY.into()]);
    assert_eq!(ret, 1303030303000000000_i64.into());
    contract.burn(owner, 1303030303000000000_i64.into());
    TestContract::new(
        &env,
        TEST_SESSION_CODE_WASM,
        TEST_SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(TOTAL_SUPPLY),
            "package_hash" => Key::Hash(contract.package_hash())
        },
        time_now,
    );
    ret = env.query_account_named_key(owner, &[TOTAL_SUPPLY.into()]);
    assert_eq!(ret, 0.into());
}
#[test]
fn set_admin() {
    let (env, owner, contract, _) = deploy();
    let admin: Key = Key::from(env.next_user());
    contract.set_admin(owner, admin);
}
#[test]
fn test_set_minter() {
    let (env, owner, contract, _) = deploy();
    let minter = Key::from(env.next_user());
    contract.set_minter(owner, minter);
}
#[test]
fn test_update_mining_parameters() {
    let (_, owner, contract, time_now) = deploy();
    contract.update_mining_parameters(owner, time_now + MILLI_SECONDS_IN_DAY);
    assert_eq!(contract.get_rate(), 8714335.into());
}
#[test]
fn test_start_epoch_time_write() {
    let (env, owner, contract, time_now) = deploy();
    TestContract::new(
        &env,
        "erc20-crv-session-code.wasm",
        "SessionCode",
        owner,
        runtime_args! {
            "entrypoint" => String::from(START_EPOCH_TIME_WRITE),
            "package_hash" => Key::Hash(contract.package_hash())
        },
        time_now,
    );
    let epcoh_time: U256 = U256::from(time_now + MILLI_SECONDS_IN_DAY - 31536000000);
    let ret: U256 = env.query_account_named_key(owner, &[START_EPOCH_TIME_WRITE.into()]);
    assert_eq!(ret / 60000, epcoh_time / 60000);
}

#[test]
fn test_future_epoch_time_write() {
    let (env, owner, contract, time_now) = deploy();
    TestContract::new(
        &env,
        "erc20-crv-session-code.wasm",
        "SessionCode",
        owner,
        runtime_args! {
            "entrypoint" => String::from(FUTURE_EPOCH_TIME_WRITE),
            "package_hash" => Key::Hash(contract.package_hash())
        },
        time_now,
    );
    let futrue_epcoh_time: U256 = U256::from(time_now + MILLI_SECONDS_IN_DAY);
    let ret: U256 = env.query_account_named_key(owner, &[FUTURE_EPOCH_TIME_WRITE.into()]);
    assert_eq!(ret / 60000, futrue_epcoh_time / 60000);
}
#[test]
fn test_available_supply() {
    let (env, owner, contract, time_now) = deploy();
    TestContract::new(
        &env,
        TEST_SESSION_CODE_WASM,
        TEST_SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(AVAILABLE_SUPPLY),
            "package_hash" => Key::Hash(contract.package_hash())
        },
        time_now,
    );
    let available_supply = contract.get_start_epoch_supply()
        + (U256::from(time_now) - contract.get_start_epoch_time()) * contract.get_rate();
    let ret: U256 = env.query_account_named_key(owner, &[AVAILABLE_SUPPLY.into()]);
    assert_eq!(ret, available_supply);
}
#[test]
fn test_total_supply() {
    let (env, owner, contract, time_now) = deploy();
    TestContract::new(
        &env,
        TEST_SESSION_CODE_WASM,
        TEST_SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(TOTAL_SUPPLY),
            "package_hash" => Key::Hash(contract.package_hash())
        },
        time_now,
    );
    let ret: U256 = env.query_account_named_key(owner, &[TOTAL_SUPPLY.into()]);
    assert_eq!(ret, 1303030303000000000_i64.into());
}
#[test]
fn test_mintable_in_timeframe() {
    let (env, owner, contract, time_now) = deploy();
    contract.update_mining_parameters(owner, time_now + MILLI_SECONDS_IN_DAY);
    let start: U256 = U256::from(time_now);
    let end: U256 = start + MILLI_SECONDS_IN_DAY;
    TestContract::new(
        &env,
        TEST_SESSION_CODE_WASM,
        TEST_SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(MINTABLE_IN_TIMEFRAME),
            "package_hash" => Key::Hash(contract.package_hash()),
            "start"=>start,
            "end"=>end
        },
        time_now,
    );

    let _ret: U256 = env.query_account_named_key(owner, &[MINTABLE_IN_TIMEFRAME.into()]);
    assert_eq!(_ret, 752918544000000_i128.into());
}
#[test]
fn test_mint() {
    let (env, owner, contract, time_now) = deploy();
    let to: Key = Key::from(env.next_user());
    let amount: U256 = U256::from(10 * TEN_E_NINE);
    let minter = Key::from(owner);
    contract.set_minter(owner, minter);
    TestContract::new(
        &env,
        "erc20-crv-session-code.wasm",
        "SessionCode",
        owner,
        runtime_args! {
            "entrypoint" => String::from(MINT),
            "package_hash" => Key::Hash(contract.package_hash()),
            "to"=>to,
            "amount"=>amount
        },
        time_now + MILLI_SECONDS_IN_DAY + 2000,
    );

    let ret: bool = env.query_account_named_key(owner, &[MINT.into()]);
    assert!(ret);
}
#[test]
fn test_increase_allowance() {
    let (env, owner, contract, _) = deploy();
    let spender: Key = Key::from(env.next_user());
    let amount: U256 = U256::from(100 * TEN_E_NINE);
    TestContract::new(
        &env,
        "erc20-crv-session-code.wasm",
        "SessionCode",
        owner,
        runtime_args! {
            "entrypoint" => String::from(INCREASE_ALLOWANCE),
            "package_hash" => Key::Hash(contract.package_hash()),
            "spender"=>spender,
            "amount"=>amount
        },
        0,
    );

    let ret: Result<(), u32> = env.query_account_named_key(owner, &[INCREASE_ALLOWANCE.into()]);
    match ret {
        Ok(()) => {}
        Err(e) => panic!("Increase Allowance Failed ERROR:{}", e),
    }
}
#[test]
fn test_transfer() {
    let (env, owner, contract, time_now) = deploy();
    let recipient: Key = Key::from(env.next_user());
    let amount: U256 = U256::from(100 * TEN_E_NINE);
    TestContract::new(
        &env,
        "erc20-crv-session-code.wasm",
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
        Err(e) => panic!("Tranfer Failed ERROR:{}", e),
    }
    TestContract::new(
        &env,
        TEST_SESSION_CODE_WASM,
        TEST_SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(BALANCE_OF),
            "package_hash" => Key::Hash(contract.package_hash()),
            "owner"=>Key::from(owner)
        },
        time_now,
    );
    let ret: U256 = env.query_account_named_key(owner, &[BALANCE_OF.into()]);
    assert_eq!(ret, 1303030203000000000_i64.into());
}
#[test]
fn test_transfer_from() {
    let (env, owner, contract, time_now) = deploy();
    let spender: AccountHash = env.next_user();
    let recipient: Key = Key::from(env.next_user());
    let amount: U256 = U256::from(100 * TEN_E_NINE);
    contract.approve(owner, Key::from(spender), amount);
    TestContract::new(
        &env,
        "erc20-crv-session-code.wasm",
        "SessionCode",
        spender,
        runtime_args! {
            "entrypoint" => String::from(TRANSFER_FROM),
            "package_hash" => Key::Hash(contract.package_hash()),
            "owner"=>Key::from(owner),
            "recipient"=>recipient,
            "amount"=>amount
        },
        time_now,
    );
    let ret: Result<(), u32> = env.query_account_named_key(spender, &[TRANSFER_FROM.into()]);
    match ret {
        Ok(()) => {}
        Err(e) => panic!("Tranfer Failed ERROR:{}", e),
    }
    TestContract::new(
        &env,
        TEST_SESSION_CODE_WASM,
        TEST_SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(BALANCE_OF),
            "package_hash" => Key::Hash(contract.package_hash()),
            "owner"=>Key::from(owner)
        },
        time_now,
    );
    let ret: U256 = env.query_account_named_key(owner, &[BALANCE_OF.into()]);
    assert_eq!(ret, 1303030203000000000_i64.into());
}
#[test]
fn test_allowance() {
    let (env, owner, contract, time_now) = deploy();
    let spender: AccountHash = env.next_user();
    let recipient: Key = Key::from(env.next_user());
    let amount: U256 = U256::from(100 * TEN_E_NINE);
    contract.approve(owner, Key::from(spender), amount);
    TestContract::new(
        &env,
        TEST_SESSION_CODE_WASM,
        TEST_SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(ALLOWANCE),
            "package_hash" => Key::Hash(contract.package_hash()),
            "owner"=>Key::from(owner),
            "spender"=>Key::from(spender)
        },
        time_now,
    );
    let ret: U256 = env.query_account_named_key(owner, &[ALLOWANCE.into()]);
    assert_eq!(ret, U256::from(100 * TEN_E_NINE));
    TestContract::new(
        &env,
        "erc20-crv-session-code.wasm",
        "SessionCode",
        spender,
        runtime_args! {
            "entrypoint" => String::from(TRANSFER_FROM),
            "package_hash" => Key::Hash(contract.package_hash()),
            "owner"=>Key::from(owner),
            "recipient"=>recipient,
            "amount"=>amount
        },
        time_now,
    );
    let ret: Result<(), u32> = env.query_account_named_key(spender, &[TRANSFER_FROM.into()]);
    match ret {
        Ok(()) => {}
        Err(e) => panic!("Tranfer Failed ERROR:{}", e),
    }
    TestContract::new(
        &env,
        TEST_SESSION_CODE_WASM,
        TEST_SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(ALLOWANCE),
            "package_hash" => Key::Hash(contract.package_hash()),
            "owner"=>Key::from(owner),
            "spender"=>Key::from(spender)
        },
        time_now,
    );
    let ret: U256 = env.query_account_named_key(owner, &[ALLOWANCE.into()]);
    assert_eq!(ret, U256::from(0));
}
