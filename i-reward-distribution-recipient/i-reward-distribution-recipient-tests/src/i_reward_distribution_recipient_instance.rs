use std::collections::BTreeMap;

use casper_types::{
    account::AccountHash, bytesrepr::FromBytes, runtime_args, CLTyped, ContractPackageHash, Key,
    RuntimeArgs, U256,
};
use test_env::{TestContract, TestEnv};

pub type TokenId = U256;
pub type Meta = BTreeMap<String, String>;

pub struct IREWARDDISTRIBUTIONRECIPIENTInstance(TestContract);

impl IREWARDDISTRIBUTIONRECIPIENTInstance {
    pub fn contract_instance(contract: TestContract) -> IREWARDDISTRIBUTIONRECIPIENTInstance {
        IREWARDDISTRIBUTIONRECIPIENTInstance(contract)
    }
    pub fn new(env: &TestEnv, contract_name: &str, sender: AccountHash) -> TestContract {
        TestContract::new(
            env,
            "i-reward-distribution-recipient.wasm",
            contract_name,
            sender,
            runtime_args! {},
            0,
        )
    }
    pub fn set_reward_distribution(&self, sender: AccountHash, reward_distribution: Key) {
        self.0.call_contract(
            sender,
            "set_reward_distribution",
            runtime_args! {
                "reward_distribution" => reward_distribution,
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
