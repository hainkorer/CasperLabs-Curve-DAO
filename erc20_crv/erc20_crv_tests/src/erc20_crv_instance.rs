use casper_types::{
    account::AccountHash, bytesrepr::FromBytes, runtime_args, CLTyped, ContractPackageHash, Key,
    RuntimeArgs, URef, U256,
};
use test_env::{TestContract, TestEnv};

pub struct ERC20CRVInstance(TestContract);

impl ERC20CRVInstance {
    pub fn contract_instance(contract: TestContract) -> ERC20CRVInstance {
        ERC20CRVInstance(contract)
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
            "erc20_crv.wasm",
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
    pub fn proxy(
        env: &TestEnv,
        contract_name: &str,
        sender: AccountHash,
        erc20_crv: Key,
    ) -> TestContract {
        TestContract::new(
            env,
            "erc20_crv_test.wasm",
            contract_name,
            sender,
            runtime_args! {
                "erc20_crv" => erc20_crv
            },
        )
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
    pub fn set_admin(&self, sender: AccountHash, admin: Key) {
        self.0.call_contract(
            sender,
            "set_admin",
            runtime_args! {
                "admin"=>admin
            },
        );
    }
    pub fn update_mining_parameters(&self, sender: AccountHash) {
        self.0
            .call_contract(sender, "update_mining_parameters", runtime_args! {});
    }

    pub fn start_epoch_time_write(&self, sender: AccountHash) {
        self.0.call_contract(
            sender,
            "start_epoch_time_write",
            runtime_args! {
            },
        );
    }
    pub fn future_epoch_time_write(&self, sender: AccountHash) {
        self.0.call_contract(
            sender,
            "future_epoch_time_write",
            runtime_args! {
            },
        );
    }
    pub fn available_supply(&self, sender: AccountHash) {
        self.0.call_contract(
            sender,
            "available_supply",
            runtime_args! {
            },
        );

    }
    pub fn mintable_in_timeframe(&self, sender: AccountHash,start:U256,end:U256) {
        self.0.call_contract(
            sender,
            "mintable_in_timeframe",
            runtime_args! {
                "start"=>start,
                "end"=>end
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
