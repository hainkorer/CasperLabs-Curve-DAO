use std::time::SystemTime;
use casper_types::{
    account::AccountHash, bytesrepr::FromBytes, runtime_args, CLTyped, ContractPackageHash, Key,
    RuntimeArgs, U256,
};
use casperlabs_test_env::{TestContract, TestEnv};

pub struct CURVEREWARDSInstance(TestContract);
#[allow(clippy::too_many_arguments)]
impl CURVEREWARDSInstance {
    pub fn contract_instance(contract: TestContract) -> CURVEREWARDSInstance {
        CURVEREWARDSInstance(contract)
    }
    pub fn new_deploy(
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
            CURVEREWARDSInstance::now(),
        )
    }
    pub fn last_time_reward_applicable(&self, sender: AccountHash) {
        self.0
            .call_contract(sender, "last_time_reward_applicable", runtime_args! {}, CURVEREWARDSInstance::now());
    }
    pub fn reward_per_token(&self, sender: AccountHash) {
        self.0
            .call_contract(sender, "reward_per_token", runtime_args! {}, CURVEREWARDSInstance::now());
    }
    pub fn earned(&self, sender: AccountHash, account: Key) {
        self.0.call_contract(
            sender,
            "earned",
            runtime_args! {
                "account" => account
            },
            CURVEREWARDSInstance::now(),
        );
    }
    pub fn stake(&self, sender: AccountHash, amount: U256) {
        self.0.call_contract(
            sender,
            "stake",
            runtime_args! {
                "amount" => amount
            },
            CURVEREWARDSInstance::now(),
        );
    }
    pub fn withdraw(&self, sender: AccountHash, amount: U256) {
        self.0.call_contract(
            sender,
            "withdraw",
            runtime_args! {
                "amount" => amount
            },
            CURVEREWARDSInstance::now(),
        );
    }
    pub fn get_reward(&self, sender: AccountHash) {
        self.0
            .call_contract(sender, "get_reward", runtime_args! {}, CURVEREWARDSInstance::now());
    }
    pub fn exit(&self, sender: AccountHash) {
        self.0.call_contract(sender, "exit", runtime_args! {}, CURVEREWARDSInstance::now());
    }
    pub fn notify_reward_amount(&self, sender: AccountHash, reward: U256) {
        self.0.call_contract(
            sender,
            "notify_reward_amount",
            runtime_args! {
                "reward" => reward,
            },
            CURVEREWARDSInstance::now(),
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
