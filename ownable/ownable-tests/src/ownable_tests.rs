use crate::ownable_instance::OWNABLEInstance;
use casper_types::{account::AccountHash, runtime_args, Key, RuntimeArgs};
use casperlabs_test_env::{TestContract, TestEnv};
use common::keys::*;
fn deploy() -> (TestEnv, AccountHash, TestContract) {
    let env = TestEnv::new();
    let owner = env.next_user();
    let instance = OWNABLEInstance::new_deploy(&env, "OWNABLE", owner);
    (env, owner, instance)
}

#[test]
fn test_deploy() {
    let (_, _, _) = deploy();
}
#[test]
fn renounce_ownership() {
    let (_, owner, instance) = deploy();
    let instance = OWNABLEInstance::contract_instance(instance);
    instance.renounce_ownership(owner);
}
#[test]
fn is_owner() {
    let (env, owner, instance) = deploy();
    let package_hash = Key::Hash(instance.package_hash());
    TestContract::new(
        &env,
        "ownable-session-code.wasm",
        SESSION_CODE_NAME,
        owner,
        runtime_args! {
            "entrypoint" => String::from(IS_OWNER),
            "package_hash" => package_hash,
        },
        300,
    );
    let ret: bool = env.query_account_named_key(owner, &[IS_OWNER.into()]);
    let res: bool = true;
    assert_eq!(ret, res);
}
#[test]
fn transfer_ownership() {
    let (env, owner, instance) = deploy();
    let instance = OWNABLEInstance::contract_instance(instance);
    let new_owner: Key = env.next_user().into();
    instance.transfer_ownership(owner, new_owner);
}
