use casper_types::{
    account::AccountHash, runtime_args, ContractPackageHash, Key, RuntimeArgs, URef, U256, U512,
};
use test_env::{TestContract, TestEnv};

use crate::vesting_escrow_simple_instance::VESTINGESCROWSIMPLEInstance;

fn deploy() -> (TestEnv, AccountHash, TestContract,TestContract) {
    let env = TestEnv::new();
    let owner = env.next_user();
    let contract = VESTINGESCROWSIMPLEInstance::new(&env, "VESTINGESCROWSIMPLE", owner);
    let proxy = VESTINGESCROWSIMPLEInstance::proxy(&env, "VESTINGESCROWSIMPLEPROXY", owner,Key::Hash(contract.contract_hash()));


    (env, owner, contract,proxy)
}

#[test]
fn test_deploy() {
    let (_, _, _,_) = deploy();
}
#[test]
fn toggle_disable() {
    let (env, owner, contract,proxy) = deploy();
    let contract = VESTINGESCROWSIMPLEInstance::contract_instance(contract);
    let _recipient_arg: Key = Key::Account(owner);
    contract.toggle_disable(owner, _recipient_arg);
}
#[test]
fn disable_can_disable() {
    let (env, owner, contract,proxy) = deploy();
    let contract = VESTINGESCROWSIMPLEInstance::contract_instance(contract);
   
    contract.disable_can_disable(owner );
}
#[test]
fn vested_of() {
    let (env, owner, contract,proxy) = deploy();
    let contract = VESTINGESCROWSIMPLEInstance::contract_instance(contract);
    let proxy =VESTINGESCROWSIMPLEInstance::contract_instance(proxy);
    let recipient_arg:Key= Key::Account(owner);
   let res= proxy.vested_of(owner,recipient_arg);
  
}
// #[test]
// fn burn_caller() {
//     let (env, owner, contract) = deploy();
//     let contract = CURVETOKENV1Instance::contract_instance(contract);
//     let _value: U256 = 100.into();
//     contract.burn_caller(owner, _value);
// }
// #[test]
// fn burn_from() {
//     let (env, owner, contract) = deploy();
//     let contract = CURVETOKENV1Instance::contract_instance(contract);
//     let _to: Key = Key::Account(owner);
//     let _value: U256 = 100.into();
//     contract.burn_from(owner, _to, _value);
// }
