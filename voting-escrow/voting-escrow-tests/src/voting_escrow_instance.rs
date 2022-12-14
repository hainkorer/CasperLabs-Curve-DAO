use blake2::{
    digest::{Update, VariableOutput},
    VarBlake2b,
};
use casper_types::{
    account::AccountHash,
    bytesrepr::{FromBytes, ToBytes},
    runtime_args, CLTyped, Key, RuntimeArgs, U256,
};
use casperlabs_test_env::{TestContract, TestEnv};
use hex::encode;
use std::time::SystemTime;

pub const MILLI_SECONDS_IN_DAY: u64 = 86400000;

pub fn now() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64
}

pub struct VOTINGESCROWInstance(TestContract);
//#[clippy::must_use]
#[allow(clippy::too_many_arguments)]
impl VOTINGESCROWInstance {
    pub fn new_deploy(
        env: &TestEnv,
        contract_name: &str,
        sender: AccountHash,
        token_addr: Key,
        name: String,
        symbol: String,
        version: String,
        time: u64,
    ) -> VOTINGESCROWInstance {
        VOTINGESCROWInstance(TestContract::new(
            env,
            "voting-escrow.wasm",
            contract_name,
            sender,
            runtime_args! {
                "token_addr" => token_addr,
                "name" => name,
                "symbol" => symbol,
                "version" => version,
            },
            time,
        ))
    }

    pub fn commit_transfer_ownership(&self, owner: AccountHash, addr: Key, time: u64) {
        self.0.call_contract(
            owner,
            "commit_transfer_ownership",
            runtime_args! {
                "addr" => addr
            },
            time,
        );
    }

    pub fn apply_transfer_ownership(&self, owner: AccountHash, time: u64) {
        self.0
            .call_contract(owner, "apply_transfer_ownership", runtime_args! {}, time);
    }
    pub fn checkpoint(&self, owner: AccountHash, time: u64) {
        self.0
            .call_contract(owner, "checkpoint", runtime_args! {}, time);
    }

    pub fn deposit_for(&self, owner: AccountHash, addr: Key, value: U256, time: u64) {
        self.0.call_contract(
            owner,
            "deposit_for",
            runtime_args! {
                "addr" => addr,
                "value" => value
            },
            time,
        );
    }

    pub fn create_lock(&self, owner: AccountHash, value: U256, unlock_time: U256, time: u64) {
        self.0.call_contract(
            owner,
            "create_lock",
            runtime_args! {
                "value" => value,
                "unlock_time" =>  unlock_time
            },
            time,
        );
    }

    pub fn increase_amount(&self, owner: AccountHash, value: U256, time: u64) {
        self.0.call_contract(
            owner,
            "increase_amount",
            runtime_args! {
                "value" => value
            },
            time,
        );
    }

    pub fn increase_unlock_time(&self, owner: AccountHash, unlock_time: U256, time: u64) {
        self.0.call_contract(
            owner,
            "increase_unlock_time",
            runtime_args! {
                "unlock_time" => unlock_time
            },
            time,
        );
    }

    pub fn withdraw(&self, owner: AccountHash, time: u64) {
        self.0
            .call_contract(owner, "withdraw", runtime_args! {}, time);
    }
    pub fn total_supply(&self, owner: AccountHash, t: Option<U256>, time: u64) {
        self.0.call_contract(
            owner,
            "total_supply",
            runtime_args! {
                "t" => t
            },
            time,
        );
    }

    pub fn change_controller(&self, owner: AccountHash, new_controller: Key, time: u64) {
        self.0.call_contract(
            owner,
            "change_controller",
            runtime_args! {
                "new_controller" => new_controller
            },
            time,
        );
    }

    pub fn package_hash(&self) -> [u8; 32] {
        self.0.package_hash()
    }

    // Get stored key values
    pub fn key_value<T: CLTyped + FromBytes>(&self, key: String) -> T {
        self.0.query_named_key(key)
    }

    pub fn contract(&self) -> &TestContract {
        &self.0
    }
}

pub fn key_to_str(key: &Key) -> String {
    match key {
        Key::Account(account) => account.to_string(),
        Key::Hash(package) => encode(package),
        _ => panic!("Unexpected key type"),
    }
}

pub fn keys_to_str<T: CLTyped + ToBytes, U: CLTyped + ToBytes>(key_a: &T, key_b: &U) -> String {
    let mut hasher = VarBlake2b::new(32).unwrap();
    hasher.update(key_a.to_bytes().unwrap());
    hasher.update(key_b.to_bytes().unwrap());

    let mut ret = [0u8; 32];
    hasher.finalize_variable(|hash| ret.clone_from_slice(hash));

    encode(ret)
}
