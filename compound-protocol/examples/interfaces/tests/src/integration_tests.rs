use casper_types::{
    account::AccountHash, runtime_args, ContractPackageHash, Key, RuntimeArgs, URef, U256, U512,
};
use test_env::{TestContract, TestEnv};

use crate::integration_tests_instance::IntegrationTestInstance;

pub fn deploy() -> (TestEnv, AccountHash, TestContract) {
    let env = TestEnv::new();
    let owner = env.next_user();
    let integration_tests = IntegrationTestInstance::new(&env, "integration_tests", owner);
    (env, owner, integration_tests)
}

#[test]
fn test_deploy() {
    let (_, _, integration_tests) = deploy();
    let package_hash: ContractPackageHash = integration_tests.query_named_key(String::from("package_hash"));
    let package_hash = Key::from(package_hash);
    let _package_hash = Key::Hash(integration_tests.package_hash());
    assert_eq!(_package_hash, package_hash);
}

#[test]
fn test_