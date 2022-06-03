use crate::ownable_instance::OWNABLEInstance;
use casper_types::{account::AccountHash, Key};
use test_env::{TestContract, TestEnv};
fn deploy() -> (TestEnv, AccountHash, TestContract, TestContract) {
    let env = TestEnv::new();
    let owner = env.next_user();
    let instance = OWNABLEInstance::new(&env, "OWNABLE", owner);
    // Test Contract For Returning Value
    let ownable_package_hash = Key::Hash(instance.package_hash());
    let proxy = OWNABLEInstance::proxy(&env, "Proxy", owner, ownable_package_hash);

    (env, owner, instance, proxy)
}

#[test]
fn test_deploy() {
    let (_, _, _, _) = deploy();
}
#[test]
fn renounce_ownership() {
    let (_, owner, instance, _) = deploy();
    let instance = OWNABLEInstance::contract_instance(instance);
    instance.renounce_ownership(owner);
}
#[test]
fn transfer_ownership() {
    let (_, owner, instance, _) = deploy();
    let instance = OWNABLEInstance::contract_instance(instance);
    let new_owner: Key = Key::from_formatted_str(
        "hash-0000000020000000000000000000000000000000000000000000000000000000".into(),
    )
    .unwrap();
    instance.transfer_ownership(owner, new_owner);
}
