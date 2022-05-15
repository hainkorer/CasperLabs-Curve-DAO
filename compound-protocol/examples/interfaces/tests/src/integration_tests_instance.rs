use casper_types::{
    account::AccountHash, bytesrepr::FromBytes, runtime_args, CLTyped, Key, RuntimeArgs, URef,
    U256, U512,
};
use test_env::{TestContract, TestEnv};

pub struct IntegrationTestInstance(TestContract);

impl IntegrationTestInstance {
    pub fn instance(contract: TestContract) -> IntegrationTestInstance {
        IntegrationTestInstance(contract)
    }

    pub fn new(env: &TestEnv, contract_name: &str, sender: AccountHash) -> TestContract {
        TestContract::new(
            env,
            "contract.wasm",
            contract_name,
            sender,
            runtime_args! {},
        )
    }

    // Result method
    pub fn query_key<T: CLTyped + FromBytes>(&self) -> T {
        self.0.query_named_key("result".to_string())
    }
}
