use casper_types::{
    account::AccountHash, bytesrepr::FromBytes, runtime_args, CLTyped, ContractPackageHash, Key,
    RuntimeArgs,
};
use casperlabs_test_env::{TestContract, TestEnv};
use std::time::SystemTime;

pub struct OWNABLEInstance(TestContract);

#[allow(clippy::too_many_arguments)]

impl OWNABLEInstance {
    pub fn contract_instance(contract: TestContract) -> OWNABLEInstance {
        OWNABLEInstance(contract)
    }
    pub fn new_deploy(env: &TestEnv, contract_name: &str, sender: AccountHash) -> TestContract {
        TestContract::new(
            env,
            "ownable.wasm",
            contract_name,
            sender,
            runtime_args! {},
            OWNABLEInstance::now(),
        )
    }
    pub fn owner(&self, sender: AccountHash) {
        self.0
            .call_contract(sender, "owner", runtime_args! {}, OWNABLEInstance::now());
    }
    pub fn is_owner(&self, sender: AccountHash) {
        self.0
            .call_contract(sender, "is_owner", runtime_args! {}, OWNABLEInstance::now());
    }
    pub fn renounce_ownership(&self, sender: AccountHash) {
        self.0.call_contract(
            sender,
            "renounce_ownership",
            runtime_args! {},
            OWNABLEInstance::now(),
        );
    }
    pub fn transfer_ownership(&self, sender: AccountHash, new_owner: Key) {
        self.0.call_contract(
            sender,
            "transfer_ownership",
            runtime_args! {
                "new_owner" => new_owner
            },
            OWNABLEInstance::now(),
        );
    }
    // Result methods
    pub fn result<T: CLTyped + FromBytes>(&self) -> T {
        self.0.query_named_key("result".to_string())
    }

    pub fn package_hash(&self) -> ContractPackageHash {
        self.0.query_named_key("self_package_hash".to_string())
    }

    pub fn now() -> u64 {
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64
    }
}
