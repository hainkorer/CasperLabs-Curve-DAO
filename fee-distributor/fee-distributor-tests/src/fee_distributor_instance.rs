use casper_types::{
    account::AccountHash, bytesrepr::FromBytes, runtime_args, CLTyped, Key, RuntimeArgs, U256,
};
use casperlabs_test_env::{TestContract, TestEnv};

pub struct FEEDISTRIBUTORInstance(TestContract);
//#[clippy::must_use]
#[allow(clippy::too_many_arguments)]
impl FEEDISTRIBUTORInstance {
    pub fn new_deploy(
        env: &TestEnv,
        contract_name: &str,
        sender: AccountHash,
        voting_escrow: Key,
        start_time: U256,
        token: Key,
        admin: Key,
        emergency_return: Key,
    ) -> FEEDISTRIBUTORInstance {
        FEEDISTRIBUTORInstance(TestContract::new(
            env,
            "fee-distributor.wasm",
            contract_name,
            sender,
            runtime_args! {
                "voting_escrow" => voting_escrow,
                "start_time" => start_time,
                "token" => token,
                "admin" => admin,
                "emergency_return" => emergency_return,
            },
            0,
        ))
    }

    pub fn checkpoint_token(&self, owner: AccountHash) {
        self.0
            .call_contract(owner, "checkpoint_token", runtime_args! {}, 0);
    }

    pub fn ve_for_at_js_client(&self, owner: AccountHash, user: Key, timestamp: U256) {
        self.0.call_contract(
            owner,
            "ve_for_at_js_client",
            runtime_args! {
                "user" => user,
                "timestamp" => timestamp
            },
            0,
        );
    }

    pub fn checkpoint_total_supply(&self, owner: AccountHash) {
        self.0
            .call_contract(owner, "checkpoint_total_supply", runtime_args! {}, 0);
    }

    pub fn claim_js_client(&self, owner: AccountHash, addr: Key) {
        self.0.call_contract(
            owner,
            "claim_js_client",
            runtime_args! {
                "addr" => addr
            },
            0,
        );
    }

    pub fn claim_many_js_client(&self, owner: AccountHash, receivers: Vec<Key>) {
        self.0.call_contract(
            owner,
            "claim_many_js_client",
            runtime_args! {
                "receivers" => receivers
            },
            0,
        );
    }

    pub fn burn_js_client(&self, owner: AccountHash, coin: Key) {
        self.0.call_contract(
            owner,
            "burn_js_client",
            runtime_args! {
                "coin" => coin
            },
            0,
        );
    }

    pub fn commit_admin(&self, owner: AccountHash, addr: Key) {
        self.0.call_contract(
            owner,
            "commit_admin",
            runtime_args! {
                "addr" => addr
            },
            0,
        );
    }

    pub fn apply_admin(&self, owner: AccountHash) {
        self.0
            .call_contract(owner, "apply_admin", runtime_args! {}, 0);
    }

    pub fn toggle_allow_checkpoint_token(&self, owner: AccountHash) {
        self.0
            .call_contract(owner, "toggle_allow_checkpoint_token", runtime_args! {}, 0);
    }

    pub fn kill_me(&self, owner: AccountHash) {
        self.0.call_contract(owner, "kill_me", runtime_args! {}, 0);
    }

    pub fn recover_balance_js_client(&self, owner: AccountHash, coin: Key) {
        self.0.call_contract(
            owner,
            "recover_balance_js_client",
            runtime_args! {
                "coin" => coin
            },
            0,
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
