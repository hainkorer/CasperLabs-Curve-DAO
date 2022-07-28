use std::collections::BTreeMap;

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

pub struct VESTINGESCROWFACTORYInstance(TestContract);
#[allow(clippy::too_many_arguments)]
impl VESTINGESCROWFACTORYInstance {
    pub fn instance(vesting_escrow_factory: TestContract) -> VESTINGESCROWFACTORYInstance {
        VESTINGESCROWFACTORYInstance(vesting_escrow_factory)
    }

    pub fn proxy(env: &TestEnv, vesting_escrow_factory: Key, sender: AccountHash) -> TestContract {
        TestContract::new(
            env,
            "vesting-escrow-factory-proxy-token.wasm",
            "proxy_test",
            sender,
            runtime_args! {
                "vesting_escrow_factory" => vesting_escrow_factory
            },
            0,
        )
    }
    pub fn proxy2(env: &TestEnv, vesting_escrow_factory: Key, sender: AccountHash) -> TestContract {
        TestContract::new(
            env,
            "vesting-escrow-factory-proxy-token.wasm",
            "proxy_test2",
            sender,
            runtime_args! {
                "vesting_escrow_factory" => vesting_escrow_factory
            },
            0,
        )
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
            "erc20-token.wasm",
            "proxy_test2",
            sender,
            runtime_args! {
                "initial_supply" => supply,
                "name" => name,
                "symbol" => symbol,
                "decimals" => decimals
            },
            0,
        )
    }

    pub fn new_deploy(
        env: &TestEnv,
        contract_name: &str,
        sender: AccountHash,
        _target: Key,
        _admin: Key,
    ) -> TestContract {
        TestContract::new(
            env,
            "vesting-escrow-factory-token.wasm",
            contract_name,
            sender,
            runtime_args! {
                "target"=>_target,
                "admin"=>_admin,
            },
            0,
        )
    }

    pub fn constructor(
        &self,
        sender: AccountHash,
        _target: Key,
        _admin: Key,
    ) {
        self.0.call_contract(
            sender,
            "constructor",
            runtime_args! {
                "target"=>_target,
                "admin"=>_admin,
            },
            0,
        );
    }

    pub fn apply_transfer_ownership(&self, sender: AccountHash) {
        self.0
            .call_contract(sender, "apply_transfer_ownership", runtime_args! {}, 0);
    }

    pub fn commit_transfer_ownership<T: Into<Key>>(&self, sender: AccountHash, addr: T) {
        self.0.call_contract(
            sender,
            "commit_transfer_ownership",
            runtime_args! {
                "addr" => addr.into(),
            },
            0,
        );
    }

    pub fn deploy_vesting_contract<T: Into<Key>>(
        &self,
        sender: AccountHash,
        token: T,
        recipient: T,
        amount: U256,
        can_disable: bool,
        vesting_duration: U256,
        vesting_start: Option<U256>,
    ) {
        self.0.call_contract(
            sender,
            "deploy_vesting_contract",
            runtime_args! {
                "token" => token.into(),
                "recipient" => recipient.into(),
                "amount" => amount,
                "can_disable" => can_disable,
                "vesting_duration" => vesting_duration,
                "vesting_start" => vesting_start,
            },
            0,
        );
    }

    pub fn admin(&self) -> Key {
        self.0.query_named_key(String::from("admin"))
    }
    pub fn future_admin(&self) -> Key {
        self.0.query_named_key(String::from("future_admin"))
    }

    pub fn target(&self) -> Key {
        self.0.query_named_key(String::from("target"))
    }

    pub fn contract_package_hash(&self) -> ContractPackageHash {
        self.0
            .query_named_key(String::from("self_contract_package_hash"))
    }
    pub fn contract_hash(&self) -> Key {
        self.0.query_named_key(String::from("self_contract_hash"))
    }

    pub fn vesting_escrow_simple_contract_package_hash(&self) -> ContractPackageHash {
        self.0
            .query_named_key(String::from("vesting_escrow_simple_contract_package_hash"))
    }
    pub fn vesting_escrow_simple_contract_hash(&self) -> Key {
        self.0
            .query_named_key(String::from("vesting_escrow_simple_contract_hash"))
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
