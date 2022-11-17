use casper_types::{
    account::AccountHash, bytesrepr::FromBytes, runtime_args, CLTyped, ContractPackageHash, Key,
    RuntimeArgs, U256,
};
use casperlabs_test_env::{TestContract, TestEnv};
use std::{collections::BTreeMap, time::SystemTime};

pub type TokenId = U256;
pub type Meta = BTreeMap<String, String>;

pub fn now() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64
}

pub struct CURVETOKENV3Instance(TestContract);
#[allow(clippy::too_many_arguments)]
impl CURVETOKENV3Instance {
    pub fn instance(curvetokenv3: TestContract) -> CURVETOKENV3Instance {
        CURVETOKENV3Instance(curvetokenv3)
    }

    pub fn new_deploy(
        env: &TestEnv,
        contract_name: &str,
        sender: AccountHash,
        name: String,
        symbol: String,
    ) -> TestContract {
        TestContract::new(
            env,
            "curve-token-v3.wasm",
            contract_name,
            sender,
            runtime_args! {
                "name" => name,
                "symbol" => symbol
            },
            now(),
        )
    }

    pub fn constructor(&self, sender: AccountHash, name: String, symbol: String) {
        self.0.call_contract(
            sender,
            "constructor",
            runtime_args! {

                "name" => name,
                "symbol" => symbol,
            },
            now(),
        );
    }
    pub fn decimals(&self, sender: AccountHash) {
        self.0
            .call_contract(sender, "decimals", runtime_args! {}, now());
    }
    pub fn transfer(&self, sender: AccountHash, recipient: Key, amount: U256) {
        self.0.call_contract(
            sender,
            "transfer",
            runtime_args! {
                "recipient" => recipient,
                "amount" => amount

            },
            now(),
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
            now(),
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
            now(),
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
            now(),
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
            now(),
        );
    }
    pub fn mint(&self, sender: AccountHash, to: Key, amount: U256) {
        self.0.call_contract(
            sender,
            "mint",
            runtime_args! {
                "to" => to,
                "amount"=>amount
            },
            now(),
        );
    }
    pub fn burn_from(&self, sender: AccountHash, from: Key, amount: U256) {
        self.0.call_contract(
            sender,
            "burn_from",
            runtime_args! {
                "from" => from,
                "amount"=>amount

            },
            now(),
        );
    }
    pub fn set_minter(&self, sender: AccountHash, minter: Key) {
        self.0.call_contract(
            sender,
            "set_minter",
            runtime_args! {
                "minter" => minter
            },
            now(),
        );
    }
    pub fn set_name(&self, sender: AccountHash, name: String, symbol: String) {
        self.0.call_contract(
            sender,
            "set_name",
            runtime_args! {
                "name" => name,
                "symbol"=>symbol

            },
            now(),
        );
    }

    // Result methods
    pub fn query<T: CLTyped + FromBytes>(&self, key: &str) -> T {
        self.0.query_named_key(key.into())
    }

    pub fn package_hash(&self) -> ContractPackageHash {
        self.0.package_hash().into()
    }
}
