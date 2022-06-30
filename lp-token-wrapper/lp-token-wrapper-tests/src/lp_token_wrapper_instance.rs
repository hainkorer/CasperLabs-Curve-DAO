use std::collections::BTreeMap;

use casper_types::{
    account::AccountHash, bytesrepr::FromBytes, runtime_args, CLTyped, ContractPackageHash, Key,
    RuntimeArgs, U256,
};
use casperlabs_test_env::{TestContract, TestEnv};

pub type TokenId = U256;
pub type Meta = BTreeMap<String, String>;

pub struct LPTOKENWRAPPERInstance(TestContract);
//#[clippy::must_use]
#[allow(clippy::too_many_arguments)]
impl LPTOKENWRAPPERInstance {
    pub fn contract_instance(contract: TestContract) -> LPTOKENWRAPPERInstance {
        LPTOKENWRAPPERInstance(contract)
    }
    pub fn new_deploy(
        env: &TestEnv,
        contract_name: &str,
        sender: AccountHash,
        uni: Key,
    ) -> TestContract {
        TestContract::new(
            env,
            "lp-token-wrapper.wasm",
            contract_name,
            sender,
            runtime_args! {
                "uni" => uni,
            },
            0,
        )
    }
    pub fn total_supply(&self, sender: AccountHash) {
        self.0
            .call_contract(sender, "total_supply", runtime_args! {}, 0);
    }
    pub fn balance_of(&self, sender: AccountHash, account: Key) {
        self.0.call_contract(
            sender,
            "balance_of",
            runtime_args! {
                "account" => account
            },
            0,
        );
    }
    pub fn stake(&self, sender: AccountHash, amount: U256) {
        self.0.call_contract(
            sender,
            "stake",
            runtime_args! {
                "amount" => amount
            },
            0,
        );
    }
    pub fn withdraw(&self, sender: AccountHash, amount: U256) {
        self.0.call_contract(
            sender,
            "withdraw",
            runtime_args! {
                "amount" => amount
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
