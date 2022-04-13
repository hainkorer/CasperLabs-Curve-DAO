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
  proxy.vested_of(owner,recipient_arg);
  let res:U256= proxy.result();
  // println!("{:}",res);
   assert_eq!(res,0.into());
}
#[test]
fn vested_supply() {
    let (env, owner, contract,proxy) = deploy();
    let contract = VESTINGESCROWSIMPLEInstance::contract_instance(contract);
    let proxy =VESTINGESCROWSIMPLEInstance::contract_instance(proxy);
   proxy.vested_supply(owner);
  let res:U256= proxy.result();
  println!("{:}",res);
  //assert_eq!(res,0.into());
  
}
#[test]
fn locked_supply() {
    let (env, owner, contract,proxy) = deploy();
    let contract = VESTINGESCROWSIMPLEInstance::contract_instance(contract);
    let proxy =VESTINGESCROWSIMPLEInstance::contract_instance(proxy);
    proxy.locked_supply(owner);
    let res:U256= proxy.result();
    //println!("{:}",res);
    assert_eq!(res,100.into());
}
#[test]
fn balance_of_vest() {
    let (env, owner, contract,proxy) = deploy();
    let contract = VESTINGESCROWSIMPLEInstance::contract_instance(contract);
    let proxy =VESTINGESCROWSIMPLEInstance::contract_instance(proxy);
    let recipient_arg:Key= Key::Account(owner);
    proxy.balance_of_vest(owner,recipient_arg);
    let res:U256= proxy.result();
   // println!("{:}",res);
   assert_eq!(res,0.into());

  
}
#[test]
fn commit_transfer_ownership() {
    let (env, owner, contract,proxy) = deploy();
    let contract = VESTINGESCROWSIMPLEInstance::contract_instance(contract);
    let proxy =VESTINGESCROWSIMPLEInstance::contract_instance(proxy);
    // let addr:Key= Key::from_formatted_str(
    //     "hash-0000000000000000000000000000000000000000000000000000000000000000".into(),
    // )
    // .unwrap();
   let addr:Key= Key::Account(owner);
    proxy.commit_transfer_ownership(owner,addr);
    let res:bool= proxy.result();
   // println!("{:}",res);
    assert_eq!(res,true);
}
#[test]
fn apply_transfer_ownership() {
    let (env, owner, contract,proxy) = deploy();
    let contract = VESTINGESCROWSIMPLEInstance::contract_instance(contract);
    let proxy =VESTINGESCROWSIMPLEInstance::contract_instance(proxy);
    proxy.apply_transfer_ownership(owner);
    let res:bool= proxy.result();
    //println!("{:}",res);
    assert_eq!(res,true);
}
