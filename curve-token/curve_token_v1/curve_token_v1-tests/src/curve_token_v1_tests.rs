use blake2::digest::consts::U2;
use casper_engine_test_support::AccountHash;
use casper_types::{runtime_args, ContractPackageHash, Key, RuntimeArgs, URef, U256, U512};
use renvm_sig::keccak256;
use test_env::{Sender, TestContract, TestEnv};

use crate::curve_token_v1_instance::CURVETOKENV1Instance;

fn deploy() -> (TestEnv, AccountHash, TestContract) {
    let env = TestEnv::new();
    let owner = env.next_user();
    let decimal:u8=2 as u8;
    let supply:U256=U256::from(100);
    // let contract = CURVETOKENV1Instance::new(&env, "CURVETOKENV1", Sender(owner));
    let contract = CURVETOKENV1Instance::new(&env,"CURVETOKENV1",Sender(owner),"CVTokenV1".to_string(),"CV1".to_string(),u8::from(2 as u8) ,U256::from(100));
    //let proxy = CURVETOKENV1Instance::proxy(&env, "CURVETOKENV1PROXY", Sender(owner),Key::Hash(contract.contract_hash()));
  
   
    (env, owner,contract)

}

#[test]
fn test_deploy() {
    let (_, _, _) = deploy();
}
#[test]
fn set_minter(){
    let (env, owner,contract) = deploy();
    let contract =CURVETOKENV1Instance::contract_instance(contract);
    let _minter_arg:Key = Key::Account(owner);
    contract.set_minter(Sender(owner),_minter_arg);
}
#[test]
fn burn_from(){
    let (env, owner,contract) = deploy();
    let contract =CURVETOKENV1Instance::contract_instance(contract);
    let _to:Key = Key::Account(owner);
    let _value:U256 = U256::from((100));

    contract.burn_from(Sender(owner),_to,_value);
}


