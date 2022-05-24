use casper_types::{
    account::AccountHash, runtime_args, ContractPackageHash, Key, RuntimeArgs, URef, U256, U512,
};
use test_env::{TestContract, TestEnv};

use crate::curve_token_v3_instance::CURVETOKENV3Instance;

fn deploy() -> (TestEnv, AccountHash, TestContract) {
    let env = TestEnv::new();
    let owner = env.next_user();

    let contract = CURVETOKENV3Instance::new(
        &env,
        "CURVETOKENV3",
        owner,
        "CVTokenV3".to_string(),
        "CV3".to_string(),
    );

    (env, owner, contract)
}

#[test]
fn test_deploy() {
    let (_, _, _) = deploy();
}
#[test]
fn mint_crv3() {
    let (env, owner, contract) = deploy();
    let contract = CURVETOKENV3Instance::contract_instance(contract);
    let _to_arg: Key = Key::from_formatted_str(
        "hash-0000000000000000000000010000000000000000000000000000000000020000".into(),
    )
    .unwrap();
    let _value_arg: U256 = 100000.into();
    contract.mint_crv3(owner,_to_arg, _value_arg);
}

#[test]
fn set_minter() {
    let (env, owner, contract) = deploy();
    let contract = CURVETOKENV3Instance::contract_instance(contract);
    let _minter_arg: Key = Key::Account(owner);
    contract.set_minter(owner, _minter_arg);
}
#[test]
fn burn_from() {
    let (env, owner, contract) = deploy();
    let contract = CURVETOKENV3Instance::contract_instance(contract);
    let _to: Key = Key::Account(owner);
    let _value: U256 = 100.into();
    contract.burn_from(owner, _to, _value);
}
#[test]
fn set_name() {
    let (env, owner, contract) = deploy();
    let contract = CURVETOKENV3Instance::contract_instance(contract);
    let _name_arg: String = "CurveTokenV3".to_string();
    let _symbol_arg: String = "CV3".to_string();
    contract.set_name(owner, _name_arg, _symbol_arg);
}
