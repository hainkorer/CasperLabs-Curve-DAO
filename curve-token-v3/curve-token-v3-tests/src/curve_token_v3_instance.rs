use casper_types::{
    account::AccountHash, bytesrepr::FromBytes, runtime_args, CLTyped, ContractPackageHash, Key,
    RuntimeArgs, U256,
};
use casperlabs_test_env::{TestContract, TestEnv};
use std::collections::BTreeMap;

pub type TokenId = U256;
pub type Meta = BTreeMap<String, String>;

pub struct CURVETOKENV3Instance(TestContract);
//#[clippy::must_use]
#[allow(clippy::too_many_arguments)]
impl CURVETOKENV3Instance {
    pub fn instance(curvetokenv3: TestContract) -> CURVETOKENV3Instance {
        CURVETOKENV3Instance(curvetokenv3)
    }

    pub fn proxy(env: &TestEnv, curve_token_v3: Key, sender: AccountHash) -> TestContract {
        TestContract::new(
            env,
            "crv3-proxy-token.wasm",
            "proxy_test",
            sender,
            runtime_args! {
                "curve_token_v3" => curve_token_v3,
            },
            0,
        )
    }
    pub fn proxy2(env: &TestEnv, curve_token_v3: Key, sender: AccountHash) -> TestContract {
        TestContract::new(
            env,
            "crv3-proxy-token.wasm",
            "proxy2_test",
            sender,
            runtime_args! {
                "curve_token_v3" => curve_token_v3,
            },
            0,
        )
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
            0,
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
            0,
        );
    }
    pub fn decimals(&self, sender: AccountHash) {
        self.0
            .call_contract(sender, "decimals", runtime_args! {}, 0);
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
    pub fn mint(&self, sender: AccountHash, to: Key, amount: U256) {
        self.0.call_contract(
            sender,
            "mint",
            runtime_args! {
                "to" => to,
                "amount"=>amount
            },
            0,
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
            0,
        );
    }
    pub fn set_minter(&self, sender: AccountHash, minter: Key) {
        self.0.call_contract(
            sender,
            "set_minter",
            runtime_args! {
                "minter" => minter
            },
            0,
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
            0,
        );
    }

    // Result methods
    pub fn result<T: CLTyped + FromBytes>(&self) -> T {
        self.0.query_named_key("result".to_string())
    }

    pub fn package_hash(&self) -> ContractPackageHash {
        self.0.package_hash().into()
    }
}
