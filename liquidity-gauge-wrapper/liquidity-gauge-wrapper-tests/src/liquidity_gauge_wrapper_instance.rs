use casper_types::{
    account::AccountHash, bytesrepr::FromBytes, runtime_args, CLTyped, Key, RuntimeArgs, U256,
};
use test_env::{TestContract, TestEnv};

pub struct LIQUIDITYGAUGEWRAPPERInstance(TestContract);

impl LIQUIDITYGAUGEWRAPPERInstance {
    pub fn contract_instance(contract: TestContract) -> LIQUIDITYGAUGEWRAPPERInstance {
        LIQUIDITYGAUGEWRAPPERInstance(contract)
    }
    pub fn new(
        env: &TestEnv,
        contract_name: &str,
        sender: AccountHash,
        name: String,
        symbol: String,
        gauge: Key,
        admin: Key
    ) -> TestContract {
        TestContract::new(
            env,
            "liquidity-gauge-wrapper.wasm",
            contract_name,
            sender,
            runtime_args! {
                "name" => name,
                "symbol" => symbol,
                "gauge" => gauge,
                "admin" => admin,
            },
            0,
        )
    }

    pub fn commit_transfer_ownership(&self, owner: AccountHash, addr: Key) {
        self.0.call_contract(
            owner,
            "commit_transfer_ownership",
            runtime_args! {
                "addr" => addr
            },
            0,
        );
    }

    pub fn apply_transfer_ownership(&self, owner: AccountHash) {
        self.0
            .call_contract(owner, "apply_transfer_ownership", runtime_args! {}, 0);
    }
    pub fn package_hash(&self) -> [u8; 32] {
        self.0.package_hash()
    }

    // Get stored key values
    pub fn key_value<T: CLTyped + FromBytes>(&self, key: String) -> T {
        self.0.query_named_key(key)
    }
}
