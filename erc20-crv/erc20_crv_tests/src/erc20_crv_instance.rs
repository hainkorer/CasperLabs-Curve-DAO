
use casper_types::{
    account::AccountHash, bytesrepr::FromBytes, runtime_args, CLTyped, Key, RuntimeArgs, U256,
};
use curve_erc20_crate::Address;
use std::time::SystemTime;

use casperlabs_test_env::{TestContract, TestEnv};
pub struct ERC20CRVInstance(TestContract);
#[allow(clippy::too_many_arguments)]
impl ERC20CRVInstance {
    pub fn new_deploy(
        env: &TestEnv,
        contract_name: &str,
        sender: AccountHash,
        name: String,
        symbol: String,
        decimals: u8,
        time_now:u64
    ) -> ERC20CRVInstance {
        ERC20CRVInstance(TestContract::new(
            env,
            "erc20_crv.wasm",
            contract_name,
            sender,
            runtime_args! {
                "name" => name,
                "symbol" => symbol,
                "decimals" => decimals,
            },
            time_now,
        ))
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
    pub fn burn(&self, sender: AccountHash, value: U256) {
        self.0.call_contract(
            sender,
            "burn",
            runtime_args! {
                "value"=>value
            },
            0,
        );
    }
    pub fn set_admin(&self, sender: AccountHash, admin: Key) {
        self.0.call_contract(
            sender,
            "set_admin",
            runtime_args! {
                "admin"=>admin
            },
            0,
        );
    }
    pub fn update_mining_parameters(&self, sender: AccountHash,time_now:u64) {
        self.0.call_contract(
            sender,
            "update_mining_parameters",
            runtime_args! {},
            time_now,
        );
    }

    pub fn start_epoch_time_write(&self, sender: AccountHash,time_now:u64) {
        self.0.call_contract(
            sender,
            "start_epoch_time_write",
            runtime_args! {},
            time_now,
        );
    }
    pub fn approve(&self, sender: AccountHash, spender: Address, amount: U256) {
        self.0.call_contract(
            sender,
            "approve",
            runtime_args! {
                "spender"=>spender,
                "amount"=>amount

            },
            0,
        );
    }
    pub fn increase_allowance(&self, sender: AccountHash, spender: Address, amount: U256) {
        self.0.call_contract(
            sender,
            "increase_allowance",
            runtime_args! {
                "spender"=>spender,
                "amount"=>amount

            },
            0,
        );
    }
    pub fn decrease_allowance(&self, sender: AccountHash, spender: Address, amount: U256) {
        self.0.call_contract(
            sender,
            "decrease_allowance",
            runtime_args! {
                "spender"=>spender,
                "amount"=>amount

            },
            0,
        );
    }
    pub fn transfer(&self, sender: AccountHash, recipient: Address, amount: U256) {
        self.0.call_contract(
            sender,
            "transfer",
            runtime_args! {
                "recipient"=>recipient,
                "amount"=>amount

            },
            0,
        );
    }
    pub fn transfer_from(&self, sender: AccountHash,owner: Address, recipient: Address, amount: U256) {
        self.0.call_contract(
            sender,
            "transfer_from",
            runtime_args! {
                "owner"=>owner,
                "recipient"=>recipient,
                "amount"=>amount

            },
            0,
        );
    }
    pub fn mint(&self, sender: AccountHash, to: Address, amount: U256,time_now:u64) {
        self.0.call_contract(
            sender,
            "mint",
            runtime_args! {
                "to"=>to,
                "amount"=>amount

            },
            time_now,
        );
    }

    pub fn available_supply(&self, sender: AccountHash) {
        self.0
            .call_contract(sender, "available_supply", runtime_args! {}, 1000000000000);
    }
    pub fn get_init_supply(&self) -> U256 {
        self.0.query_named_key(String::from("init_supply"))
    }
    pub fn get_admin(&self) -> Key {
        self.0.query_named_key(String::from("admin"))
    }
    pub fn get_minter(&self) -> Key {
        self.0.query_named_key(String::from("minter"))
    }
    pub fn get_start_epoch_time(&self) -> U256 {
        self.0.query_named_key(String::from("start_epoch_time"))
    }
    pub fn get_rate(&self) -> U256 {
        self.0.query_named_key(String::from("rate"))
    }
    pub fn get_start_epoch_supply(&self) -> U256 {
        self.0.query_named_key(String::from("start_epoch_supply"))
    }
    // pub fn get_mining_epoch(&self) -> U128 {
    //     self.0.query_named_key(String::from("mining_epoch"))
    // }

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
}
