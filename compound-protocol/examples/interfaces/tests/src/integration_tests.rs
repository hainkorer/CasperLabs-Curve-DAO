use casper_types::{
    account::AccountHash, runtime_args, ContractPackageHash, Key, RuntimeArgs, URef, U256, U512,
};
use test_env::{TestContract, TestEnv};

use crate::integration_tests_instance::IntegrationTestInstance;

pub fn deploy() -> (TestEnv, AcccountHash, TestContract) {
    let env = TestEnv::new();
    let owner = env.next_user();

    (
        env,
        owner,
        IntegrationTestInstance::new(&env, "integration_test", owner),
    )
}

#[test]
fn test_deploy() {
    (_, _, _) = deploy();
}
