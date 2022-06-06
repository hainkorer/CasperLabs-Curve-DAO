use std::collections::BTreeMap;

use casper_types::{
    account::AccountHash, bytesrepr::FromBytes, runtime_args, CLTyped, ContractPackageHash, Key,
    RuntimeArgs, U256,
};
use test_env::{TestContract, TestEnv};

pub type TokenId = U256;
pub type Meta = BTreeMap<String, String>;

pub struct CURVEREWARDSInstance(TestContract);

impl CURVEREWARDSInstance {
    pub fn contract_instance(contract: TestContract) -> CURVEREWARDSInstance {
        CURVEREWARDSInstance(contract)
    }
    pub fn new(
        env: &TestEnv,
        contract_name: &str,
        sender: AccountHash,
        token: Key,
        reward: Key,
    ) -> TestContract {
        TestContract::new(
            env,
            "curve-rewards.wasm",
            contract_name,
            sender,
            runtime_args! {
                "token" => token,
                "reward" => reward
            },
            0,
        )
    }
    pub fn last_time_reward_applicable(&self, sender: AccountHash) {
        self.0
            .call_contract(sender, "last_time_reward_applicable", runtime_args! {}, 200);
    }
    pub fn reward_per_token(&self, sender: AccountHash) {
        self.0
            .call_contract(sender, "reward_per_token", runtime_args! {}, 200);
    }
    pub fn earned(&self, sender: AccountHash, account: Key) {
        self.0.call_contract(
            sender,
            "earned",
            runtime_args! {
                "account" => account
            },
            200,
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
    pub fn get_reward(&self, sender: AccountHash) {
        self.0
            .call_contract(sender, "get_reward", runtime_args! {}, 0);
    }
    pub fn exit(&self, sender: AccountHash) {
        self.0.call_contract(sender, "exit", runtime_args! {}, 0);
    }
    pub fn notify_reward_amount(&self, sender: AccountHash, reward: U256) {
        self.0.call_contract(
            sender,
            "notify_reward_amount",
            runtime_args! {
                "reward" => reward,
            },
            50,
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
