use casper_types::{
    account::AccountHash,
    bytesrepr::{Bytes, FromBytes},
    runtime_args, CLTyped, Key, RuntimeArgs, U256,
};
use casperlabs_test_env::{TestContract, TestEnv};

pub struct GAUGEPROXYInstance(TestContract);

impl GAUGEPROXYInstance {
    pub fn new(
        env: &TestEnv,
        contract_name: &str,
        sender: AccountHash,
        ownership_admin: Key,
        emergency_admin: Key,
    ) -> GAUGEPROXYInstance {
        GAUGEPROXYInstance(TestContract::new(
            env,
            "gauge-proxy.wasm",
            contract_name,
            sender,
            runtime_args! {
                "ownership_admin" => ownership_admin,
                "emergency_admin" => emergency_admin,
            },
            0,
        ))
    }

    pub fn commit_set_admins(&self, owner: AccountHash, o_admin: Key, e_admin: Key) {
        self.0.call_contract(
            owner,
            "commit_set_admins",
            runtime_args! {
                "o_admin" => o_admin,
                "e_admin" => e_admin
            },
            0,
        );
    }

    pub fn accept_set_admins(&self, owner: AccountHash) {
        self.0
            .call_contract(owner, "accept_set_admins", runtime_args! {}, 0);
    }

    pub fn commit_transfer_ownership(&self, owner: AccountHash, gauge: Key, new_owner: Key) {
        self.0.call_contract(
            owner,
            "commit_transfer_ownership",
            runtime_args! {
                "gauge" => gauge,
                "new_owner" => new_owner
            },
            0,
        );
    }

    pub fn accept_transfer_ownership(&self, owner: AccountHash, gauge: Key) {
        self.0.call_contract(
            owner,
            "accept_transfer_ownership",
            runtime_args! {
                "gauge" => gauge
            },
            0,
        );
    }

    pub fn set_killed(&self, owner: AccountHash, gauge: Key, is_killed: bool) {
        self.0.call_contract(
            owner,
            "set_killed",
            runtime_args! {
                "gauge" => gauge,
                "is_killed" => is_killed
            },
            0,
        );
    }

    pub fn set_rewards(
        &self,
        owner: AccountHash,
        gauge: Key,
        reward_contract: Key,
        sigs: Bytes,
        reward_tokens: Vec<Key>,
    ) {
        self.0.call_contract(
            owner,
            "set_rewards",
            runtime_args! {
                "gauge" => gauge,
                "reward_contract" => reward_contract,
                "sigs" => sigs,
                "reward_tokens" => reward_tokens
            },
            0,
        );
    }

    pub fn package_hash(&self) -> [u8; 32] {
        self.0.package_hash()
    }

    // Get stored key values
    pub fn key_value<T: CLTyped + FromBytes>(&self, key: String) -> T {
        self.0.query_named_key(key)
    }
}
