use std::time::SystemTime;

use casper_types::{
    account::AccountHash, bytesrepr::FromBytes, runtime_args, CLTyped, Key, RuntimeArgs, U256,
};
use casperlabs_contract_utils::key_to_str;
use casperlabs_test_env::{TestContract, TestEnv};

pub struct FEEDISTRIBUTORInstance(TestContract);
#[allow(clippy::too_many_arguments)]
impl FEEDISTRIBUTORInstance {
    pub fn new_deploy(
        env: &TestEnv,
        contract_name: &str,
        sender: AccountHash,
        voting_escrow: Key,
        start_time: U256,
        token: Key,
        admin: Key,
        emergency_return: Key,
        time_now:u64
    ) -> FEEDISTRIBUTORInstance {
        FEEDISTRIBUTORInstance(TestContract::new(
            env,
            "fee-distributor.wasm",
            contract_name,
            sender,
            runtime_args! {
                "voting_escrow" => voting_escrow,
                "start_time" => start_time,
                "token" => token,
                "admin" => admin,
                "emergency_return" => emergency_return,
            },
            time_now,
        ))
    }

    pub fn checkpoint_token(&self, owner: AccountHash, time_now: u64) {
        self.0
            .call_contract(owner, "checkpoint_token", runtime_args! {}, time_now);
    }

    pub fn checkpoint_total_supply(&self, owner: AccountHash, time_now: u64) {
        self.0
            .call_contract(owner, "checkpoint_total_supply", runtime_args! {}, time_now);
    }

    pub fn commit_admin(&self, owner: AccountHash, time_now: u64, addr: Key) {
        self.0.call_contract(
            owner,
            "commit_admin",
            runtime_args! {
                "addr" => addr
            },
            time_now,
        );
    }

    pub fn apply_admin(&self, owner: AccountHash, time_now: u64) {
        self.0
            .call_contract(owner, "apply_admin", runtime_args! {}, time_now);
    }

    pub fn toggle_allow_checkpoint_token(&self, owner: AccountHash, time_now: u64) {
        self.0.call_contract(
            owner,
            "toggle_allow_checkpoint_token",
            runtime_args! {},
            time_now,
        );
    }

    pub fn kill_me(&self, owner: AccountHash, time_now: u64) {
        self.0
            .call_contract(owner, "kill_me", runtime_args! {}, time_now);
    }

    pub fn package_hash(&self) -> [u8; 32] {
        self.0.package_hash()
    }

    // Get stored key values
    pub fn key_value<T: CLTyped + FromBytes>(&self, key: String) -> T {
        self.0.query_named_key(key)
    }
    pub fn now() -> u64 {
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64
    }
    pub fn admin(&self) -> Key {
        self.0.query_named_key(String::from("admin"))
    }
    pub fn start_time(&self) -> U256 {
        self.0.query_named_key(String::from("start_time"))
    }
    pub fn last_token_time(&self) -> U256 {
        self.0.query_named_key(String::from("last_token_time"))
    }

    pub fn ve_supply<T: Into<Key>>(&self, week: T) -> U256 {
        self.0
            .query_dictionary("ve_supply", key_to_str(&week.into()))
            .unwrap_or_default()
    }
    pub fn future_admin(&self) -> Key {
        self.0.query_named_key(String::from("future_admin"))
    }
}
