use casper_types::{
    account::AccountHash, bytesrepr::FromBytes, runtime_args, CLTyped, ContractPackageHash, Key,
    RuntimeArgs, URef, U256,
};
use contract_utils::key_to_str;
use test_env::{TestContract, TestEnv};

pub struct VESTINGESCROWSIMPLEInstance(TestContract);

impl VESTINGESCROWSIMPLEInstance {
    pub fn contract_instance(contract: TestContract) -> VESTINGESCROWSIMPLEInstance {
        VESTINGESCROWSIMPLEInstance(contract)
    }
    pub fn new(
        env: &TestEnv,
        contract_name: &str,
        sender: AccountHash,
        token: Key,
    ) -> TestContract {
        TestContract::new(
            env,
            "vesting-escrow-simple.wasm",
            contract_name,
            sender,
            runtime_args! {
                "token"=>token
            },
            0,
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
            "vesting-escrow-simple-proxy-token.wasm",
            contract_name,
            sender,
            runtime_args! {
                "vesting_escrow_simple" => vesting_escrow_simple
            },
            0,
        )
    }
    pub fn initialize(
        &self,
        sender: AccountHash,
        admin: Key,
        token: Key,
        recipient: Key,
        amount: U256,
        start_time: U256,
        end_time: U256,
        can_disable: bool,
    ) {
        self.0.call_contract(
            sender,
            "initialize",
            runtime_args! {
                "admin"=>admin,
                "token"=>token,
                "recipient"=>recipient,
                "amount"=>amount,
                "start_time"=>start_time,
                "end_time"=>end_time,
                "can_disable"=>can_disable
            },
            0,
        );
    }
    pub fn toggle_disable(&self, sender: AccountHash, recipient: Key) {
        self.0.call_contract(
            sender,
            "toggle_disable",
            runtime_args! {
                "recipient" => recipient
            },
            0,
        );
    }
    pub fn disable_can_disable(&self, sender: AccountHash) {
        self.0
            .call_contract(sender, "disable_can_disable", runtime_args! {}, 0);
    }
    pub fn vested_supply(&self, sender: AccountHash) {
        self.0
            .call_contract(sender, "vested_supply", runtime_args! {}, 0);
    }
    pub fn vested_of(&self, sender: AccountHash, recipient: Key) {
        self.0.call_contract(
            sender,
            "vested_of",
            runtime_args! {
                "recipient" => recipient
            },
            0,
        );
    }
    //queries
    pub fn locked_supply(&self, sender: AccountHash) {
        self.0
            .call_contract(sender, "locked_supply", runtime_args! {}, 0);
    }
    pub fn balance_of_vest(&self, sender: AccountHash, recipient: Key) {
        self.0.call_contract(
            sender,
            "balance_of_vest",
            runtime_args! {
                "recipient" => recipient
            },
            0,
        );
    }
    pub fn commit_transfer_ownership(&self, sender: AccountHash, addr: Key) {
        self.0.call_contract(
            sender,
            "commit_transfer_ownership",
            runtime_args! {
                "addr" => addr
            },
            0,
        );
    }

    pub fn apply_transfer_ownership(&self, sender: AccountHash) {
        self.0
            .call_contract(sender, "apply_transfer_ownership", runtime_args! {}, 0);
    }
    pub fn claim(&self, sender: AccountHash, addr: Key) {
        self.0.call_contract(
            sender,
            "claim",
            runtime_args! {
                "addr" => addr
            },
            0,
        );
    }
    pub fn initial_locked<T: Into<Key>>(&self, account: T) -> U256 {
        self.0
            .query_dictionary("initial_locked", key_to_str(&account.into()))
            .unwrap_or_default()
    }

    pub fn total_claimed<T: Into<Key>>(&self, account: T) -> U256 {
        self.0
            .query_dictionary("total_claimed", key_to_str(&account.into()))
            .unwrap_or_default()
    }
    pub fn disabled_at<T: Into<Key>>(&self, account: T) -> U256 {
        self.0
            .query_dictionary("disabled_at", key_to_str(&account.into()))
            .unwrap_or_default()
    }
    pub fn start_time(&self) -> U256 {
        self.0.query_named_key(String::from("start_time"))
    }
    pub fn end_time(&self) -> U256 {
        self.0.query_named_key(String::from("end_time"))
    }

    pub fn initial_locked_supply(&self) -> U256 {
        self.0
            .query_named_key(String::from("initial_locked_supply"))
    }

    pub fn can_disable(&self) -> bool {
        self.0.query_named_key(String::from("can_disable"))
    }

    // Result methods
    pub fn result<T: CLTyped + FromBytes>(&self) -> T {
        self.0.query_named_key("result".to_string())
    }

    pub fn package_hash(&self) -> ContractPackageHash {
        self.0.package_hash().into()
    }
}
