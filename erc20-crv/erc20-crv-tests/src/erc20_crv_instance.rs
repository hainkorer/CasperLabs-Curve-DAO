use casper_types::{
    account::AccountHash, bytesrepr::FromBytes, runtime_args, CLTyped, ContractPackageHash, Key,
    RuntimeArgs, URef, U256,
};
use test_env::{TestContract, TestEnv};

pub struct ERC20CRVInstance(TestContract);

impl ERC20CRVInstance {
    pub fn new(
        env: &TestEnv,
        contract_name: &str,
        sender: AccountHash,
        name: String,
        symbol: String,
        decimal: u8,
        supply: U256,
    ) -> ERC20CRVInstance {
        ERC20CRVInstance(TestContract::new(
            env,
            "erc20_crv.wasm",
            contract_name,
            sender,
            runtime_args! {
                "name" => name,
                "symbol" => symbol,
                "decimal" => decimal,
                "supply" => supply,
            },
            100000000,
        ))
    }
    pub fn set_minter(&self, sender: AccountHash, _minter: Key) {
        self.0.call_contract(
            sender,
            "set_minter",
            runtime_args! {
                "_minter" => _minter
            },
            0,
        );
    }
    pub fn burn_caller(&self, sender: AccountHash, _value: U256) {
        self.0.call_contract(
            sender,
            "burn_caller",
            runtime_args! {
                "_value"=>_value
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
    pub fn start_epoch_time_write_js_client(&self, sender: AccountHash) {
        self.0.call_contract(
            sender,
            "start_epoch_time_write_js_client",
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
    pub fn future_epoch_time_write_js_client(&self, sender: AccountHash) {
        self.0.call_contract(
            sender,
            "future_epoch_time_write_js_client",
            runtime_args! {},
            1000000000,
        );
    }
    pub fn available_supply(&self, sender: AccountHash) {
        self.0
            .call_contract(sender, "available_supply", runtime_args! {}, 1000000000);
    }
    pub fn available_supply_js_client(&self, sender: AccountHash) {
        self.0.call_contract(
            sender,
            "available_supply_js_client",
            runtime_args! {},
            1000000000,
        );
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
    pub fn mintable_in_timeframe_js_client(&self, sender: AccountHash, start: U256, end: U256) {
        self.0.call_contract(
            sender,
            "mintable_in_timeframe_js_client",
            runtime_args! {
                "start"=>start,
                "end"=>end
            },
            0,
        );
    }
    pub fn mint_crv(&self, sender: AccountHash, to: Key, value: U256) {
        self.0.call_contract(
            sender,
            "mint_crv",
            runtime_args! {
                "to"=>to,
                "value"=>value
            },
            1000000000,
        );
    }
    pub fn mint_crv_js_client(&self, sender: AccountHash, to: Key, value: U256) {
        self.0.call_contract(
            sender,
            "mint_crv_js_client",
            runtime_args! {
                "to"=>to,
                "value"=>value
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
