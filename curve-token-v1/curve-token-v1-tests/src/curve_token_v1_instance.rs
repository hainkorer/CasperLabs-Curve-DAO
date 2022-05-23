use casper_types::{
    account::AccountHash, bytesrepr::FromBytes, runtime_args, CLTyped, ContractPackageHash, Key,
    RuntimeArgs, URef, U256,
};
use test_env::{TestContract, TestEnv};

pub struct CURVETOKENV1Instance(TestContract);

impl CURVETOKENV1Instance {
    pub fn contract_instance(contract: TestContract) -> CURVETOKENV1Instance {
        CURVETOKENV1Instance(contract)
    }
    pub fn new(
        env: &TestEnv,
        contract_name: &str,
        sender: AccountHash,
        name: String,
        symbol: String,
        decimal: u8,
        supply: U256,
    ) -> TestContract {
        TestContract::new(
            env,
            "curve_token_v1.wasm",
            contract_name,
            sender,
            runtime_args! {
                "name" => name,
                "symbol" => symbol,
                "decimal" => decimal,
                "supply" => supply,
            },
        )
    }
    
    pub fn mint_crv1(&self, sender: AccountHash, _to: Key,_value:U256) {
        self.0.call_contract(
            sender,
            "mint_crv1",
            runtime_args! {
                "_to" => _to,
                "_value"=>_value
            },
        );
    }
    pub fn set_minter(&self, sender: AccountHash, _minter: Key) {
        self.0.call_contract(
            sender,
            "set_minter",
            runtime_args! {
                "_minter" => _minter
            },
        );
    }
    pub fn burn_caller(&self, sender: AccountHash, _value: U256) {
        self.0.call_contract(
            sender,
            "burn_caller",
            runtime_args! {
                "_value"=>_value
            },
        );
    }
    pub fn burn_from(&self, sender: AccountHash, _to: Key, _value: U256) {
        self.0.call_contract(
            sender,
            "burn_from",
            runtime_args! {
                "_to" => _to,
                "_value"=>_value

            },
        );
    }

    // Result methods
    pub fn result<T: CLTyped + FromBytes>(&self) -> T {
        self.0.query_named_key("result".to_string())
    }

    pub fn package_hash(&self) -> ContractPackageHash {
        self.0.query_named_key("self_package_hash".to_string())
    }
}
