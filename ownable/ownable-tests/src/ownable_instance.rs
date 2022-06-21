use std::collections::BTreeMap;

use casper_types::{
    account::AccountHash, bytesrepr::FromBytes, runtime_args, CLTyped, ContractPackageHash, Key,
    RuntimeArgs, U256,
};
use test_env::{TestContract, TestEnv};

pub type TokenId = U256;
pub type Meta = BTreeMap<String, String>;

pub struct OWNABLEInstance(TestContract);

impl OWNABLEInstance {
    pub fn contract_instance(contract: TestContract) -> OWNABLEInstance {
        OWNABLEInstance(contract)
    }
    pub fn new(env: &TestEnv, contract_name: &str, sender: AccountHash) -> TestContract {
        TestContract::new(
            env,
            "ownable.wasm",
            contract_name,
            sender,
            runtime_args! {},
            0,
        )
    }
    pub fn proxy(
        env: &TestEnv,
        contract_name: &str,
        sender: AccountHash,
        ownable: Key,
    ) -> TestContract {
        TestContract::new(
            env,
            "ownable_test.wasm",
            contract_name,
            sender,
            runtime_args! {
                "ownable" => ownable
            },
            0,
        )
    }
    pub fn owner(&self, sender: AccountHash) {
        self.0.call_contract(sender, "owner", runtime_args! {}, 0);
    }
    pub fn is_owner(&self, sender: AccountHash) {
        self.0
            .call_contract(sender, "is_owner", runtime_args! {}, 0);
    }
    pub fn renounce_ownership(&self, sender: AccountHash) {
        self.0
            .call_contract(sender, "renounce_ownership", runtime_args! {}, 0);
    }
    pub fn transfer_ownership(&self, sender: AccountHash, new_owner: Key) {
        self.0.call_contract(
            sender,
            "transfer_ownership",
            runtime_args! {
                "new_owner" => new_owner
            },
            0,
        );
    }
    // Result methods
    pub fn result<T: CLTyped + FromBytes>(&self) -> T {
        self.0.query_named_key("result".to_string())
    }

    pub fn package_hash(&self) -> ContractPackageHash {
        self.0.query_named_key("self_package_hash".to_string())
    }
}
