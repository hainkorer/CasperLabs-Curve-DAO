use casper_types::{
    account::AccountHash, bytesrepr::FromBytes, runtime_args, CLTyped, ContractPackageHash, Key,
    RuntimeArgs, URef, U256,
};
use test_env::{TestContract, TestEnv};

pub struct CURVETOKENV2Instance(TestContract);

impl CURVETOKENV2Instance {
    pub fn contract_instance(contract: TestContract) -> CURVETOKENV2Instance {
        CURVETOKENV2Instance(contract)
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
            "curve_token_v2.wasm",
            contract_name,
            sender,
            runtime_args! {
                "name" => name,
                "symbol" => symbol,
                "decimal" => decimal,
                "supply" => supply,
            },
            0,
        )
    }
    pub fn proxy(
        env: &TestEnv,
        contract_name: &str,
        sender: AccountHash,
        curve_token_v2: Key,
    ) -> CURVETOKENV2Instance {
        CURVETOKENV2Instance(TestContract::new(
            env,
            "contract.wasm",
            contract_name,
            sender,
            runtime_args! {"curve_token_v2"=>curve_token_v2},
            0,
        ))
    }
    pub fn mint_crv2(&self, sender: AccountHash, _to: Key, _value: U256) {
        self.0.call_contract(
            sender,
            "mint_crv2",
            runtime_args! {
                "_to" => _to,
                "_value"=>_value
            },
            0,
        );
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
    pub fn burn_from(&self, sender: AccountHash, _to: Key, _value: U256) {
        self.0.call_contract(
            sender,
            "burn_from",
            runtime_args! {
                "_to" => _to,
                "_value"=>_value

            },
            0,
        );
    }
    pub fn set_name(&self, sender: AccountHash, _name: String, _symbol: String) {
        self.0.call_contract(
            sender,
            "set_name",
            runtime_args! {
                "_name"=>_name,
                "_symbol"=>_symbol
            },
            0,
        );
    }

    // Result methods
    pub fn result<T: CLTyped + FromBytes>(&self) -> T {
        self.0.query_named_key("result".to_string())
    }

    pub fn package_hash(&self) -> ContractPackageHash {
        self.0
            .query_named_key("self_contract_package_hash".to_string())
    }
}
