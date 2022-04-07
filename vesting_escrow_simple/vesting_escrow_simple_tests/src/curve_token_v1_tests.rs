use casper_types::{
    account::AccountHash,runtime_args, ContractPackageHash, Key, RuntimeArgs, URef, U256, U512,
};
use test_env::{TestContract, TestEnv};

use crate::curve_token_v1_instance::CURVETOKENV1Instance;

fn deploy() -> (TestEnv, AccountHash, TestContract) {
    let env = TestEnv::new();
    let owner = env.next_user();
    let decimal: u8 = 2 as u8;
    let supply: U256 = 100.into();
   
    let contract = CURVETOKENV1Instance::new(
        &env,
        "CURVETOKENV1",
        owner,
        "CVTokenV1".to_string(),
        "CV1".to_string(),
        u8::from(2 as u8),
        100.into()
    );
  

    (env, owner, contract)
}

#[test]
fn test_deploy() {
    let (_, _, _) = deploy();
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
