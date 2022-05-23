use casper_types::{
    account::AccountHash, runtime_args, ContractPackageHash, Key, RuntimeArgs, URef, U256, U512,
};
use test_env::{TestContract, TestEnv};

use crate::curve_token_v2_instance::CURVETOKENV2Instance;

fn deploy() -> (TestEnv, AccountHash, TestContract) {
    let env = TestEnv::new();
    let owner = env.next_user();
    let decimal: u8 = 2 as u8;
    let supply: U256 = 100.into();

    let contract = CURVETOKENV2Instance::new(
        &env,
        "CURVETOKENV2",
        owner,
        "CVTokenV2".to_string(),
        "CV2".to_string(),
        u8::from(2 as u8),
        100.into(),
    );

    (env, owner, contract)
}

#[test]
fn test_deploy() {
    let (_, _, _) = deploy();
}
#[test]
fn mint_crv2() {
    let (env, owner, contract) = deploy();
    let contract = CURVETOKENV2Instance::contract_instance(contract);
    let _to_arg: Key = Key::from_formatted_str(
        "hash-0000000000000000000000010000000000000000000000000000000000020000".into(),
    )
    .unwrap();
    let _value_arg: U256 = 100000.into();
    contract.mint_crv2(owner,_to_arg, _value_arg);
}
#[test]
fn set_minter() {
    let (env, owner, contract) = deploy();
    let contract = CURVETOKENV2Instance::contract_instance(contract);
    let _minter_arg: Key = Key::Account(owner);
    contract.set_minter(owner, _minter_arg);
}
#[test]
fn burn_from() {
    let (env, owner, contract) = deploy();
    let contract = CURVETOKENV2Instance::contract_instance(contract);
    let _to: Key = Key::Account(owner);
    let _value: U256 = 100.into();
    contract.burn_from(owner, _to, _value);
}
#[test]
fn set_name() {
    let (env, owner, contract) = deploy();
    let contract = CURVETOKENV2Instance::contract_instance(contract);
    let _name_arg: String = "CurveTokenV2".to_string();
    let _symbol_arg: String = "CV2".to_string();
    contract.set_name(owner, _name_arg, _symbol_arg);
}
