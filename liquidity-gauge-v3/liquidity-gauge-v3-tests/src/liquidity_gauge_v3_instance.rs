use std::collections::BTreeMap;

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

pub type TokenId = U256;
pub type Meta = BTreeMap<String, String>;

pub struct LIQUIDITYGUAGEV3INSTANCEInstance(TestContract);

impl LIQUIDITYGUAGEV3INSTANCEInstance {
    pub fn instance(liquidity_gauge_v3: TestContract) -> LIQUIDITYGUAGEV3INSTANCEInstance {
        LIQUIDITYGUAGEV3INSTANCEInstance(liquidity_gauge_v3)
    }

    pub fn new_deploy(
        env: &TestEnv,
        contract_name: &str,
        sender: AccountHash,
        lp_addr: Key,
        minter: Key,
        admin: Key,
    ) -> TestContract {
        TestContract::new(
            env,
            "liquidity-gauge-v3.wasm",
            contract_name,
            sender,
            runtime_args! {
                "lp_addr" => lp_addr,
                "minter"=>minter,
                "admin" => admin,
            },
            100000,
        )
    }
    pub fn commit_transfer_ownership(&self, sender: AccountHash, addr: Key) {
        self.0.call_contract(
            sender,
            "commit_transfer_ownership",
            runtime_args! {
                "addr" => addr,
            },
            0,
        );
    }
    pub fn accept_transfer_ownership(&self, sender: AccountHash) {
        self.0
            .call_contract(sender, "accept_transfer_ownership", runtime_args! {}, 0);
    }
    pub fn set_killed(&self, sender: AccountHash, is_killed: bool) {
        self.0.call_contract(
            sender,
            "set_killed",
            runtime_args! {
                "is_killed"=>is_killed,
            },
            0,
        );
    }
    pub fn approve(&self, sender: AccountHash, spender: Key, amount: U256) {
        self.0.call_contract(
            sender,
            "approve",
            runtime_args! {
                "spender" => spender,
                "amount" => amount

            },
            0,
        );
    }
    pub fn increase_allowance(&self, sender: AccountHash, spender: Key, amount: U256) {
        self.0.call_contract(
            sender,
            "increase_allowance",
            runtime_args! {
                "spender" => spender,
                "amount" => amount,

            },
            0,
        );
    }
    pub fn decrease_allowance(&self, sender: AccountHash, spender: Key, amount: U256) {
        self.0.call_contract(
            sender,
            "decrease_allowance",
            runtime_args! {
                "spender" => spender,
                "amount" => amount

            },
            0,
        );
    }
    pub fn decimals(&self, sender: AccountHash) {
        self.0
            .call_contract(sender, "decimals", runtime_args! {}, 0);
    }
    pub fn integrate_checkpoint(&self, sender: AccountHash) {
        self.0
            .call_contract(sender, "integrate_checkpoint", runtime_args! {}, 0);
    }
    pub fn deposit(
        &self,
        sender: AccountHash,
        value: U256,
        addr: Option<Key>,
        claim_rewards: Option<bool>,
    ) {
        self.0.call_contract(
            sender,
            "deposit",
            runtime_args! {
                "value" => value,
                "addr" => addr,
                "claim_rewards" => claim_rewards,
            },
            0,
        );
    }
    pub fn withdraw(&self, sender: AccountHash, value: U256, claim_rewards: Option<bool>) {
        self.0.call_contract(
            sender,
            "withdraw",
            runtime_args! {
                "value" => value,
                "claim_rewards" => claim_rewards,
            },
            0,
        );
    }
    pub fn transfer(&self, sender: AccountHash, recipient: Key, amount: U256) {
        self.0.call_contract(
            sender,
            "transfer",
            runtime_args! {
                "recipient" => recipient,
                "amount" => amount
            },
            0,
        );
    }
    pub fn transfer_from(&self, sender: AccountHash, owner: Key, recipient: Key, amount: U256) {
        self.0.call_contract(
            sender,
            "transfer_from",
            runtime_args! {
                "owner" => owner,
                "recipient" => recipient,
                "amount" => amount

            },
            0,
        );
    }
    pub fn set_rewards_receiver(&self, sender: AccountHash, receiver: Key) {
        self.0.call_contract(
            sender,
            "set_rewards_receiver",
            runtime_args! {
                "receiver" => receiver

            },
            0,
        );
    }
    pub fn set_rewards(&self, sender: AccountHash, receiver: Key) {
        self.0.call_contract(
            sender,
            "set_rewards_receiver",
            runtime_args! {
                "receiver" => receiver

            },
            0,
        );
    }
    pub fn claim_rewards(&self, sender: AccountHash, addr: Option<Key>, receiver: Option<Key>) {
        self.0.call_contract(
            sender,
            "claim_rewards",
            runtime_args! {
                "addr" => addr,
                "receiver" => receiver,
            },
            0,
        );
    }
    //var
    pub fn future_admin(&self) -> Key {
        self.0.query_named_key(String::from("future_admin"))
    }
    pub fn admin(&self) -> Key {
        self.0.query_named_key(String::from("admin"))
    }
    pub fn is_killed(&self) -> bool {
        self.0.query_named_key(String::from("is_killed"))
    }
    pub fn allowance<T: Into<Key>>(&self, owner: T, spender: T) -> U256 {
        let owner: Key = owner.into();
        let spender: Key = spender.into();
        self.0
            .query_dictionary(
                "allowances",
                LIQUIDITYGUAGEV3INSTANCEInstance::keys_to_str(&owner, &spender),
            )
            .unwrap_or_default()
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
    pub fn package_hash(&self) -> [u8; 32] {
        self.0.package_hash()
    }

    // Get stored key values
    pub fn key_value<T: CLTyped + FromBytes>(&self, key: String) -> T {
        self.0.query_named_key(key)
    }
}
