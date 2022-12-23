use std::{collections::BTreeMap, time::SystemTime};

use blake2::{
    digest::{Update, VariableOutput},
    VarBlake2b,
};
use casper_types::{
    account::AccountHash, bytesrepr::ToBytes, runtime_args, CLTyped, ContractPackageHash, Key,
    RuntimeArgs, U256,
};
use casperlabs_test_env::{TestContract, TestEnv};

pub type TokenId = U256;
pub type Meta = BTreeMap<String, String>;

pub struct VESTINGESCROWInstance(TestContract);
#[allow(clippy::too_many_arguments)]
impl VESTINGESCROWInstance {
    pub fn instance(vesting_escrow: TestContract) -> VESTINGESCROWInstance {
        VESTINGESCROWInstance(vesting_escrow)
    }

    pub fn erc20(
        env: &TestEnv,
        sender: AccountHash,
        name: &str,
        symbol: &str,
        decimals: u8,
        supply: U256,
    ) -> TestContract {
        TestContract::new(
            env,
            "curve-erc20.wasm",
            "proxy_test2",
            sender,
            runtime_args! {
                "initial_supply" => supply,
                "name" => name,
                "symbol" => symbol,
                "decimals" => decimals
            },
            VESTINGESCROWInstance::now(),
        )
    }

    pub fn new_deploy(
        env: &TestEnv,
        contract_name: &str,
        sender: AccountHash,
        _token: Key,
        _start_time: U256,
        _end_time: U256,
        _can_disable: bool,
        _fund_admins: Vec<String>,
    ) -> TestContract {
        TestContract::new(
            env,
            "vesting-escrow-token.wasm",
            contract_name,
            sender,
            runtime_args! {
                "token"=>_token,
                "start_time"=>_start_time,
                "end_time"=>_end_time,
                "can_disable"=> _can_disable,
                "fund_admins"=> _fund_admins,
            },
            VESTINGESCROWInstance::now(),
        )
    }

    pub fn commit_transfer_ownership<T: Into<Key>>(
        &self,
        sender: AccountHash,
        time_now: u64,
        addr: T,
    ) {
        self.0.call_contract(
            sender,
            "commit_transfer_ownership",
            runtime_args! {
                "addr" => addr.into(),
            },
            time_now,
        );
    }
    pub fn apply_transfer_ownership<T: Into<Key>>(&self, sender: AccountHash, time_now: u64) {
        self.0.call_contract(
            sender,
            "apply_transfer_ownership",
            runtime_args! {},
            time_now,
        );
    }

    pub fn disable_fund_admins(&self, sender: AccountHash, time_now: u64) {
        self.0
            .call_contract(sender, "disable_fund_admins", runtime_args! {}, time_now);
    }

    pub fn disable_can_disable(&self, sender: AccountHash, time_now: u64) {
        self.0
            .call_contract(sender, "disable_can_disable", runtime_args! {}, time_now);
    }
    pub fn toggle_disable<T: Into<Key>>(&self, sender: AccountHash, time_now: u64, _recipient: T) {
        self.0.call_contract(
            sender,
            "toggle_disable",
            runtime_args! {
                "recipient" => _recipient.into(),
            },
            time_now,
        );
    }
    pub fn add_tokens(&self, sender: AccountHash, time_now: u64, _amount: U256) {
        self.0.call_contract(
            sender,
            "add_tokens",
            runtime_args! {
                "amount" => _amount,
            },
            time_now,
        );
    }
    pub fn fund(
        &self,
        sender: AccountHash,
        time_now: u64,
        _recipients: Vec<String>,
        _amounts: Vec<U256>,
    ) {
        self.0.call_contract(
            sender,
            "fund",
            runtime_args! {
                "recipients" => _recipients,
                "amounts" => _amounts,
            },
            time_now,
        );
    }

    pub fn initial_locked<T: Into<Key>>(&self, account: T) -> U256 {
        self.0
            .query_dictionary("initial_locked", key_to_str(&account.into()))
            .unwrap_or_default()
    }
    pub fn total_claimed<T: Into<Key>>(&self, account: T) -> U256 {
        self.0
            .query_dictionary("total_claimed", key_to_str(&account.into()))
            .unwrap_or_default()
    }
    pub fn disabled_at<T: Into<Key>>(&self, account: T) -> U256 {
        self.0
            .query_dictionary("disabled_at", key_to_str(&account.into()))
            .unwrap_or_default()
    }
    pub fn fund_admins<T: Into<Key>>(&self, account: T) -> bool {
        self.0
            .query_dictionary("fund_admins", key_to_str(&account.into()))
            .unwrap_or_default()
    }

    pub fn lock(&self) -> u64 {
        self.0.query_named_key(String::from("lock"))
    }
    pub fn admin(&self) -> Key {
        self.0.query_named_key(String::from("admin"))
    }
    pub fn token(&self) -> Key {
        self.0.query_named_key(String::from("token"))
    }
    pub fn future_admin(&self) -> Key {
        self.0.query_named_key(String::from("future_admin"))
    }
    pub fn start_time(&self) -> U256 {
        self.0.query_named_key(String::from("start_time"))
    }
    pub fn end_time(&self) -> U256 {
        self.0.query_named_key(String::from("end_time"))
    }
    pub fn initial_locked_supply(&self) -> U256 {
        self.0
            .query_named_key(String::from("initial_locked_supply"))
    }
    pub fn unallocated_supply(&self) -> U256 {
        self.0.query_named_key(String::from("unallocated_supply"))
    }
    pub fn can_disable(&self) -> bool {
        self.0.query_named_key(String::from("can_disable"))
    }
    pub fn fund_admins_enabled(&self) -> bool {
        self.0.query_named_key(String::from("fund_admins_enabled"))
    }
    pub fn get_hash(&self) -> Key {
        self.0.query_named_key(String::from("self_contract_hash"))
    }
    pub fn package_hash(&self) -> ContractPackageHash {
        self.0
            .query_named_key(String::from("self_contract_package_hash"))
    }
    pub fn now() -> u64 {
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64
    }
}

pub fn key_to_str(key: &Key) -> String {
    match key {
        Key::Account(account) => account.to_string(),
        Key::Hash(package) => hex::encode(package),
        _ => panic!("Unexpected key type"),
    }
}

pub fn keys_to_str(key_a: &Key, key_b: &Key) -> String {
    let mut hasher = VarBlake2b::new(32).unwrap();
    hasher.update(key_a.to_bytes().unwrap());
    hasher.update(key_b.to_bytes().unwrap());
    let mut ret = [0u8; 32];
    hasher.finalize_variable(|hash| ret.clone_from_slice(hash));
    hex::encode(ret)
}

pub fn key_and_value_to_str<T: CLTyped + ToBytes>(key: &Key, value: &T) -> String {
    let mut hasher = VarBlake2b::new(32).unwrap();
    hasher.update(key.to_bytes().unwrap());
    hasher.update(value.to_bytes().unwrap());
    let mut ret = [0u8; 32];
    hasher.finalize_variable(|hash| ret.clone_from_slice(hash));
    hex::encode(ret)
}
