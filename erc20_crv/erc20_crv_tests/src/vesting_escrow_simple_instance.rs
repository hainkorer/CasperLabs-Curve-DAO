use casper_types::{
    account::AccountHash, bytesrepr::FromBytes, runtime_args, CLTyped, ContractPackageHash, Key,
    RuntimeArgs, URef, U256,
};
use test_env::{TestContract, TestEnv};

pub struct VESTINGESCROWSIMPLEInstance(TestContract);

impl VESTINGESCROWSIMPLEInstance {
    pub fn contract_instance(contract: TestContract) -> VESTINGESCROWSIMPLEInstance {
        VESTINGESCROWSIMPLEInstance(contract)
    }
    pub fn new(env: &TestEnv, contract_name: &str, sender: AccountHash,token:Key) -> TestContract {
        TestContract::new(
            env,
            "vesting_escrow_simple.wasm",
            contract_name,
            sender,
            runtime_args! {
                "token"=>token
            },
        )
    }
    pub fn proxy(
        env: &TestEnv,
        contract_name: &str,
        sender: AccountHash,
        vesting_escrow_simple: Key,
    ) -> TestContract {
        TestContract::new(
            env,
            "contract.wasm",
            contract_name,
            sender,
            runtime_args! {
                "vesting_escrow_simple" => vesting_escrow_simple
            },
        )
    }
    pub fn toggle_disable(&self, sender: AccountHash, recipient: Key) {
        self.0.call_contract(
            sender,
            "toggle_disable",
            runtime_args! {
                "recipient" => recipient
            },
        );
    }
    pub fn disable_can_disable(&self, sender: AccountHash) {
        self.0
            .call_contract(sender, "disable_can_disable", runtime_args! {});
    }
    pub fn vested_supply(&self, sender: AccountHash) {
        self.0
            .call_contract(sender, "vested_supply", runtime_args! {
            });
    }
    pub fn vested_of(&self, sender: AccountHash,recipient: Key) {
        self.0
            .call_contract(sender, "vested_of", runtime_args! {
                "recipient" => recipient
            });
    }
    pub fn locked_supply(&self, sender: AccountHash) {
        self.0
            .call_contract(sender, "locked_supply", runtime_args! {
            });
    }
    pub fn balance_of_vest(&self, sender: AccountHash,recipient: Key) {
        self.0
            .call_contract(sender, "balance_of_vest", runtime_args! {
                "recipient" => recipient
            });
    }
    pub fn commit_transfer_ownership(&self, sender: AccountHash,addr: Key) {
        self.0
            .call_contract(sender, "commit_transfer_ownership", runtime_args! {
                "addr" => addr
            });
    }
    
    pub fn apply_transfer_ownership(&self, sender: AccountHash) {
        self.0
            .call_contract(sender, "apply_transfer_ownership", runtime_args! {
                
            });
    }
    pub fn claim(&self, sender: AccountHash,addr: Key) {
        self.0
            .call_contract(sender, "claim", runtime_args! {
                "addr" => addr
            });
    }

    // Result methods
    pub fn result<T: CLTyped + FromBytes>(&self) -> T {
        self.0.query_named_key("result".to_string())
    }

    pub fn package_hash(&self) -> ContractPackageHash {
        self.0.query_named_key("self_package_hash".to_string())
    }
}
