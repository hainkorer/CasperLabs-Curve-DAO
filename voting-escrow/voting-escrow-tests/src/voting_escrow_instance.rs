use casper_types::{
    account::AccountHash, bytesrepr::FromBytes, runtime_args, CLTyped, Key, RuntimeArgs, URef, U256,
};
use test_env::{TestContract, TestEnv};

pub struct VOTINGESCROWInstance(TestContract);

impl VOTINGESCROWInstance {
    pub fn new(
        env: &TestEnv,
        contract_name: &str,
        sender: AccountHash,
        token_addr: Key,
        name: String,
        symbol: String,
        version: String,
    ) -> VOTINGESCROWInstance {
        VOTINGESCROWInstance(TestContract::new(
            env,
            "voting-escrow.wasm",
            contract_name,
            sender,
            runtime_args! {
                "token_addr" => token_addr,
                "name" => name,
                "symbol" => symbol,
                "version" => version,
            },
            0,
        ))
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

    pub fn commit_smart_wallet_checker(&self, owner: AccountHash, addr: Key) {
        self.0.call_contract(
            owner,
            "commit_smart_wallet_checker",
            runtime_args! {
                "addr" => addr
            },
            0,
        );
    }

    pub fn apply_smart_wallet_checker(&self, owner: AccountHash) {
        self.0
            .call_contract(owner, "apply_smart_wallet_checker", runtime_args! {}, 0);
    }

    pub fn get_last_user_slope_js_client(&self, owner: AccountHash, addr: Key) {
        self.0.call_contract(
            owner,
            "get_last_user_slope_js_client",
            runtime_args! {
                "addr" => addr
            },
            0,
        );
    }

    pub fn user_point_history_ts_js_client(&self, owner: AccountHash, addr: Key, idx: U256) {
        self.0.call_contract(
            owner,
            "user_point_history_ts_js_client",
            runtime_args! {
                "addr" => addr,
                "idx" => idx,
            },
            0,
        );
    }

    pub fn locked_end_js_client(&self, owner: AccountHash, addr: Key) {
        self.0.call_contract(
            owner,
            "locked_end_js_client",
            runtime_args! {
                "addr" => addr,
            },
            0,
        );
    }

    pub fn checkpoint(&self, owner: AccountHash) {
        self.0
            .call_contract(owner, "checkpoint", runtime_args! {}, 0);
    }

    pub fn deposit_for(&self, owner: AccountHash, addr: Key, value: U256) {
        self.0.call_contract(
            owner,
            "deposit_for",
            runtime_args! {
                "addr" => addr,
                "value" => value
            },
            0,
        );
    }

    pub fn create_lock(&self, owner: AccountHash, value: U256, unlock_time: U256) {
        self.0.call_contract(
            owner,
            "create_lock",
            runtime_args! {
                "value" => value,
                "unlock_time" =>  unlock_time
            },
            0,
        );
    }

    // Get stored key values
    pub fn key_value<T: CLTyped + FromBytes>(&self, key: String) -> T {
        self.0.query_named_key(key)
    }
}
