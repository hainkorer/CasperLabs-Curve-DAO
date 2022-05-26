use blake2::digest::consts::U2;
use casper_types::{
    account::AccountHash, runtime_args, ContractPackageHash, Key, RuntimeArgs, URef, U256, U512,
};
use test_env::{TestContract, TestEnv};

use crate::curve_token_v1_instance::CURVETOKENV1Instance;

fn deploy() -> (TestEnv, AccountHash, TestContract) {
    let env = TestEnv::new();
    let owner = env.next_user();

    let contract = CURVETOKENV1Instance::new(
        &env,
        "CURVETOKENV1",
        owner,
        "CVTokenV1".to_string(),
        "CV1".to_string(),
        u8::from(9 as u8),
        1000000000.into(),
    );

    (env, owner, contract)
}

#[test]
fn test_deploy() {
    let (_, _, _) = deploy();
}

#[test]
fn mint_crv1() {
    let (env, owner, contract) = deploy();
    let contract = CURVETOKENV1Instance::contract_instance(contract);
    let _to_arg: Key = Key::from_formatted_str(
        "hash-0000000000000000000000010000000000000000000000000000000000020000".into(),
    )
    .unwrap();
    let _value_arg: U256 = 100000.into();
    contract.mint_crv1(owner, _to_arg, _value_arg);
}
#[test]
fn set_minter() {
    let (env, owner, contract) = deploy();
    let contract = CURVETOKENV1Instance::contract_instance(contract);
    let _minter_arg: Key = Key::Account(owner);
    contract.set_minter(owner, _minter_arg);
}
#[test]
fn burn_caller() {
    let (env, owner, contract) = deploy();
    let contract = CURVETOKENV1Instance::contract_instance(contract);
    let _value: U256 = 100.into();
    contract.burn_caller(owner, _value);
}
#[test]
fn burn_from() {
    let (env, owner, contract) = deploy();
    let contract = CURVETOKENV1Instance::contract_instance(contract);
    let _to: Key = Key::Account(owner);
    let _value: U256 = 100.into();
    contract.burn_from(owner, _to, _value);
}
