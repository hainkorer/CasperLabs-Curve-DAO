use crate::erc20_crv_instance::ERC20CRVInstance;
use casper_types::{account::AccountHash, runtime_args, Key, RuntimeArgs, U256};
use casperlabs_test_env::{TestContract, TestEnv};
use common::keys::*;
pub const TEN_E_NINE: u128 = 1000000000;
const MILLI_SECONDS_IN_DAY: u64 = 86_400_000;
fn deploy() -> (TestEnv, AccountHash, ERC20CRVInstance) {
    let env = TestEnv::new();
    let owner = env.next_user();
    let instance = ERC20CRVInstance::new_deploy(
        &env,
        "ERC20CRV",
        owner,
        "ERC20CRV".to_string(),
        "erc20_crv".to_string(),
        9_u8,
    );
    (env, owner, instance)
}

#[test]
fn test_deploy() {
    let (env, owner, contract) = deploy();
    assert_eq!(contract.get_init_supply(),1303030303000000000_i64.into());
    assert_eq!(contract.get_admin(),Key::from(owner));
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
        ERC20CRVInstance::now(),
    );

    let ret: U256 = env.query_account_named_key(owner, &[BALANCE_OF.into()]);
    assert_eq!(ret,1303030303000000000_i64.into());
    // assert_eq!(contract.get_start_epoch_time(),0.into());  //epcoh time will be last year from the current time
   
}

#[test]
fn burn() {
    let (env, owner, contract) = deploy();
    TestContract::new(
        &env,
        TEST_SESSION_CODE_WASM,
        TEST_SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(TOTAL_SUPPLY),
            "package_hash" => Key::Hash(contract.package_hash())
        },
        ERC20CRVInstance::now(),
    );
    let mut ret: U256 = env.query_account_named_key(owner, &[TOTAL_SUPPLY.into()]);
    assert_eq!(ret, 1303030303000000000_i64.into());
    contract.burn(owner,  1303030303000000000_i64.into());
    TestContract::new(
        &env,
        TEST_SESSION_CODE_WASM,
        TEST_SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(TOTAL_SUPPLY),
            "package_hash" => Key::Hash(contract.package_hash())
        },
        ERC20CRVInstance::now(),
    );
   ret=env.query_account_named_key(owner, &[TOTAL_SUPPLY.into()]);
    assert_eq!(ret, 0.into());
}
#[test]
fn set_admin() {
    let (env, owner, contract) = deploy();
    let admin: Key = Key::from(env.next_user());
    contract.set_admin(owner, admin);
}
#[test]
fn test_set_minter() {
    let (env, owner, contract) = deploy();
    let minter = Key::from(env.next_user());
    contract.set_minter(owner, minter);
}
#[test]
fn test_update_mining_parameters() {
    let (_, owner, contract) = deploy();
    contract.update_mining_parameters(owner);
    assert_eq!(contract.get_rate(),8714335457889396_i64.into());
}
#[test]
fn test_start_epoch_time_write() {
    let (env, owner, contract) = deploy();
    TestContract::new(
        &env,
        "erc20-crv-session-code.wasm",
        "SessionCode",
        owner,
        runtime_args! {
            "entrypoint" => String::from(START_EPOCH_TIME_WRITE),
            "package_hash" => Key::Hash(contract.package_hash())
        },
        ERC20CRVInstance::now(),
    );
    let epcoh_time:U256=U256::from(ERC20CRVInstance::now()+MILLI_SECONDS_IN_DAY-31536000000);
    let ret: U256 = env.query_account_named_key(owner, &[START_EPOCH_TIME_WRITE.into()]);
    assert_eq!(ret/60000, epcoh_time/60000);
}

#[test]
fn test_future_epoch_time_write() {
    let (env, owner, contract) = deploy();
    TestContract::new(
        &env,
        "erc20-crv-session-code.wasm",
        "SessionCode",
        owner,
        runtime_args! {
            "entrypoint" => String::from(FUTURE_EPOCH_TIME_WRITE),
            "package_hash" => Key::Hash(contract.package_hash())
        },
        ERC20CRVInstance::now(),
    );
    let futrue_epcoh_time:U256=U256::from(ERC20CRVInstance::now()+MILLI_SECONDS_IN_DAY);
    let ret: U256 = env.query_account_named_key(owner, &[FUTURE_EPOCH_TIME_WRITE.into()]);
    assert_eq!(ret/60000, futrue_epcoh_time/60000);
}
#[test]
fn test_available_supply() {
    let (env, owner, contract) = deploy();
    TestContract::new(
        &env,
        TEST_SESSION_CODE_WASM,
        TEST_SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(AVAILABLE_SUPPLY),
            "package_hash" => Key::Hash(contract.package_hash())
        },
        ERC20CRVInstance::now(),
    );
    let available_supply=contract.get_start_epoch_supply()+(U256::from(ERC20CRVInstance::now())-contract.get_start_epoch_time())*contract.get_rate();
    let ret: U256 = env.query_account_named_key(owner, &[AVAILABLE_SUPPLY.into()]);
    assert_eq!(ret, available_supply);
}
#[test]
fn test_total_supply() {
    let (env, owner, contract) = deploy();
    TestContract::new(
        &env,
        TEST_SESSION_CODE_WASM,
        TEST_SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(TOTAL_SUPPLY),
            "package_hash" => Key::Hash(contract.package_hash())
        },
        ERC20CRVInstance::now(),
    );
    let ret: U256 = env.query_account_named_key(owner, &[TOTAL_SUPPLY.into()]);
    assert_eq!(ret, 1303030303000000000_i64.into());
}
#[test]
fn test_mintable_in_timeframe() {
    let (env, owner, contract) = deploy();
    contract.update_mining_parameters(owner);
    let start: U256 = U256::from(ERC20CRVInstance::now());
    let end: U256 =U256::from(start+86400000);
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
        ERC20CRVInstance::now(),
    );

    let _ret: U256 = env.query_account_named_key(owner, &[MINTABLE_IN_TIMEFRAME.into()]);
    // assert_eq!(ret, 1234010148955192638901447884_i128.into());
}
#[test]
fn test_mint() {
    let (env, owner, contract) = deploy();
    let to: Key = Key::from(env.next_user());
    let amount: U256 = U256::from(10*TEN_E_NINE);
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
        ERC20CRVInstance::now()+86400000
    );

    let ret: bool = env.query_account_named_key(owner, &[MINT.into()]);
    assert!(ret);
}
#[test]
fn test_increase_allowance() {
    let (env, owner, contract) = deploy();
    let spender: Key = Key::from(env.next_user());
    let amount: U256 = U256::from(100*TEN_E_NINE);
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
    let (env, owner, contract) = deploy();
    let recipient:Key=Key::from(env.next_user());
    let amount:U256=U256::from(100*TEN_E_NINE);
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
        ERC20CRVInstance::now(),
    );
    let ret: U256 = env.query_account_named_key(owner, &[BALANCE_OF.into()]);
    assert_eq!(ret,1303030203000000000_i64.into());
}
#[test]
fn test_transfer_from() {
    let (env, owner, contract) = deploy();
    let spender:AccountHash=env.next_user();
    let recipient:Key=Key::from(env.next_user());
    let amount:U256=U256::from(100*TEN_E_NINE);
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
        ERC20CRVInstance::now(),
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
        ERC20CRVInstance::now(),
    );
    let ret: U256 = env.query_account_named_key(owner, &[BALANCE_OF.into()]);
    assert_eq!(ret,1303030203000000000_i64.into());
 
}
#[test]
fn test_allowance() {
    let (env, owner, contract) = deploy();
    let spender:AccountHash=env.next_user();
    let recipient:Key=Key::from(env.next_user());
    let amount:U256=U256::from(100*TEN_E_NINE);
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
        ERC20CRVInstance::now(),
    );
    let ret: U256 = env.query_account_named_key(owner, &[ALLOWANCE.into()]);
    assert_eq!(ret,U256::from(100*TEN_E_NINE));
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
        ERC20CRVInstance::now(),
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
        ERC20CRVInstance::now(),
    );
    let ret: U256 = env.query_account_named_key(owner, &[ALLOWANCE.into()]);
    assert_eq!(ret,U256::from(0));
    
 
}


