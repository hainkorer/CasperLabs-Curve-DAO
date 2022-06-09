use casper_types::{account::AccountHash, Key, U256};
use test_env::{TestContract, TestEnv};

use crate::curve_token_v3_instance::CURVETOKENV3Instance;

const NAME: &str = "CRVTokenV3";
const SYMBOL: &str = "CRV3";

fn deploy() -> (
    TestEnv,
    CURVETOKENV3Instance,
    AccountHash,
    CURVETOKENV3Instance,
    CURVETOKENV3Instance,
) {
    let env = TestEnv::new();
    let owner = env.next_user();
    let token: TestContract =
        CURVETOKENV3Instance::new(&env, NAME, owner, NAME.to_string(), SYMBOL.to_string());
    let test_contract: TestContract =
        CURVETOKENV3Instance::proxy(&env, Key::Hash(token.contract_hash()), owner);
    let test_contract2: TestContract =
        CURVETOKENV3Instance::proxy2(&env, Key::Hash(token.contract_hash()), owner);

    (
        env,
        CURVETOKENV3Instance::instance(token),
        owner,
        CURVETOKENV3Instance::instance(test_contract),
        CURVETOKENV3Instance::instance(test_contract2),
    )
}

#[test]
fn test_deploy() {
    let (env, token, owner, _, _) = deploy();
}
#[test]
fn test_decimals() {
    let (env, token, owner, proxy, _) = deploy();
    proxy.decimals(owner);
    let res: U256 = proxy.result();
    assert_eq!(res, 9.into());
}
#[test]
fn test_set_minter() {
    let (env, token, owner, _, _) = deploy();
    let _minter_arg: Key = Key::Account(owner);
    token.set_minter(owner, _minter_arg);
}
#[test]
fn test_mint() {
    let (env, token, owner, proxy, _) = deploy();
    let _to_arg: Key = Key::from_formatted_str(
        "hash-0000000000000000000000010000000000000000000000000000000000020000".into(),
    )
    .unwrap();
    token.set_minter(owner, proxy.package_hash().into());
    let _value_arg: U256 = 2000000000.into();
    proxy.mint(owner, _to_arg, _value_arg);
    let res: bool = proxy.result();
    assert_eq!(res, true);
}
#[test]
fn test_transfer() {
    let (env, token, owner, proxy, _) = deploy();
    let _to_arg: Key = Key::from_formatted_str(
        "hash-0000000000000000000000010000000000000000000000000000000000020000".into(),
    )
    .unwrap();
    token.set_minter(owner, proxy.package_hash().into());
    let _value_arg: U256 = 2000000000.into();
    proxy.mint(owner, proxy.package_hash().into(), _value_arg);
    proxy.transfer(owner, _to_arg, _value_arg);
    let res: Result<(), u32> = proxy.result();
    match res {
        Ok(()) => {}
        Err(e) => assert!(false, "Transfer Failed ERROR:{}", e),
    }
}
#[test]
fn test_transfer_from() {
    let (env, token, owner, proxy, proxy2) = deploy();
    let to_arg: Key = Key::from_formatted_str(
        "hash-0000000000000000000000010000000000000000000000000000000000020000".into(),
    )
    .unwrap();
    let mint_to: Key = Key::from_formatted_str(
        "hash-0000000000000000000000010000000000000000000000000000000000030000".into(),
    )
    .unwrap();
    token.set_minter(owner, proxy.package_hash().into());
    let _value_arg: U256 = 2000000000.into();
    proxy.mint(owner, proxy.package_hash().into(), _value_arg);
    proxy.approve(owner, proxy2.package_hash().into(), _value_arg);
    proxy2.transfer_from(owner, proxy.package_hash().into(), to_arg, _value_arg);
    let res: Result<(), u32> = proxy2.result();
    match res {
        Ok(()) => {}
        Err(e) => assert!(false, "transfer_from Failed ERROR:{}", e),
    }
}
#[test]
fn test_approve() {
    let (env, token, owner, proxy, _) = deploy();
    let spender: Key = Key::from_formatted_str(
        "hash-0000000000000000000000010000000000000000000000000000000000020000".into(),
    )
    .unwrap();
    token.set_minter(owner, proxy.package_hash().into());
    let _value_arg: U256 = 2000000000.into();
    let approve_amount: U256 = 1000000000.into();
    proxy.mint(owner, proxy.package_hash().into(), _value_arg);
    proxy.approve(owner, spender, approve_amount);
}
#[test]
fn test_increase_allowance() {
    let (env, token, owner, proxy, _) = deploy();
    let spender: Key = Key::from_formatted_str(
        "hash-0000000000000000000000010000000000000000000000000000000000020000".into(),
    )
    .unwrap();
    token.set_minter(owner, proxy.package_hash().into());
    let _value_arg: U256 = 2000000000.into();
    let approve_amount: U256 = 1000000000.into();
    proxy.mint(owner, proxy.package_hash().into(), _value_arg);

    proxy.increase_allowance(owner, spender, approve_amount);
    let res: Result<(), u32> = proxy.result();
    match res {
        Ok(()) => {}
        Err(e) => assert!(false, "increase_allowance Failed ERROR:{}", e),
    }
}
#[test]
fn test_decrease_allowance() {
    let (env, token, owner, proxy, _) = deploy();
    let spender: Key = Key::from_formatted_str(
        "hash-0000000000000000000000010000000000000000000000000000000000020000".into(),
    )
    .unwrap();
    token.set_minter(owner, proxy.package_hash().into());
    let _value_arg: U256 = 2000000000.into();
    let decrease_amount: U256 = 1000000000.into();
    proxy.mint(owner, proxy.package_hash().into(), _value_arg);
    proxy.approve(owner, spender, _value_arg);

    proxy.decrease_allowance(owner, spender, decrease_amount);
    let res: Result<(), u32> = proxy.result();
    match res {
        Ok(()) => {}
        Err(e) => assert!(false, "decrease_allowance Failed ERROR:{}", e),
    }
}
#[test]
fn test_burn_from() {
    let (env, token, owner, proxy, _) = deploy();
    let burn_from: Key = Key::from_formatted_str(
        "hash-0000000000000000000000010000000000000000000000000000000000020000".into(),
    )
    .unwrap();

    token.set_minter(owner, proxy.package_hash().into());
    let mint_amount: U256 = 2000000000.into();
    let burn_amount: U256 = 1000000000.into();
    proxy.mint(owner, proxy.package_hash().into(), mint_amount);
    proxy.burn_from(owner, proxy.package_hash().into(), burn_amount);
    let res: bool = proxy.result();
    assert_eq!(res, true);
}
#[test]
fn test_name() {
    let (env, token, owner, _, _) = deploy();
    let burn_from: Key = Key::from_formatted_str(
        "hash-0000000000000000000000010000000000000000000000000000000000020000".into(),
    )
    .unwrap();
    token.set_name(owner, "curve-token-v3".to_string(), "crvtok3".to_string());
}
