use casper_types::{
    account::AccountHash, runtime_args, ContractPackageHash, Key, RuntimeArgs, URef, U256, U512,
};
use test_env::{TestContract, TestEnv};
use crate::erc20_crv_instance::ERC20CRVInstance;

fn deploy() -> (TestEnv, AccountHash, TestContract) {
    let env = TestEnv::new();
    let owner = env.next_user();
    let instance = ERC20CRVInstance::new(
        &env,
        "ERC20CRV",
        owner,
        "ERC20CRV".to_string(),
        "erc20_crv".to_string(),
        u8::from(2 as u8),
        100.into(),
    );
    // let proxy =
    //     ERC20CRVInstance::proxy(&env, "ERC20CRV", owner, Key::Hash(instance.package_hash()));

    (env, owner, instance)
}

#[test]
fn test_deploy() {
    let (_, _, _) = deploy();
}
#[test]
fn set_minter() {
    let (env, owner, instance) = deploy();
    let instance = ERC20CRVInstance::contract_instance(instance);
    let _minter_arg: Key = Key::Account(owner);
    instance.set_minter(owner, _minter_arg);
}
#[test]
fn burn_caller() {
    let (env, owner, contract) = deploy();
    let contract = ERC20CRVInstance::contract_instance(contract);
    let _value: U256 = 1.into();
    contract.burn_caller(owner, _value);
}
#[test]
fn set_admin() {
    let (env, owner, contract) = deploy();
    let contract = ERC20CRVInstance::contract_instance(contract);
    let admin_arg: Key = Key::Account(owner);
    contract.set_admin(owner, admin_arg);
}
#[test]
fn update_mining_parameters() {
    let (env, owner, contract) = deploy();
    let contract = ERC20CRVInstance::contract_instance(contract);

    contract.update_mining_parameters(owner);
}
#[test]
fn start_epoch_time_write() {
    let (env, owner, contract) = deploy();
    let contract = ERC20CRVInstance::contract_instance(contract);
  

    contract.start_epoch_time_write_js_client(owner);
  let res:U256= contract.result();
  //println!("{:}",res);
  assert_eq!(res,0.into());

}
#[test]
fn future_epoch_time_write() {
    let (env, owner, contract) = deploy();
    let contract = ERC20CRVInstance::contract_instance(contract);
    //let proxy =ERC20CRVInstance::contract_instance(proxy);

    contract.future_epoch_time_write_js_client(owner);
  let res:U256= contract.result();
  assert_eq!(res,31536000.into());

}
#[test]
fn available_supply() {
    let (env, owner, contract) = deploy();
    let contract = ERC20CRVInstance::contract_instance(contract);
    contract.available_supply_js_client(owner);
    let res:U256= contract.result();
  //println!("{:}",res);
    assert_eq!(res,10000.into());

}
#[test]
fn mintable_in_timeframe() {
    let (env, owner, contract) = deploy();
    let contract = ERC20CRVInstance::contract_instance(contract);
    //let proxy =ERC20CRVInstance::contract_instance(proxy);
    let start_arg: U256=10.into();
    let end_arg: U256=100.into();
    contract.mintable_in_timeframe_js_client(owner,start_arg,end_arg);
  let res:U256= contract.result();
  //println!("{:}",res);
  assert_eq!(res,0.into());

}
#[test]
fn mint_crv() {
    let (env, owner, contract) = deploy();
    let contract = ERC20CRVInstance::contract_instance(contract);
   // let proxy =ERC20CRVInstance::contract_instance(proxy);
    let to: Key=Key::Account(owner);
    let value: U256=10.into();
    contract.mint_crv_js_client(owner,to,value);
  let res:bool= contract.result();
  //println!("{:}",res);
  assert_eq!(res,true);
}
