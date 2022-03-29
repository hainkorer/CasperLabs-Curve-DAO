use casper_types::{
    bytesrepr::FromBytes, runtime_args, CLTyped, ContractPackageHash, Key, RuntimeArgs, URef, U256,
};
use test_env::{Sender, TestContract, TestEnv};

pub struct CURVETOKENV1Instance(TestContract);

impl CURVETOKENV1Instance {
    pub fn contract_instance(contract:TestContract)-> CURVETOKENV1Instance{
        CURVETOKENV1Instance(contract)
    }
    pub fn new(env: &TestEnv, contract_name: &str, sender: Sender,name:String,symbol:String,decimal:u8,supply:U256) -> TestContract {
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
    pub fn proxy(env: &TestEnv, contract_name: &str, sender: Sender,curve_token_v1:Key) -> CURVETOKENV1Instance {
        CURVETOKENV1Instance(TestContract::new(
            env,
            "contract.wasm",
            contract_name,
            sender,
            runtime_args! {"curve_token_v1"=>curve_token_v1},
        ))
    }
    pub fn set_minter(
        &self,
        sender: Sender,
        _minter: Key
    ) {
        self.0.call_contract(
            sender,
            "set_minter",
            runtime_args! {
                "_minter" => _minter
            },
        );
    }
    pub fn burn_from(
        &self,
        sender: Sender,
        _to: Key,
        _value:U256

    ) {
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
