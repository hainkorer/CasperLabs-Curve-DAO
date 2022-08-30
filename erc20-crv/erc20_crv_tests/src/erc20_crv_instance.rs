use casper_types::{
    account::AccountHash, bytesrepr::FromBytes, runtime_args, CLTyped, Key, RuntimeArgs, U256,
};
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
            100000000,
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
    pub fn update_mining_parameters(&self, sender: AccountHash) {
        self.0.call_contract(
            sender,
            "update_mining_parameters",
            runtime_args! {},
            1000000000,
        );
    }

    pub fn start_epoch_time_write(&self, sender: AccountHash) {
        self.0.call_contract(
            sender,
            "start_epoch_time_write",
            runtime_args! {},
            1000000000,
        );
    }

    pub fn future_epoch_time_write(&self, sender: AccountHash) {
        self.0.call_contract(
            sender,
            "future_epoch_time_write",
            runtime_args! {},
            1000000000,
        );
    }

    pub fn available_supply(&self, sender: AccountHash) {
        self.0
            .call_contract(sender, "available_supply", runtime_args! {}, 1000000000);
    }
    pub fn mintable_in_timeframe(&self, sender: AccountHash, start: U256, end: U256) {
        self.0.call_contract(
            sender,
            "mintable_in_timeframe",
            runtime_args! {
                "start"=>start,
                "end"=>end
            },
            0,
        );
    }
    pub fn mint(&self, sender: AccountHash, to: Key, amount: U256) {
        self.0.call_contract(
            sender,
            "mint",
            runtime_args! {
                "to"=>to,
                "amount"=>amount
            },
            1000000000,
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
